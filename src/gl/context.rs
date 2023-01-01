use std::rc::Rc;

use bytemuck::Pod;

use super::{BufferUsage, untyped, CreateBufferError};

pub(crate) struct ContextShared {}

pub struct Context {
    shared: Rc<ContextShared>,
    gl: Rc<glow::Context>,
}

impl Context {
    pub(crate) fn shared(&self) -> &Rc<ContextShared> {
        &self.shared
    }

    pub fn gl(&self) -> &Rc<glow::Context> {
        &self.gl
    }

    pub fn untyped_create_buffer<T: Pod>(&self, data: &[T], usage: BufferUsage) -> Result<untyped::Buffer, CreateBufferError> {
        untyped::Buffer::new(self.gl.clone(), data, usage)
    }
}
