use std::{cell::Cell, rc::Rc};

use glow::HasContext;

use crate::gl::{raw::error::check_gl_error, TextureError};

use super::{
    context::ContextShared, sampler_params::set_comparison, Caps, Comparison, Image,
    ImageInternalFormat, Sampler2dParams,
};

pub struct Texture2d {
    ctx: Rc<ContextShared>,
    id: glow::Texture,
    size: [u32; 2],
    internal_format: ImageInternalFormat,
    levels: usize,
    params: Cell<Sampler2dParams>,
    comparison: Cell<Option<Comparison>>,
}

#[derive(Clone)]
pub enum Sampler {
    Sampler2d(Sampler2d),
}

#[derive(Clone)]
pub struct Sampler2d {
    pub texture: Rc<Texture2d>,
    pub params: Sampler2dParams,
    pub comparison: Option<Comparison>,
}

struct ImageData<'a> {
    image: &'a Image<'a>,
    buffer: Vec<u8>,
}

impl<'a> ImageData<'a> {
    fn new(image: &'a Image<'a>) -> Self {
        Self {
            image,
            buffer: Vec::new(),
        }
    }

    fn as_slice(&mut self) -> Result<&[u8], TextureError> {
        let len = self.image.required_data_len();

        if let Some(slice) = self.image.data {
            // Safety: check that `slice` has the correct size.
            if slice.len() != len {
                return Err(TextureError::DataSizeMismatch {
                    expected: len,
                    got: slice.len(),
                });
            }

            Ok(slice)
        } else {
            self.buffer.resize(len, 0);
            Ok(self.buffer.as_slice())
        }
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
        let width = image.size[0]
            .try_into()
            .expect("max_texture_size is out of i32 range");
        let height = image.size[1]
            .try_into()
            .expect("max_texture_size is out of i32 range");

        let mut data = ImageData::new(&image);
        let slice = data.as_slice()?;

        let gl = ctx.gl();

        // OpenGL expects each row of image data to have a specific alignment.
        // The default row alignment is 4. However, in our case, we tightly pack
        // image data, so we specify a row alignment is 1.
        //
        // Note that this alignment refers to the image data that we provide for
        // the upload. It does not affect how the image is stored on the GPU.
        //
        // See also:
        // <https://www.khronos.org/opengl/wiki/Pixel_Transfer#Pixel_layout>.
        unsafe {
            gl.pixel_store_i32(glow::UNPACK_ALIGNMENT, 1);
        }

        let id = unsafe { gl.create_texture() }.map_err(TextureError::ObjectCreation)?;

        ctx.bind_texture_2d(0, Some(id));

        unsafe {
            gl.tex_storage_2d(
                glow::TEXTURE_2D,
                levels,
                image.internal_format.to_gl(),
                width,
                height,
            )
        };
        unsafe {
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
            )
        };

        let texture = Texture2d {
            ctx: ctx.clone(),
            id,
            size: image.size,
            internal_format: image.internal_format,
            levels: levels as usize,
            params: Default::default(),
            comparison: Default::default(),
        };

        // Check for errors *after* passing ownership of the texture to
        // `shared` so that it will be cleaned up if there is an error.
        check_gl_error(gl, "after new texture").map_err(TextureError::Unexpected)?;

        Ok(texture)
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
        let levels = (image.size[0].max(image.size[1]) as f64).log2() as usize + 1;

        let texture = Self::new_with_levels(ctx.clone(), image, levels)?;

        ctx.bind_texture_2d(0, Some(texture.id));

        let gl = ctx.gl();

        unsafe {
            gl.generate_mipmap(glow::TEXTURE_2D);
        }

        check_gl_error(gl, "after new texture with mipmaps").map_err(TextureError::Unexpected)?;

        Ok(texture)
    }

    pub(super) fn id(&self) -> glow::Texture {
        self.id
    }

    pub fn size(&self) -> [u32; 2] {
        self.size
    }

    pub fn internal_format(&self) -> ImageInternalFormat {
        self.internal_format
    }

    pub fn set(
        &self,
        level: usize,
        lower_left_corner: [u32; 2],
        image: Image,
    ) -> Result<(), TextureError> {
        assert!(level <= self.levels);
        assert_eq!(self.internal_format, image.internal_format);

        let mut data = ImageData::new(&image);
        let slice = data.as_slice()?;

        let gl = self.ctx.gl();

        let level = level.try_into().unwrap();
        let x = lower_left_corner[0].try_into().unwrap();
        let y = lower_left_corner[1].try_into().unwrap();
        let width = image.size[0].try_into().unwrap();
        let height = image.size[1].try_into().unwrap();

        self.ctx.bind_texture_2d(0, Some(self.id));

        unsafe {
            gl.tex_sub_image_2d(
                glow::TEXTURE_2D,
                level,
                x,
                y,
                width,
                height,
                image.internal_format.to_format().to_gl(),
                image.ty.to_gl(),
                glow::PixelUnpackData::Slice(slice),
            )
        };

        // This might be triggered if `rect` is outside of the texture image
        // bounds.
        check_gl_error(gl, "after texture set").map_err(TextureError::Unexpected)?;

        Ok(())
    }

    pub(super) fn set_params(&self, new: Sampler2dParams, comparison: Option<Comparison>) {
        let gl = &self.ctx.gl();

        let current = self.params.get();
        new.set_delta(gl, &current);
        self.params.set(new);

        if self.comparison.get() != comparison {
            // FIXME: Check that comparison can be applied to the texture.
            set_comparison(gl, glow::TEXTURE_2D, comparison);

            self.comparison.set(comparison);
        }

        #[cfg(debug_assertions)]
        check_gl_error(gl, "after texture params").unwrap();
    }
}

impl Drop for Texture2d {
    fn drop(&mut self) {
        self.ctx.unbind_texture_2d_if_bound(self.id);

        let gl = self.ctx.gl();

        unsafe {
            gl.delete_texture(self.id);
        }
    }
}

impl Sampler {
    pub(super) fn context(&self) -> &ContextShared {
        use Sampler::*;

        match self {
            Sampler2d(texture) => &texture.texture.ctx,
        }
    }

    pub(super) fn bind(&self, unit: usize) {
        match self {
            Sampler::Sampler2d(Sampler2d {
                texture,
                params,
                comparison,
            }) => {
                let id = texture.id;

                texture.ctx.bind_texture_2d(unit, Some(id));
                texture.set_params(*params, *comparison);
            }
        }
    }
}

fn validate_size(size: [u32; 2], caps: &Caps) -> Result<(), TextureError> {
    // OpenGL ES 3.0.6: 3.8.4 Immutable-Format Texture Images
    // > If [...] `width`, `height` [...] is less than 1, the error
    // > `INVALID_VALUE` is generated.
    if size[0] == 0 || size[1] == 0 {
        return Err(TextureError::Empty);
    }

    if size[0] > caps.max_texture_size {
        return Err(TextureError::Oversized {
            requested: size[0],
            max: caps.max_texture_size,
        });
    }

    if size[1] > caps.max_texture_size {
        return Err(TextureError::Oversized {
            requested: size[1],
            max: caps.max_texture_size,
        });
    }

    Ok(())
}
