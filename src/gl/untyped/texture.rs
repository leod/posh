use std::rc::Rc;

use glow::HasContext;

use crate::gl::TextureError;

use super::ImageData;

struct Texture2dShared {
    gl: Rc<glow::Context>,
    id: glow::Texture,
    num_levels: usize,
}

pub struct Texture2d {
    shared: Rc<Texture2dShared>,
}

impl Texture2d {
    fn new(gl: Rc<glow::Context>, data: ImageData) -> Result<Self, TextureError> {
        let id = unsafe { gl.create_texture() }.map_err(TextureError::Create)?;

        unsafe {
            gl.tex_storage_2d(
                glow::TEXTURE_2D,
                1,
                data.internal_format.to_gl(),
                i32::try_from(data.size[0]).unwrap(),
                i32::try_from(data.size[1]).unwrap(),
            );
        }

        Ok(todo!())
    }
}
