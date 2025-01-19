use std::rc::Rc;

use smallvec::SmallVec;

use super::{
    context::ContextShared, Comparison, ImageInternalFormat, Sampler2d, Sampler2dParams, Texture2d,
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

    pub fn id(&self) -> glow::Texture {
        use Attachment::*;

        match self {
            Texture2d { texture, .. } => texture.id(),
        }
    }
}

pub type AttachmentVec = SmallVec<[Attachment; 8]>;

#[derive(Clone)]
pub enum Framebuffer {
    Default,
    Attachments {
        color_attachments: AttachmentVec,
        depth_attachment: Option<Attachment>,
    },
}

impl Framebuffer {
    pub(super) fn size(&self, ctx: &ContextShared) -> [u32; 2] {
        use Framebuffer::*;

        match self {
            Default => ctx.default_framebuffer_size(),
            Attachments {
                color_attachments,
                depth_attachment,
            } => color_attachments
                .iter()
                .chain(depth_attachment)
                .map(|attachment| attachment.size())
                .fold([0, 0], |[x1, y1], [x2, y2]| [x1.max(x2), y1.max(y2)]),
        }
    }
}
