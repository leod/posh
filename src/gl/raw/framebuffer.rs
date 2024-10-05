use std::rc::Rc;

use glow::HasContext;

use super::{
    context::ContextShared,
    error::{check_framebuffer_completeness, check_gl_error, FramebufferError},
    Caps, Comparison, ImageInternalFormat, Sampler2d, Sampler2dParams, Texture2d,
};

#[derive(Clone)]
pub enum Attachment {
    Texture2d { texture: Rc<Texture2d>, level: u32 },
}

impl Attachment {
    pub fn size(&self) -> [u32; 2] {
        use Attachment::*;

        match self {
            Texture2d { texture, .. } => texture.size(),
        }
    }

    pub fn internal_format(&self) -> ImageInternalFormat {
        use Attachment::*;

        match self {
            Texture2d { texture, .. } => texture.internal_format(),
        }
    }

    pub fn sampler(&self, params: Sampler2dParams, comparison: Option<Comparison>) -> Sampler2d {
        use Attachment::*;

        match self {
            Texture2d { texture, .. } => Sampler2d {
                texture: texture.clone(),
                params,
                comparison,
            },
        }
    }
}

#[derive(Clone)]
pub enum Framebuffer {
    Default,
    Attachments { attachments: Vec<Attachment> },
}

impl Framebuffer {
    pub(super) fn size(&self, ctx: &ContextShared) -> [u32; 2] {
        use Framebuffer::*;

        match self {
            Default => ctx.default_framebuffer_size(),
            Attachments { attachments } => attachments
                .iter()
                .map(|attachment| attachment.size())
                .fold([0, 0], |[x1, y1], [x2, y2]| [x1.max(x2), y1.max(y2)]),
        }
    }
}

fn with_locations(attachments: &[Attachment]) -> impl Iterator<Item = (u32, &Attachment)> {
    attachments.iter().scan(0, |num_color, attachment| {
        let format = attachment.internal_format();
        let location = if format.is_color_renderable() {
            let location = glow::COLOR_ATTACHMENT0 + *num_color as u32;
            *num_color += 1;
            location
        } else if format.is_depth_renderable() && format.is_stencil_renderable() {
            glow::DEPTH_STENCIL_ATTACHMENT
        } else if format.is_depth_renderable() {
            glow::DEPTH_ATTACHMENT
        } else if format.is_stencil_renderable() {
            glow::STENCIL_ATTACHMENT
        } else {
            // FIXME: This does not actually hold! Also, `is_color_renderable`
            // depends on available extensions. All of this needs to be checked
            // here.
            panic!(
                "every ImageInternalFormat must satisfy at least one of color renderable, depth \
                 renderable, or stencil renderable"
            );
        };

        Some((location, attachment))
    })
}

fn bind_attachments(
    ctx: &ContextShared,
    attachments: &[Attachment],
) -> Result<(), FramebufferError> {
    let gl = ctx.gl();

    for (location, attachment) in with_locations(attachments) {
        match attachment {
            Attachment::Texture2d { texture, level } => {
                let level = (*level).try_into().expect("level is out of i32 range");

                unsafe {
                    gl.framebuffer_texture_2d(
                        glow::FRAMEBUFFER,
                        location,
                        glow::TEXTURE_2D,
                        Some(texture.id()),
                        level,
                    )
                };
            }
        };
    }

    let draw_buffers: Vec<_> = with_locations(attachments)
        .filter(|(_, attachment)| attachment.internal_format().is_color_renderable())
        .map(|(location, _)| location)
        .collect();

    unsafe { gl.draw_buffers(&draw_buffers) };

    #[cfg(debug_assertions)]
    check_framebuffer_completeness(gl).map_err(FramebufferError::Incomplete)?;

    #[cfg(debug_assertions)]
    check_gl_error(gl, "after binding attachments").map_err(FramebufferError::Unexpected)?;

    Ok(())
}

fn unbind_attachments(ctx: &ContextShared, attachments: &[Attachment]) {
    let gl = ctx.gl();

    for (location, attachment) in with_locations(attachments) {
        match attachment {
            Attachment::Texture2d { level, .. } => {
                let level = (*level).try_into().expect("level is out of i32 range");

                unsafe {
                    gl.framebuffer_texture_2d(
                        glow::FRAMEBUFFER,
                        location,
                        glow::TEXTURE_2D,
                        None,
                        level,
                    )
                };
            }
        };
    }
}

impl Framebuffer {
    pub(super) fn bind(&self, ctx: &ContextShared) -> Result<(), FramebufferError> {
        use Framebuffer::*;

        match self {
            Framebuffer::Default => Ok(()),
            Attachments { attachments } => {
                validate_attachments(ctx.caps(), attachments)?;

                unsafe {
                    ctx.gl()
                        .bind_framebuffer(glow::FRAMEBUFFER, Some(ctx.draw_fbo()))
                };

                bind_attachments(ctx, attachments)
            }
        }
    }

    pub(super) fn unbind(&self, ctx: &ContextShared) {
        use Framebuffer::*;

        match self {
            Framebuffer::Default => {}
            Attachments { attachments } => {
                // TODO: Remove overly conservative unbinding.
                unbind_attachments(ctx, attachments);

                // TODO: Remove overly conservative unbinding.
                unsafe { ctx.gl().bind_framebuffer(glow::FRAMEBUFFER, None) };
            }
        }
    }
}

fn validate_attachments(caps: &Caps, attachments: &[Attachment]) -> Result<(), FramebufferError> {
    for attachment in attachments {
        use Attachment::*;

        match attachment {
            Texture2d { level, .. } => {
                // OpenGL ES 3.0.6: 4.4.2.4 Attaching Texture Images to a
                // Framebuffer
                // > If `textarget` is `TEXTURE_2D`, `level` must be greater
                // > than or equal to zero and no larger than `log_2` of the
                // > value of `MAX_TEXTURE_SIZE`.

                let max_level = (caps.max_texture_size as f64).log2() as u32;

                if *level > max_level {
                    return Err(FramebufferError::LevelTooLarge {
                        requested: *level,
                        max: max_level,
                    });
                }
            }
        }
    }

    let count = |f: fn(&ImageInternalFormat) -> bool| {
        attachments
            .iter()
            .map(Attachment::internal_format)
            .filter(f)
            .count()
            .try_into()
            .expect("number of attachments is out of u32 range")
    };

    // OpenGL ES 3.0.6: 4.4.2.4 Attaching Texture Images to a Framebuffer
    // > An `INVALID_OPERATION` is generated if `attachment` is
    // > `COLOR_ATTACHMENTm` where `m` is greater than or equal to the value of
    // > `MAX_COLOR_ATTACHMENTS`.
    if count(ImageInternalFormat::is_color_renderable) > caps.max_color_attachments {
        return Err(FramebufferError::TooManyColorAttachments {
            requested: count(ImageInternalFormat::is_color_renderable),
            max: caps.max_color_attachments,
        });
    }

    // OpenGL ES 3.0.6: 4.2.1 Selecting Buffers for Writing
    // > An `INVALID_VALUE` error is generated if `n` is negative, or greater
    // > than the value of `MAX_DRAW_BUFFERS`.
    if count(ImageInternalFormat::is_color_renderable) > caps.max_draw_buffers {
        return Err(FramebufferError::TooManyDrawBuffers {
            requested: count(ImageInternalFormat::is_color_renderable),
            max: caps.max_draw_buffers,
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

    // FIXME: Do we need to check for the presence of at least one attachment
    // here?

    Ok(())
}
