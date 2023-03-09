use std::rc::Rc;

use glow::HasContext;

use super::{
    error::{check_gl_error, FramebufferError},
    texture::Texture2dShared,
    Caps, ImageInternalFormat, Texture2d,
};

struct FramebufferShared {
    gl: Rc<glow::Context>,
    id: glow::Framebuffer,
}

#[derive(Clone)]
pub enum FramebufferAttachment<'a> {
    Texture2d { texture: &'a Texture2d, level: u32 },
}

pub struct Framebuffer {
    shared: Rc<FramebufferShared>,

    // We need to keep our attachments alive.
    texture_2d_attachments: Vec<Rc<Texture2dShared>>,
}

impl Framebuffer {
    pub fn new(
        gl: Rc<glow::Context>,
        caps: &Caps,
        attachments: &[FramebufferAttachment],
    ) -> Result<Self, FramebufferError> {
        // Validate the `attachments` *before* creating or binding the new
        // framebuffer object.
        let internal_formats: Vec<_> = attachments
            .iter()
            .map(|attachment| {
                use FramebufferAttachment::*;
                match attachment {
                    Texture2d { texture, level } => {
                        // OpenGL ES 3.0.6: 4.4.2.4 Attaching Texture Images to
                        // a Framebuffer
                        // > If `textarget` is `TEXTURE_2D`, `level` must be
                        // > greater than or equal to zero and no larger than
                        // > `log_2` of the value of `MAX_TEXTURE_SIZE`.

                        let max_level = (caps.max_texture_size as f64).log2() as u32;

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
        if count(ImageInternalFormat::is_color_renderable) > caps.max_color_attachments {
            return Err(FramebufferError::TooManyColorAttachments {
                requested: count(ImageInternalFormat::is_color_renderable),
                max: caps.max_color_attachments,
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

        let id = unsafe { gl.create_framebuffer() }.map_err(FramebufferError::ObjectCreation)?;

        unsafe {
            gl.bind_framebuffer(glow::FRAMEBUFFER, Some(id));
        }

        let mut texture_2d_attachments = Vec::new();
        let mut num_color_attachments = 0;

        for (attachment, format) in attachments.iter().zip(internal_formats) {
            let location = if format.is_color_renderable() {
                num_color_attachments += 1;
                glow::COLOR_ATTACHMENT0 + (num_color_attachments - 1)
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
                FramebufferAttachment::Texture2d { texture, level } => {
                    texture_2d_attachments.push(texture.shared());

                    let level = (*level).try_into().expect("level is out of i32 range");

                    unsafe {
                        gl.framebuffer_texture_2d(
                            glow::FRAMEBUFFER,
                            location,
                            glow::TEXTURE_2D,
                            Some(texture.shared().id()),
                            level,
                        );
                    }
                }
            };
        }

        unsafe {
            gl.bind_framebuffer(glow::FRAMEBUFFER, None);
        }

        let shared = Rc::new(FramebufferShared { gl: gl.clone(), id });

        // Check for errors *after* unbinding and passing ownership of the
        // framebuffer to `shared` so that it will be cleaned up if there is an
        // error.
        check_gl_error(&gl).map_err(FramebufferError::Unexpected)?;

        Ok(Framebuffer {
            shared,
            texture_2d_attachments,
        })
    }
}
