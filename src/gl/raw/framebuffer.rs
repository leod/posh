use std::rc::Rc;

use glow::HasContext;

use super::{
    context::ContextShared,
    error::{check_framebuffer_completeness, check_gl_error, FramebufferError},
    ImageInternalFormat, Texture2d,
};

#[derive(Clone)]
pub struct FramebufferAttachment2d {
    pub texture: Rc<Texture2d>,
    pub level: u32,
}

#[derive(Clone)]
pub enum FramebufferAttachment {
    Texture2d(FramebufferAttachment2d),
}

pub struct Framebuffer {
    ctx: Rc<ContextShared>,
    id: glow::Framebuffer,

    // We need to keep our attachments alive.
    _attachments: Vec<FramebufferAttachment>,
}

#[derive(Clone)]
pub enum FramebufferBinding {
    Default,
    Framebuffer(Rc<Framebuffer>),
}

impl Framebuffer {
    pub(super) fn new(
        ctx: Rc<ContextShared>,
        attachments: &[FramebufferAttachment],
    ) -> Result<Self, FramebufferError> {
        // Validate the `attachments` *before* creating or binding the new
        // framebuffer object.
        let internal_formats: Vec<_> = attachments
            .iter()
            .map(|attachment| {
                use FramebufferAttachment::*;

                match attachment {
                    Texture2d(FramebufferAttachment2d { texture, level }) => {
                        // OpenGL ES 3.0.6: 4.4.2.4 Attaching Texture Images to
                        // a Framebuffer
                        // > If `textarget` is `TEXTURE_2D`, `level` must be
                        // > greater than or equal to zero and no larger than
                        // > `log_2` of the value of `MAX_TEXTURE_SIZE`.

                        let max_level = (ctx.caps().max_texture_size as f64).log2() as u32;

                        if *level > max_level {
                            return Err(FramebufferError::LevelTooLarge {
                                requested: *level,
                                max: max_level,
                            });
                        }

                        Ok(texture.internal_format())
                    }
                }
            })
            .collect::<Result<_, _>>()?;

        let count = |f: fn(&ImageInternalFormat) -> bool| {
            internal_formats
                .iter()
                .copied()
                .filter(f)
                .count()
                .try_into()
                .expect("number of attachments is out of u32 range")
        };

        // OpenGL ES 3.0.6: 4.4.2.4 Attaching Texture Images to a Framebuffer
        // > An `INVALID_OPERATION` is generated if `attachment` is
        // > `COLOR_ATTACHMENTm` where `m` is greater than or equal to the value
        // > of `MAX_COLOR_ATTACHMENTS`.
        if count(ImageInternalFormat::is_color_renderable) > ctx.caps().max_color_attachments {
            return Err(FramebufferError::TooManyColorAttachments {
                requested: count(ImageInternalFormat::is_color_renderable),
                max: ctx.caps().max_color_attachments,
            });
        }

        // OpenGL ES 3.0.6: 4.2.1 Selecting Buffers for Writing
        // > An `INVALID_VALUE` error is generated if `n` is negative, or
        // > greater than the value of `MAX_DRAW_BUFFERS`.

        // Note that we currently always activate all color attachments in our
        // call to `draw_buffers`. That is why we need to check the count here.
        // In the future, we will hopefully allow rendering to subsets of
        // framebuffer attachments, but I don't know yet how this should look on
        // the type level.
        if count(ImageInternalFormat::is_color_renderable) > ctx.caps().max_draw_buffers {
            return Err(FramebufferError::TooManyDrawBuffers {
                requested: count(ImageInternalFormat::is_color_renderable),
                max: ctx.caps().max_draw_buffers,
            });
        }

        // There is only one depth attachment location.
        if count(ImageInternalFormat::is_depth_renderable) > 1 {
            return Err(FramebufferError::TooManyDepthAttachments {
                requested: count(ImageInternalFormat::is_depth_renderable),
            });
        }

        // There is only one stencil attachment location.
        if count(ImageInternalFormat::is_stencil_renderable) > 1 {
            return Err(FramebufferError::TooManyStencilAttachments {
                requested: count(ImageInternalFormat::is_stencil_renderable),
            });
        }

        let gl = ctx.gl();
        let id = unsafe { gl.create_framebuffer() }.map_err(FramebufferError::ObjectCreation)?;
        unsafe {
            gl.bind_framebuffer(glow::FRAMEBUFFER, Some(id));
        }

        let mut draw_buffers = Vec::new();

        for (attachment, format) in attachments.iter().zip(internal_formats) {
            let location = if format.is_color_renderable() {
                let location = glow::COLOR_ATTACHMENT0 + draw_buffers.len() as u32;
                draw_buffers.push(location);
                location
            } else if format.is_depth_renderable() && format.is_stencil_renderable() {
                glow::DEPTH_STENCIL_ATTACHMENT
            } else if format.is_depth_renderable() {
                glow::DEPTH_ATTACHMENT
            } else if format.is_stencil_renderable() {
                glow::STENCIL_ATTACHMENT
            } else {
                panic!(
                    "every ImageInternalFormat must satisfy at least one of 
                     color renderable, depth renderable, or stencil renderable"
                );
            };

            match attachment {
                FramebufferAttachment::Texture2d(FramebufferAttachment2d { texture, level }) => {
                    let level = (*level).try_into().expect("level is out of i32 range");

                    unsafe {
                        gl.framebuffer_texture_2d(
                            glow::FRAMEBUFFER,
                            location,
                            glow::TEXTURE_2D,
                            Some(texture.id()),
                            level,
                        );
                    }
                }
            };
        }

        // We currently always activate all color attachments. See the comment
        // on `max_draw_buffers` above for more detail.
        unsafe {
            gl.draw_buffers(&draw_buffers);
        }

        let completeness = check_framebuffer_completeness(gl);

        unsafe {
            gl.bind_framebuffer(glow::FRAMEBUFFER, None);
        }

        let framebuffer = Framebuffer {
            ctx: ctx.clone(),
            id,
            _attachments: attachments.to_vec(),
        };

        // Check for errors *after* unbinding the framebuffer and passing
        // ownership, so that we leave in a clean state.
        completeness.map_err(FramebufferError::Incomplete)?;
        check_gl_error(gl).map_err(FramebufferError::Unexpected)?;

        Ok(framebuffer)
    }

    pub(super) fn context(&self) -> &ContextShared {
        &self.ctx
    }
}

impl FramebufferBinding {
    pub(super) fn bind(&self, ctx: &ContextShared) {
        use FramebufferBinding::*;

        match self {
            Default => {}
            Framebuffer(framebuffer) => unsafe {
                assert!(framebuffer.context().ref_eq(ctx));

                let gl = ctx.gl();
                gl.bind_framebuffer(glow::FRAMEBUFFER, Some(framebuffer.id));

                check_framebuffer_completeness(gl)
                    .expect("framebuffer turned incomplete after creation");
            },
        }
    }

    pub(super) fn unbind(&self, gl: &glow::Context) {
        use FramebufferBinding::*;

        match self {
            Default => {}
            Framebuffer(_) => unsafe {
                gl.bind_framebuffer(glow::FRAMEBUFFER, None);
            },
        }
    }
}
