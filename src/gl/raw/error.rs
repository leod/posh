use thiserror::Error;

/// An error that occurred while creating a buffer.
#[derive(Debug, Clone, Error)]
#[error("failed to create buffer: {0}")]
pub struct BufferError(pub String);

/// An error that occurred while creating a texture.
#[derive(Debug, Clone, Error)]
pub enum TextureError {
    #[error("could not create texture object: {0}")]
    ObjectCreation(String),

    #[error("texture is empty")]
    Empty,

    #[error("texture too large: requested {0}, but max size is {1}")]
    Oversized(usize, usize),

    #[error("invalid data size: expected {0} bytes, but got {1}")]
    DataSizeMismatch(usize, usize),
}

/// An error that occurred while creating a vertex array.
#[derive(Debug, Clone, Error)]
#[error("failed to create vertex array: {0}")]
pub struct VertexArrayError(pub String);

/// An error that occurred while creating a program.
#[derive(Debug, Clone, Error)]
pub enum ProgramError {
    #[error("failed to create shader object: {0}")]
    ShaderCreation(String),

    #[error("failed to create program: {0}")]
    ProgramCreation(String),

    #[error("failed to compile program:\nvertex shader: {vertex_shader_info}\nfragment shader: {fragment_shader_info}\nprogram: {program_info}")]
    Compiler {
        vertex_shader_info: String,
        fragment_shader_info: String,
        program_info: String,
    },
}

/// An error that occured while creating a object.
#[derive(Debug, Clone, Error)]
pub enum Error {
    #[error("{0}")]
    Buffer(#[from] BufferError),

    #[error("{0}")]
    Program(#[from] ProgramError),

    #[error("{0}")]
    Texture(#[from] TextureError),

    #[error("{0}")]
    VertexArray(#[from] VertexArrayError),
}
