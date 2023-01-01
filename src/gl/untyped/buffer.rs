use std::{rc::Rc, cell::Cell};

use bytemuck::Pod;
use glow::HasContext;

use crate::gl::{BufferUsage, CreateBufferError};

pub struct Buffer {
    gl: Rc<glow::Context>,
    id: glow::Buffer,
    usage: BufferUsage,
    len: Cell<usize>,
}

impl Buffer {
    pub(crate) fn new<T: Pod>(
        gl: Rc<glow::Context>,
        data: &[T],
        usage: BufferUsage,
    ) -> Result<Self, CreateBufferError> {
        let id = unsafe { gl.create_buffer() }.map_err(CreateBufferError)?;

        let buffer = Buffer {
            gl,
            id,
            usage,
            len: Cell::new(0),
        };


        buffer.set(data);

        Ok(buffer)
    }

    pub fn gc(&self) -> &Rc<glow::Context> {
        &self.gl
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

    pub fn set<T: Pod>(&self, data: &[T]) {
        let raw_data = bytemuck::cast_slice(data);

        // We can get away with always using `ARRAY_BUFFER` as the target here,
        // since the target does not carry any meaning for setting data. It is
        // just a binding point.
        let target = glow::ARRAY_BUFFER;

        unsafe {
            self.gl.bind_buffer(target, Some(self.id));
            self.gl.buffer_data_u8_slice(target, raw_data, self.usage.to_gl());

            // TODO: Could avoid unbinding here by using `ContextShared`.
            self.gl.bind_buffer(target, None);
        }

        self.len.set(raw_data.len());
    }
}

impl Drop for Buffer {
    fn drop(&mut self) {
        unsafe {
            self.gl.delete_buffer(self.id);
        }
    }
}
