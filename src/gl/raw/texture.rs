use std::rc::Rc;

use glow::HasContext;

use crate::gl::{image, TextureError};

use super::{Caps, ImageData};

struct Texture2dShared {
    gl: Rc<glow::Context>,
    id: glow::Texture,
    num_levels: usize,
}

pub struct Texture2d {
    shared: Rc<Texture2dShared>,
}

impl Texture2d {
    fn validate_size(size: [usize; 2], caps: &Caps) -> Result<(), TextureError> {
        if size[0] == 0 || size[1] == 0 {
            return Err(TextureError::Empty);
        }

        if size[0] > caps.max_texture_size {
            return Err(TextureError::Oversized(size[0], caps.max_texture_size));
        }

        if size[1] > caps.max_texture_size {
            return Err(TextureError::Oversized(size[1], caps.max_texture_size));
        }

        Ok(())
    }

    fn new_with_levels(
        gl: Rc<glow::Context>,
        caps: &Caps,
        image_data: ImageData,
        levels: usize,
    ) -> Result<Self, TextureError> {
        assert!(levels > 0);

        Self::validate_size(image_data.size, caps)?;

        let id = unsafe { gl.create_texture() }.map_err(TextureError::ObjectCreation)?;

        let width =
            i32::try_from(image_data.size[0]).expect("max_texture_size is out of i32 range");
        let height =
            i32::try_from(image_data.size[1]).expect("max_texture_size is out of i32 range");

        let mut buffer = Vec::new();
        let data_len = image_data.required_data_len();
        let slice = if let Some(slice) = image_data.data {
            if slice.len() != data_len {
                return Err(TextureError::DataSizeMismatch(slice.len(), data_len));
            }

            slice
        } else {
            buffer.resize(data_len, 0);
            buffer.as_slice()
        };

        unsafe {
            gl.bind_texture(glow::TEXTURE_2D, Some(id));
            gl.tex_storage_2d(
                glow::TEXTURE_2D,
                1,
                image_data.internal_format.to_gl(),
                width,
                height,
            );
            gl.tex_sub_image_2d(
                glow::TEXTURE_2D,
                0,
                0,
                0,
                width,
                height,
                image_data.internal_format.to_format().to_gl(),
                image_data.ty.to_gl(),
                glow::PixelUnpackData::Slice(slice),
            );
            gl.bind_texture(glow::TEXTURE_2D, None);
        }

        Ok(todo!())
    }

    pub(super) fn new(
        gl: Rc<glow::Context>,
        caps: &Caps,
        image_data: ImageData,
    ) -> Result<Self, TextureError> {
        Self::new_with_levels(gl, caps, image_data, 1)
    }

    pub(super) fn new_with_mipmap(
        gl: Rc<glow::Context>,
        caps: &Caps,
        image_data: ImageData,
    ) -> Result<Self, TextureError> {
        let levels = (image_data.size[0].max(image_data.size[1]) as f64).log2() as usize;

        let texture = Self::new_with_levels(gl.clone(), caps, image_data, levels)?;

        unsafe {
            gl.bind_texture(glow::TEXTURE_2D, Some(texture.shared.id));
            gl.generate_mipmap(glow::TEXTURE_2D);
            gl.bind_texture(glow::TEXTURE_2D, None);
        }

        Ok(texture)
    }
}

impl Drop for Texture2dShared {
    fn drop(&mut self) {
        unsafe {
            self.gl.delete_texture(self.id);
        }
    }
}
