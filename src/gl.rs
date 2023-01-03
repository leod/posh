mod context;
mod draw_params;
mod element_buffer;
mod enums;
mod error;
mod geometry_stream;
mod program;
mod sampler;
mod surface;
mod texture;
mod uniform_buffer;
mod vertex_array;
mod vertex_buffer;

// TODO: Make `untyped` private. Figure out escape hatches later. Only exposing
// this for tests right now.
#[doc(hidden)]
pub mod untyped;

pub use context::Context;
pub use draw_params::DrawParams;
pub use element_buffer::{Element, ElementBuffer, ElementSource};
pub use enums::{BufferUsage, ElementType, GeometryType};
pub use error::{CreateBufferError, CreateProgramError, CreateVertexArrayError};
pub use geometry_stream::GeometryStream;
pub use program::Program;
pub use sampler::{Sampler2d, Sampler2dBinding};
pub use surface::{DefaultFramebuffer, Surface};
pub use texture::Texture2dBinding;
pub use uniform_buffer::{UniformBuffer, UniformBufferBinding};
pub use vertex_array::VertexArray;
pub use vertex_buffer::VertexBuffer;
