use glow::HasContext;
use thiserror::Error;

pub(super) fn check_gl_error(gl: &glow::Context) -> Result<(), String> {
    let error_info = unsafe { gl.get_error() };

    if error_info == glow::NO_ERROR {
        Ok(())
    } else {
        let s = match error_info {
            glow::INVALID_ENUM => "INVALID_ENUM".to_string(),
            glow::INVALID_VALUE => "INVALID_VALUE".to_string(),
            glow::INVALID_OPERATION => "INVALID_OPERATION".to_string(),
            glow::INVALID_FRAMEBUFFER_OPERATION => "INVALID_FRAMEBUFFER_OPERATION".to_string(),
            glow::OUT_OF_MEMORY => {
                // Results of OpenGL operations are undefined if it ran out of
                // memory. I'm not sure if we can do better than panicking here.
                panic!("OpenGL ran out of memory");
            }
            _ => format!("unknown OpenGL error: {error_info}"),
        };

        Err(s)
    }
}

/// An error that occurred while creating a buffer.
#[derive(Debug, Clone, Error)]
pub enum BufferError {
    #[error("could not create buffer object: {0}")]
    ObjectCreation(String),

    #[error("unexpected error: {0}")]
    Unexpected(String),
}

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

    #[error("unexpected error: {0}")]
    Unexpected(String),
}

/// An error that occurred while creating a vertex array.
#[derive(Debug, Clone, Error)]
pub enum VertexArrayError {
    #[error("could not create vertex array object: {0}")]
    ObjectCreation(String),

    #[error("unexpected error: {0}")]
    Unexpected(String),
}

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

    #[error("unexpected error: {0}")]
    Unexpected(String),
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
