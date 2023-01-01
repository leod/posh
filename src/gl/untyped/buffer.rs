use std::rc::Rc;

use glow::HasContext;

pub struct Buffer {
    gl: Rc<glow::Context>,
    id: glow::Buffer,
}

impl Drop for Buffer {
    fn drop(&mut self) {
        unsafe {
            self.gl.delete_buffer(self.id);
        }
    }
}
