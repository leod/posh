use std::{cell::Cell, rc::Rc};

use glow::HasContext;

use crate::gl::{raw::error::check_gl_error, TextureError};

use super::{context::ContextShared, Caps, Image, ImageInternalFormat, Sampler2dParams};

pub(super) struct Texture2dShared {
    ctx: Rc<ContextShared>,
    id: glow::Texture,
    internal_format: ImageInternalFormat,
    levels: usize,
    sampler_params: Cell<Sampler2dParams>,
}

#[derive(Clone)]
pub enum Sampler {
    Sampler2d(Sampler2d),
}

#[derive(Clone)]
pub struct Sampler2d {
    shared: Rc<Texture2dShared>,
    params: Sampler2dParams,
}

pub struct Texture2d {
    shared: Rc<Texture2dShared>,
}

impl Texture2dShared {
    pub(super) fn id(&self) -> glow::Texture {
        self.id
    }

    pub(super) fn set_sampler_params(&self, new: Sampler2dParams) {
        let gl = &self.ctx.gl();

        let current = self.sampler_params.get();
        new.set_delta(gl, &current);
        self.sampler_params.set(new);

        check_gl_error(gl).unwrap();
    }
}

impl Texture2d {
    fn new_with_levels(
        ctx: Rc<ContextShared>,
        image: Image,
        levels: usize,
    ) -> Result<Self, TextureError> {
        // OpenGL ES 3.0.6: 3.8.4 Immutable-Format Texture Images
        // > If [...] `levels` is less than 1, the error `INVALID_VALUE` is
        // > generated.
        assert!(levels > 0);

        let levels = levels.try_into().expect("levels is out of i32 range");
        let width = image
            .size
            .x
            .try_into()
            .expect("max_texture_size is out of i32 range");
        let height = image
            .size
            .y
            .try_into()
            .expect("max_texture_size is out of i32 range");

        let mut buffer = Vec::new();
        let data_len = image.required_data_len();
        let slice = if let Some(slice) = image.data {
            // Safety: check that `slice` has the correct size.
            if slice.len() != data_len {
                return Err(TextureError::DataSizeMismatch {
                    expected: data_len,
                    got: slice.len(),
                });
            }

            slice
        } else {
            buffer.resize(data_len, 0);
            buffer.as_slice()
        };

        let gl = ctx.gl();
        let id = unsafe { gl.create_texture() }.map_err(TextureError::ObjectCreation)?;

        unsafe {
            gl.bind_texture(glow::TEXTURE_2D, Some(id));
            gl.tex_storage_2d(
                glow::TEXTURE_2D,
                levels,
                image.internal_format.to_gl(),
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
                image.internal_format.to_format().to_gl(),
                image.ty.to_gl(),
                glow::PixelUnpackData::Slice(slice),
            );
            gl.bind_texture(glow::TEXTURE_2D, None);
        }

        let shared = Rc::new(Texture2dShared {
            ctx: ctx.clone(),
            id,
            internal_format: image.internal_format,
            levels: levels as usize,
            sampler_params: Default::default(),
        });

        // Check for errors *after* passing ownership of the texture to
        // `shared` so that it will be cleaned up if there is an error.
        check_gl_error(gl).map_err(TextureError::Unexpected)?;

        Ok(Texture2d { shared })
    }

    pub(super) fn new(ctx: Rc<ContextShared>, image: Image) -> Result<Self, TextureError> {
        validate_size(image.size, ctx.caps())?;

        Self::new_with_levels(ctx, image, 1)
    }

    pub(super) fn new_with_mipmap(
        ctx: Rc<ContextShared>,
        image: Image,
    ) -> Result<Self, TextureError> {
        validate_size(image.size, ctx.caps())?;

        // OpenGL ES 3.0.6: 3.8.4 Immutable-Format Texture Images
        // > An INVALID_OPERATION error is generated if `levels` is greater than
        // > `floor(log_2(max(width, height))) + 1`.
        let levels = (image.size.x.max(image.size.y) as f64).log2() as usize + 1;

        let texture = Self::new_with_levels(ctx.clone(), image, levels)?;
        let gl = ctx.gl();

        unsafe {
            gl.bind_texture(glow::TEXTURE_2D, Some(texture.shared.id));
            gl.generate_mipmap(glow::TEXTURE_2D);
            gl.bind_texture(glow::TEXTURE_2D, None);
        }

        check_gl_error(gl).map_err(TextureError::Unexpected)?;

        Ok(texture)
    }

    pub(super) fn shared(&self) -> Rc<Texture2dShared> {
        self.shared.clone()
    }

    pub fn internal_format(&self) -> ImageInternalFormat {
        self.shared.internal_format
    }

    pub fn sampler(&self, params: Sampler2dParams) -> Sampler2d {
        Sampler2d {
            shared: self.shared.clone(),
            params,
        }
    }
}

impl Drop for Texture2dShared {
    fn drop(&mut self) {
        let gl = self.ctx.gl();

        unsafe {
            gl.delete_texture(self.id);
        }
    }
}

fn validate_size(size: glam::UVec2, caps: &Caps) -> Result<(), TextureError> {
    // OpenGL ES 3.0.6: 3.8.4 Immutable-Format Texture Images
    // > If [...] `width`, `height` [...] is less than 1, the error
    // > `INVALID_VALUE` is generated.
    if size.x == 0 || size.y == 0 {
        return Err(TextureError::Empty);
    }

    if size.x > caps.max_texture_size {
        return Err(TextureError::Oversized {
            requested: size.x,
            max: caps.max_texture_size,
        });
    }

    if size.y > caps.max_texture_size {
        return Err(TextureError::Oversized {
            requested: size.y,
            max: caps.max_texture_size,
        });
    }

    Ok(())
}

impl Sampler {
    pub(super) fn context(&self) -> &ContextShared {
        use Sampler::*;

        match self {
            Sampler2d(texture) => &texture.shared.ctx,
        }
    }

    pub(super) fn bind(&self) {
        use Sampler::*;

        match self {
            Sampler2d(texture) => {
                let gl = texture.shared.ctx.gl();
                let id = texture.shared.id;

                unsafe {
                    gl.bind_texture(glow::TEXTURE_2D, Some(id));
                }

                texture.shared.set_sampler_params(texture.params);
            }
        }
    }

    pub(super) fn unbind(&self) {
        use Sampler::*;

        match self {
            Sampler2d(texture) => {
                let gl = texture.shared.ctx.gl();

                unsafe {
                    gl.bind_texture(glow::TEXTURE_2D, None);
                }
            }
        }
    }
}
