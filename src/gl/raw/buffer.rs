use std::{cell::Cell, rc::Rc};

use glow::HasContext;

use super::{context::ContextShared, error::check_gl_error, BufferError};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BufferUsage {
    StreamDraw,
    StreamRead,
    StaticDraw,
    StaticRead,
    DynamicDraw,
    DynamicRead,
}

impl BufferUsage {
    pub const fn to_gl(self) -> u32 {
        use BufferUsage::*;

        match self {
            StreamDraw => glow::STREAM_DRAW,
            StreamRead => glow::STREAM_READ,
            StaticDraw => glow::STATIC_DRAW,
            StaticRead => glow::STATIC_READ,
            DynamicDraw => glow::DYNAMIC_DRAW,
            DynamicRead => glow::DYNAMIC_READ,
        }
    }
}

pub struct Buffer {
    ctx: Rc<ContextShared>,
    id: glow::Buffer,
    target: u32,
    usage: BufferUsage,
    len: Cell<usize>,
}

impl Buffer {
    pub(super) fn new(
        ctx: Rc<ContextShared>,
        data: &[u8],
        target: u32,
        usage: BufferUsage,
    ) -> Result<Self, BufferError> {
        let gl = ctx.gl();
        let id = unsafe { gl.create_buffer() }.map_err(BufferError::ObjectCreation)?;

        let buffer = Buffer {
            ctx: ctx.clone(),
            id,
            target,
            usage,
            len: Cell::new(0),
        };

        buffer.set(data);

        check_gl_error(gl, "after new buffer").map_err(BufferError::Unexpected)?;

        Ok(buffer)
    }

    pub(super) fn context(&self) -> &ContextShared {
        &self.ctx
    }

    pub fn id(&self) -> glow::Buffer {
        self.id
    }

    pub fn usage(&self) -> BufferUsage {
        self.usage
    }

    pub fn len(&self) -> usize {
        self.len.get()
    }

    pub fn is_empty(&self) -> bool {
        self.len() != 0
    }

    pub fn set(&self, data: &[u8]) {
        let gl = self.ctx.gl();

        unsafe {
            gl.bind_buffer(self.target, Some(self.id));
            gl.buffer_data_u8_slice(self.target, data, self.usage.to_gl());

            // TODO: Could avoid unbinding here by using `ContextShared`.
            gl.bind_buffer(self.target, None);
        }

        self.len.set(data.len());

        #[cfg(debug_assertions)]
        check_gl_error(gl, "after buffer set").expect("OpenGL error after Buffer::set");
    }
}

impl Drop for Buffer {
    fn drop(&mut self) {
        self.ctx.unbind_buffer_if_bound(self.id);

        let gl = self.ctx.gl();

        unsafe {
            gl.delete_buffer(self.id);
        }
    }
}
