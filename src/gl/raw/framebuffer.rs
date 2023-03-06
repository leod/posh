use std::rc::Rc;

use glow::HasContext;

use super::{error::FramebufferError, texture::Texture2dShared, Caps, Texture2d};

struct FramebufferShared {
    gl: Rc<glow::Context>,
    id: glow::Framebuffer,
}

#[derive(Clone)]
pub enum FramebufferAttachment<'a> {
    Texture2d {
        texture: &'a Texture2d,
        layer: usize,
    },
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
        let id = unsafe { gl.create_framebuffer() }.map_err(FramebufferError::ObjectCreation)?;

        unsafe {
            gl.bind_framebuffer(glow::FRAMEBUFFER, Some(id));
        }

        let shared = Rc::new(FramebufferShared { gl, id });

        let texture_2d_attachments = attachments
            .iter()
            .filter_map(|attachment| {
                let FramebufferAttachment::Texture2d { texture, .. } = attachment else { return None; };
                Some(texture.shared())
            })
            .collect();

        Ok(Framebuffer {
            shared,
            texture_2d_attachments,
        })
    }
}
