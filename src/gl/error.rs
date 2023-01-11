use thiserror::Error;

#[derive(Debug, Clone, Error)]
#[error("failed to create buffer: {0}")]
pub struct CreateBufferError(pub String);

#[derive(Debug, Clone, Error)]
#[error("failed to create vertex array: {0}")]
pub struct CreateVertexArrayError(pub String);

#[derive(Debug, Clone, Error)]
pub enum CreateProgramError {
    #[error("failed to create shader: {0}")]
    CreateShader(String),

    #[error("failed to create program: {0}")]
    CreateProgram(String),

    #[error("failed to compile program:\nvertex shader: {vertex_shader_info}\nfragment shader: {fragment_shader_info}\nprogram: {program_info}")]
    CompilationError {
        vertex_shader_info: String,
        fragment_shader_info: String,
        program_info: String,
    },
}

#[derive(Debug, Clone, Error)]
pub enum CreateError {
    #[error("failed to create buffer: {0}")]
    CreateBuffer(#[from] CreateBufferError),

    #[error("failed to create vertex array: {0}")]
    CreateVertexArray(#[from] CreateVertexArrayError),

    #[error("failed to create program: {0}")]
    CreateProgram(#[from] CreateProgramError),
}
