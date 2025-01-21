use std::{
    cell::{Cell, RefCell},
    rc::Rc,
};

use glow::HasContext;
use smallvec::SmallVec;

use crate::{
    gl::{BufferError, BufferUsage, ProgramError},
    sl::program_def::ProgramDef,
};

use super::{
    error::{check_framebuffer_completeness, check_gl_error},
    params::ClearParams,
    Attachment, Buffer, Caps, ContextError, DrawParams, Framebuffer, FramebufferError, Image,
    Program, Texture2d, TextureError,
};

pub(super) struct ContextShared {
    gl: glow::Context,
    caps: Caps,
    draw_params: Cell<DrawParams>,
    draw_vao: glow::VertexArray,
    draw_fbo: glow::Framebuffer,
    default_framebuffer_size: Cell<[u32; 2]>,
    // TODO: Should probably combine all bound state into a separate struct and
    // put that whole thing into a `RefCell`.
    bound_program_id: Cell<Option<glow::Program>>,
    is_draw_fbo_bound: Cell<bool>,
    bound_color_attachment_ids: RefCell<Vec<glow::Texture>>,
    bound_depth_attachment_id: Cell<Option<(u32, glow::Texture)>>,
    bound_uniform_buffer_ids: RefCell<Vec<Option<glow::Buffer>>>,
    bound_texture_2d_ids: RefCell<Vec<Option<glow::Texture>>>,
}

pub struct Context {
    shared: Rc<ContextShared>,
}

impl ContextShared {
    pub fn ref_eq(&self, other: &ContextShared) -> bool {
        std::ptr::eq(self as *const ContextShared, other as *const ContextShared)
    }

    pub fn gl(&self) -> &glow::Context {
        &self.gl
    }

    pub fn caps(&self) -> &Caps {
        &self.caps
    }

    pub(super) fn set_draw_params(&self, new: &DrawParams, framebuffer_size: [u32; 2]) {
        let gl = &self.gl;

        let current = self.draw_params.get();
        new.set_delta(gl, &current, framebuffer_size);
        self.draw_params.set(*new);
    }

    pub(super) fn default_framebuffer_size(&self) -> [u32; 2] {
        self.default_framebuffer_size.get()
    }

    pub(super) fn bind_program(&self, id: Option<glow::Program>) {
        if id == self.bound_program_id.get() {
            return;
        }

        unsafe { self.gl.use_program(id) };

        self.bound_program_id.set(id);
    }

    pub(super) fn unbind_program_if_bound(&self, id: glow::Program) {
        if Some(id) == self.bound_program_id.get() {
            self.bind_program(None);
        }
    }

    fn bind_color_attachments(&self, attachments: &[Attachment]) -> Result<(), FramebufferError> {
        // OpenGL ES 3.0.6: 4.4.2.4 Attaching Texture Images to a Framebuffer
        // > An `INVALID_OPERATION` is generated if `attachment` is
        // > `COLOR_ATTACHMENTm` where `m` is greater than or equal to the value
        // > of `MAX_COLOR_ATTACHMENTS`.
        let num_attachments = attachments
            .len()
            .try_into()
            .expect("number of attachments is out of u32 range");

        if num_attachments > self.caps.max_color_attachments {
            return Err(FramebufferError::TooManyColorAttachments {
                requested: num_attachments,
                max: self.caps.max_color_attachments,
            });
        }

        if num_attachments > self.caps.max_draw_buffers {
            return Err(FramebufferError::TooManyDrawBuffers {
                requested: num_attachments,
                max: self.caps.max_draw_buffers,
            });
        }

        let mut bound_ids = self.bound_color_attachment_ids.borrow_mut();

        for (idx, attachment) in attachments.iter().enumerate() {
            if idx < bound_ids.len() && bound_ids[idx] == attachment.id() {
                continue;
            }

            match attachment {
                Attachment::Texture2d { texture, level } => {
                    // OpenGL ES 3.0.6: 4.4.2.4 Attaching Texture Images to a
                    // Framebuffer
                    // > If `textarget` is `TEXTURE_2D`, `level` must be greater
                    // > than or equal to zero and no larger than `log_2` of the
                    // > value of `MAX_TEXTURE_SIZE`.

                    assert!(texture.internal_format().is_color_renderable());

                    let max_level = (self.caps.max_texture_size as f64).log2() as u32;

                    if *level > max_level {
                        return Err(FramebufferError::LevelTooLarge {
                            requested: *level,
                            max: max_level,
                        });
                    }

                    let location = glow::COLOR_ATTACHMENT0 + idx as u32;
                    let level: i32 = (*level).try_into().expect("level is out of i32 range");

                    unsafe {
                        self.gl.framebuffer_texture_2d(
                            glow::FRAMEBUFFER,
                            location,
                            glow::TEXTURE_2D,
                            Some(texture.id()),
                            level,
                        )
                    };
                }
            };
        }

        if bound_ids.len() > attachments.len() {
            for idx in attachments.len()..bound_ids.len() {
                let location = glow::COLOR_ATTACHMENT0 + idx as u32;

                unsafe {
                    self.gl.framebuffer_texture_2d(
                        glow::FRAMEBUFFER,
                        location,
                        glow::TEXTURE_2D,
                        None,
                        0,
                    )
                };
            }
        }

        if bound_ids.len() != attachments.len() {
            let draw_buffers: SmallVec<[_; 8]> = (0..attachments.len())
                .map(|idx| glow::COLOR_ATTACHMENT0 + idx as u32)
                .collect();

            unsafe { self.gl.draw_buffers(&draw_buffers) };
        }

        bound_ids.clear();
        bound_ids.extend(attachments.iter().map(|attachment| attachment.id()));

        // FIXME: Do we need to check for the presence of at least one
        // attachment?

        Ok(())
    }

    fn bind_depth_attachment(&self, attachment: Option<&Attachment>) {
        let attachment_id = attachment.map(|attachment| attachment.id());
        let bound_id = self.bound_depth_attachment_id.get().map(|(_, id)| id);

        if attachment_id == bound_id {
            return;
        }

        let attachment = attachment.map(|attachment| {
            let format = attachment.internal_format();

            match (format.is_depth_renderable(), format.is_stencil_renderable()) {
                (true, true) => (glow::DEPTH_STENCIL_ATTACHMENT, attachment),
                (true, false) => (glow::DEPTH_ATTACHMENT, attachment),
                _ => panic!("expected texture to have a depth attachment"),
            }
        });

        let location = attachment.map(|(location, _)| location);
        let bound_location = self
            .bound_depth_attachment_id
            .get()
            .map(|(bound_location, _)| bound_location);

        if let Some(bound_location) =
            bound_location.filter(|bound_location| Some(*bound_location) != location)
        {
            // TODO: In the future, the existing binding could be a non-2D
            // texture, but we are currently using the 2D unbinding function in
            // all cases. Is this correct?
            unsafe {
                self.gl.framebuffer_texture_2d(
                    glow::FRAMEBUFFER,
                    bound_location,
                    glow::TEXTURE_2D,
                    None,
                    0,
                )
            };
        }

        if let Some((location, attachment)) = attachment {
            match attachment {
                Attachment::Texture2d { level, texture } => {
                    let level = (*level).try_into().expect("level is out of i32 range");

                    unsafe {
                        self.gl.framebuffer_texture_2d(
                            glow::FRAMEBUFFER,
                            location,
                            glow::TEXTURE_2D,
                            Some(texture.id()),
                            level,
                        )
                    };
                }
            }
        }

        self.bound_depth_attachment_id
            .set(attachment.map(|(location, attachment)| (location, attachment.id())));
    }

    pub(super) fn bind_texture_2d(&self, unit: usize, id: Option<glow::Texture>) {
        let mut bound_ids = self.bound_texture_2d_ids.borrow_mut();

        if bound_ids.len() <= unit {
            let diff = unit - bound_ids.len() + 1;
            bound_ids.extend(std::iter::repeat(None).take(diff));
        }

        if bound_ids[unit] == id {
            return;
        }

        let unit_gl = texture_unit_gl(unit);

        unsafe {
            self.gl.active_texture(unit_gl);
            self.gl.bind_texture(glow::TEXTURE_2D, id);
        }

        bound_ids[unit] = id;
    }

    pub(super) fn unbind_texture_2d_if_bound(&self, id: glow::Texture) {
        if let Some((location, _)) = self
            .bound_depth_attachment_id
            .get()
            .filter(|(_, bound_id)| *bound_id == id)
        {
            self.bind_draw_fbo(true);

            unsafe {
                self.gl.framebuffer_texture_2d(
                    glow::FRAMEBUFFER,
                    location,
                    glow::TEXTURE_2D,
                    None,
                    0,
                )
            };

            self.bound_depth_attachment_id.set(None);
        }

        if self
            .bound_color_attachment_ids
            .borrow()
            .iter()
            .any(|bound_id| *bound_id == id)
        {
            self.bind_draw_fbo(true);

            // TODO: We could be more efficient in how much we unbind here, but
            // I expect that this would help only rarely.
            self.bind_color_attachments(&[])
                .expect("setting empty color attachments should always succeed");
        }

        let mut bound_ids = self.bound_texture_2d_ids.borrow_mut();

        for (unit, bound_id) in bound_ids.iter_mut().enumerate() {
            if Some(id) == *bound_id {
                let unit_gl = texture_unit_gl(unit);

                unsafe {
                    self.gl.active_texture(unit_gl);
                    self.gl.bind_texture(glow::TEXTURE_2D, None);
                }

                *bound_id = None;
            }
        }
    }

    fn bind_draw_fbo(&self, yes: bool) {
        if yes != self.is_draw_fbo_bound.get() {
            let fbo = Some(self.draw_fbo).filter(|_| yes);

            unsafe { self.gl.bind_framebuffer(glow::FRAMEBUFFER, fbo) };

            self.is_draw_fbo_bound.set(yes);
        }
    }

    pub(super) fn bind_framebuffer(
        &self,
        framebuffer: &Framebuffer,
    ) -> Result<(), FramebufferError> {
        match framebuffer {
            Framebuffer::Default => {
                self.bind_draw_fbo(false);
            }
            Framebuffer::Attachments {
                color_attachments,
                depth_attachment,
            } => {
                self.bind_draw_fbo(true);
                self.bind_color_attachments(color_attachments)?;
                self.bind_depth_attachment(depth_attachment.as_ref());

                #[cfg(debug_assertions)]
                check_framebuffer_completeness(&self.gl).map_err(FramebufferError::Incomplete)?;

                #[cfg(debug_assertions)]
                check_gl_error(&self.gl, "after binding attachments")
                    .map_err(FramebufferError::Unexpected)?;
            }
        }

        Ok(())
    }

    pub(super) fn bind_uniform_buffer(&self, location: u32, id: Option<glow::Buffer>) {
        let mut bound_ids = self.bound_uniform_buffer_ids.borrow_mut();

        if bound_ids.len() <= location as usize {
            let diff = location as usize - bound_ids.len() + 1;
            bound_ids.extend(std::iter::repeat(None).take(diff));
        }

        if bound_ids[location as usize] == id {
            return;
        }

        unsafe {
            self.gl.bind_buffer_base(glow::UNIFORM_BUFFER, location, id);
        }

        bound_ids[location as usize] = id;
    }

    pub(super) fn unbind_buffer_if_bound(&self, id: glow::Buffer) {
        let mut bound_ids = self.bound_uniform_buffer_ids.borrow_mut();

        for (location, bound_id) in bound_ids.iter_mut().enumerate() {
            if Some(id) == *bound_id {
                unsafe {
                    self.gl
                        .bind_buffer_base(glow::UNIFORM_BUFFER, location as u32, None);
                }

                *bound_id = None;
            }
        }
    }
}

impl Context {
    pub fn new(gl: glow::Context) -> Result<Self, ContextError> {
        let caps = Caps::new(&gl);

        // All vertex bindings are made through a single vertex array object
        // that is bound at the start. The vertex array object binding must not
        // be changed during the lifetime of a context.
        let draw_vao = unsafe { gl.create_vertex_array() }.map_err(ContextError::ObjectCreation)?;

        unsafe { gl.bind_vertex_array(Some(draw_vao)) };

        // All framebuffer attachments are made with a single framebuffer object
        // that is created at the start.
        let draw_fbo = unsafe { gl.create_framebuffer() }.map_err(ContextError::ObjectCreation)?;

        let default_framebuffer_size = {
            let mut viewport = [0, 0, 0, 0];

            unsafe { gl.get_parameter_i32_slice(glow::VIEWPORT, &mut viewport) };

            [
                viewport[2].try_into().unwrap(),
                viewport[3].try_into().unwrap(),
            ]
        };

        let shared = Rc::new(ContextShared {
            gl,
            caps,
            draw_params: Cell::new(DrawParams::new()),
            draw_vao,
            draw_fbo,
            default_framebuffer_size: Cell::new(default_framebuffer_size),
            bound_program_id: Default::default(),
            is_draw_fbo_bound: Default::default(),
            bound_color_attachment_ids: Default::default(),
            bound_depth_attachment_id: Default::default(),
            bound_uniform_buffer_ids: Default::default(),
            bound_texture_2d_ids: Default::default(),
        });

        Ok(Self { shared })
    }

    pub fn caps(&self) -> &Caps {
        &self.shared.caps
    }

    pub fn create_buffer(
        &self,
        data: &[u8],
        target: u32,
        usage: BufferUsage,
    ) -> Result<Buffer, BufferError> {
        Buffer::new(self.shared.clone(), data, target, usage)
    }

    pub fn create_texture_2d(&self, image: Image) -> Result<Texture2d, TextureError> {
        Texture2d::new(self.shared.clone(), image)
    }

    pub fn create_texture_2d_with_mipmap(&self, image: Image) -> Result<Texture2d, TextureError> {
        Texture2d::new_with_mipmap(self.shared.clone(), image)
    }

    pub fn create_program(&self, def: ProgramDef) -> Result<Program, ProgramError> {
        Program::new(self.shared.clone(), def)
    }

    pub fn finish(&self) {
        unsafe { self.shared.gl.finish() };
    }

    pub fn default_framebuffer_size(&self) -> [u32; 2] {
        self.shared.default_framebuffer_size.get()
    }

    pub fn set_default_framebuffer_size(&self, size: [u32; 2]) {
        self.shared.default_framebuffer_size.set(size);
    }

    pub fn clear(
        &self,
        framebuffer: &Framebuffer,
        params: ClearParams,
    ) -> Result<(), FramebufferError> {
        self.shared.bind_framebuffer(framebuffer)?;

        let mut clear_mask = 0;

        if let Some(c) = params.stencil {
            unsafe { self.shared.gl.clear_stencil(c as i32) };

            clear_mask |= glow::STENCIL_BUFFER_BIT;
        }

        if let Some(c) = params.depth {
            unsafe { self.shared.gl.clear_depth_f32(c) };

            clear_mask |= glow::DEPTH_BUFFER_BIT;
        }

        if let Some(c) = params.color {
            unsafe { self.shared.gl.clear_color(c[0], c[1], c[2], c[3]) };

            clear_mask |= glow::COLOR_BUFFER_BIT;
        }

        if clear_mask > 0 {
            unsafe { self.shared.gl.clear(clear_mask) };
        }

        Ok(())
    }
}

impl Drop for ContextShared {
    fn drop(&mut self) {
        unsafe {
            self.gl.delete_vertex_array(self.draw_vao);
            self.gl.delete_framebuffer(self.draw_fbo);
        }
    }
}

fn texture_unit_gl(unit: usize) -> u32 {
    u32::try_from(unit)
        .unwrap()
        .checked_add(glow::TEXTURE0)
        .unwrap()
}
