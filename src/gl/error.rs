use thiserror::Error;

/// An error that occurred while creating a buffer.
#[derive(Debug, Clone, Error)]
#[error("failed to create buffer: {0}")]
pub struct BufferError(pub String);

/// An error that occurred while creating a texture.
#[derive(Debug, Clone, Error)]
pub enum TextureError {
    #[error("failed to create texture: {0}")]
    Create(String),

    #[error("texture too large: requested {0}, but max size is {1}")]
    TooLarge(usize, usize),
}

/// An error that occurred while creating a vertex array.
#[derive(Debug, Clone, Error)]
#[error("failed to create vertex array: {0}")]
pub struct VertexArrayError(pub String);

/// An error that occurred while creating a program.
#[derive(Debug, Clone, Error)]
pub enum ProgramError {
    #[error("failed to create shader: {0}")]
    CreateShader(String),

    #[error("failed to create program: {0}")]
    CreateProgram(String),

    #[error("failed to compile program:\nvertex shader: {vertex_shader_info}\nfragment shader: {fragment_shader_info}\nprogram: {program_info}")]
    CompilerError {
        vertex_shader_info: String,
        fragment_shader_info: String,
        program_info: String,
    },
}

/// An error that occured while creating a object.
#[derive(Debug, Clone, Error)]
pub enum Error {
    #[error("{0}")]
    CreateBuffer(#[from] BufferError),

    #[error("{0}")]
    CreateProgram(#[from] ProgramError),

    #[error("{0}")]
    CreateTexture(#[from] TextureError),

    #[error("{0}")]
    CreateVertexArray(#[from] VertexArrayError),
}
