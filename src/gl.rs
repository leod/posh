//! The graphics library.

mod context;
mod element_buffer;
mod framebuffer;
mod image;
mod mat;
mod program;
mod raw;
mod texture;
mod uniform_buffer;
mod vec;
mod vertex_buffer;
mod vertex_spec;

use bytemuck::{Pod, Zeroable};
use crevice::std140::AsStd140;

use crate::{sl, ToSl};

pub use self::image::{ColorImage, DepthImage};
pub use context::{CacheDrawBuilder, Context};
pub use element_buffer::{Element, ElementBuffer, ElementBufferBinding};
pub use framebuffer::{ColorAttachment, DepthAttachment, Framebuffer};
pub use mat::{Mat2, Mat3, Mat4};
pub use program::{
    DrawBuilder, DrawBuilderWithFramebuffer, DrawBuilderWithUniforms,
    DrawBuilderWithUniformsAndFramebuffer, Program,
};
pub use raw::{
    BlendEquation, BlendFunc, Blending, BufferError, BufferUsage, Caps, ClearParams, Comparison,
    ContextError, CreateError, CullFace, DrawError, DrawParams, ElementType, FramebufferError,
    ImageFormat, ImageInternalFormat, PrimitiveMode, ProgramError, ProgramValidationError, Rect,
    Sampler2dParams, SamplerMagFilter, SamplerMinFilter, SamplerWrap, StencilOp, StencilOps,
    StencilTest, TextureError, VertexArrayError,
};
pub use texture::{ColorSampler2d, ColorTexture2d, ComparisonSampler2d, DepthTexture2d};
pub use uniform_buffer::{UniformBuffer, UniformBufferBinding};
pub use vec::{BVec2, BVec3, BVec4, IVec2, IVec3, IVec4, UVec2, UVec3, UVec4, Vec2, Vec3, Vec4};
pub use vertex_buffer::{VertexBuffer, VertexBufferBinding};
pub use vertex_spec::VertexSpec;

#[derive(Clone, Copy, Zeroable, Pod, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
#[repr(transparent)]
pub struct Bool(u32);

impl AsStd140 for Bool {
    type Output = crevice::std140::Bool;

    fn as_std140(&self) -> Self::Output {
        (*self).into()
    }

    fn from_std140(value: Self::Output) -> Self {
        value.into()
    }
}

impl Default for Bool {
    fn default() -> Self {
        false.into()
    }
}

impl ToSl for Bool {
    type Output = sl::Bool;

    fn to_sl(self) -> Self::Output {
        bool::from(self).to_sl()
    }
}

impl From<bool> for Bool {
    fn from(value: bool) -> Self {
        Bool(value as u32)
    }
}

impl From<Bool> for bool {
    fn from(value: Bool) -> Self {
        value.0 != 0
    }
}

impl From<crevice::std140::Bool> for Bool {
    fn from(value: crevice::std140::Bool) -> Self {
        bool::from(value).into()
    }
}

impl From<Bool> for crevice::std140::Bool {
    fn from(value: Bool) -> Self {
        bool::from(value).into()
    }
}
