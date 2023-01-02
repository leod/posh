use std::{cell::Cell, rc::Rc};

use bytemuck::Pod;
use glow::HasContext;

use crate::gl::{BufferUsage, CreateBufferError};

struct BufferShared {
    gl: Rc<glow::Context>,
    id: glow::Buffer,
    usage: BufferUsage,
    len: Cell<usize>,
}

pub struct Buffer {
    shared: Rc<BufferShared>,
}

#[derive(Clone)]
pub struct BufferBinding {
    shared: Rc<BufferShared>,
}

impl Buffer {
    pub(crate) fn new<T: Pod>(
        gl: Rc<glow::Context>,
        data: &[T],
        usage: BufferUsage,
    ) -> Result<Self, CreateBufferError> {
        let id = unsafe { gl.create_buffer() }.map_err(CreateBufferError)?;

        let shared = Rc::new(BufferShared {
            gl,
            id,
            usage,
            len: Cell::new(0),
        });

        let buffer = Buffer { shared };

        buffer.set(data);

        Ok(buffer)
    }

    pub fn gl(&self) -> &Rc<glow::Context> {
        &self.shared.gl
    }

    pub fn id(&self) -> glow::Buffer {
        self.shared.id
    }

    pub fn usage(&self) -> BufferUsage {
        self.shared.usage
    }

    pub fn len(&self) -> usize {
        self.shared.len.get()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn set<T: Pod>(&self, data: &[T]) {
        let gl = &self.shared.gl;
        let raw_data = bytemuck::cast_slice(data);

        // We can get away with always using `ARRAY_BUFFER` as the target here,
        // since the target does not carry any meaning for setting data. It is
        // just a binding point.
        let target = glow::ARRAY_BUFFER;

        unsafe {
            gl.bind_buffer(target, Some(self.shared.id));
            gl.buffer_data_u8_slice(target, raw_data, self.shared.usage.to_gl());

            // TODO: Could avoid unbinding here by using `ContextShared`.
            gl.bind_buffer(target, None);
        }

        self.shared.len.set(raw_data.len());
    }

    #[must_use]
    pub fn bind(&self) -> BufferBinding {
        BufferBinding {
            shared: self.shared.clone(),
        }
    }
}

impl BufferBinding {
    pub fn gl(&self) -> &Rc<glow::Context> {
        &self.shared.gl
    }

    pub fn id(&self) -> glow::Buffer {
        self.shared.id
    }

    pub fn len(&self) -> usize {
        self.shared.len.get()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl Drop for BufferShared {
    fn drop(&mut self) {
        unsafe {
            self.gl.delete_buffer(self.id);
        }
    }
}
