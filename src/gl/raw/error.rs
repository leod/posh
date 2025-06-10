use glow::HasContext;
use thiserror::Error;

pub(super) fn check_gl_error(gl: &glow::Context, name: &str) -> Result<(), String> {
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

        Err(format!("{s} ({name})"))
    }
}

/// An error that occurred while creating a context.
#[derive(Debug, Clone, Error)]
pub enum ContextError {
    #[error("could not create main vertex array object: {0}")]
    ObjectCreation(String),

    #[error("unexpected error while creating context: {0}")]
    Unexpected(String),
}

/// An error that occurred while creating a buffer.
#[derive(Debug, Clone, Error)]
pub enum BufferError {
    #[error("could not create buffer object: {0}")]
    ObjectCreation(String),

    #[error("unexpected error while creating buffer: {0}")]
    Unexpected(String),
}

/// An error that occurred while creating a texture.
#[derive(Debug, Clone, Error)]
pub enum TextureError {
    #[error("could not create texture object: {0}")]
    ObjectCreation(String),

    #[error("texture is empty")]
    Empty,

    #[error("texture too large: requested {requested}, but the maximum size is {max}")]
    Oversized { requested: u32, max: u32 },

    #[error("invalid data size: expected {expected} bytes, but got {got}")]
    DataSizeMismatch { expected: usize, got: usize },

    #[error("unexpected error while creating texture: {0}")]
    Unexpected(String),
}

/// An error that occurred while creating a vertex array.
#[derive(Debug, Clone, Error)]
pub enum VertexArrayError {
    #[error("could not create vertex array object: {0}")]
    ObjectCreation(String),

    #[error("invalid vertex attribute: {0}")]
    InvalidVertexAttribute(String),

    #[error("unexpected error while creating vertex array: {0}")]
    Unexpected(String),
}

/// An error that occurred while creating a query.
#[derive(Debug, Clone, Error)]
pub enum QueryError {
    #[error("could not create query object: {0}")]
    ObjectCreation(String),

    #[error("unsupported")]
    Unsupported,

    #[error("unexpected error while creating query: {0}")]
    Unexpected(String),
}

/// An error that was detected by framebuffer completeness checks.
#[derive(Debug, Clone, Error)]
pub enum FramebufferIncompleteError {
    #[error("the default framebuffer does not exist")]
    Undefined,

    #[error("an attachment is incomplete")]
    IncompleteAttachment,

    #[error("there needs to be at least one attachment")]
    IncompleteMissingAttachment,

    #[error("unsupported (e.g. using different images for depth and stencil)")]
    Unsupported,

    #[error("incomplete multisample")]
    IncompleteMultisample,

    #[error("general OpenGL error")]
    Error(String),

    #[error("unknown error: {0}")]
    Unknown(u32),
}

pub(super) fn check_framebuffer_completeness(
    gl: &glow::Context,
) -> Result<(), FramebufferIncompleteError> {
    let status = unsafe { gl.check_framebuffer_status(glow::FRAMEBUFFER) };

    use FramebufferIncompleteError::*;

    match status {
        glow::FRAMEBUFFER_COMPLETE => Ok(()),
        glow::FRAMEBUFFER_UNDEFINED => Err(Undefined),
        glow::FRAMEBUFFER_INCOMPLETE_ATTACHMENT => Err(IncompleteAttachment),
        glow::FRAMEBUFFER_INCOMPLETE_MISSING_ATTACHMENT => Err(IncompleteMissingAttachment),
        glow::FRAMEBUFFER_UNSUPPORTED => Err(Unsupported),
        glow::FRAMEBUFFER_INCOMPLETE_MULTISAMPLE => Err(IncompleteMultisample),

        // OpenGL ES 3.0.6: 4.4.4.2 Whole Framebuffer Completeness
        // > If *CheckFramebufferStatus* generates an error, zero is returned.
        0 => check_gl_error(gl, "framebuffer completeness status").map_err(Error),

        // This should not be reachable.
        error => Err(Unknown(error)),
    }
}

/// An error that occurred while creating a framebuffer.
#[derive(Debug, Clone, Error)]
pub enum FramebufferError {
    #[error("could not create framebuffer object: {0}")]
    ObjectCreation(String),

    #[error("texture level is too large: requested {requested}, but the maximum level is {max}")]
    LevelTooLarge { requested: u32, max: u32 },

    #[error("too many color attachments: requested {requested}, but the maximum number of color attachments is {max}")]
    TooManyColorAttachments { requested: u32, max: u32 },

    #[error("too many draw buffers: requested {requested}, but the maximum number of draw buffers is {max}")]
    TooManyDrawBuffers { requested: u32, max: u32 },

    #[error("too many depth attachments: requested {requested}, but the maximum number of depth attachments is 1")]
    TooManyDepthAttachments { requested: u32 },

    #[error("too many stencil attachments: requested {requested}, but the maximum number of stencil attachments is 1")]
    TooManyStencilAttachments { requested: u32 },

    #[error("framebuffer is incomplete unexpectedly: {0}")]
    Incomplete(FramebufferIncompleteError),

    #[error("unexpected error while creating framebuffer: {0}")]
    Unexpected(String),
}

/// An error that was found while validating a program.
#[derive(Debug, Clone, Error)]
pub enum ProgramValidationError {
    #[error("duplicate sampler name: {0}")]
    DuplicateSampler(String),

    #[error("duplicate sampler texture unit: {0}")]
    DuplicateSamplerTextureUnit(usize),

    #[error("duplicate uniform block name: {0}")]
    DuplicateUniformBlock(String),

    #[error("duplicate uniform block location: {0}")]
    DuplicateUniformBlockLocation(usize),
}

/// An error that occurred while creating a program.
#[derive(Debug, Clone, Error)]
pub enum ProgramError {
    #[error("invalid program definition: {0}")]
    Validation(#[from] ProgramValidationError),

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

    #[error("invalid vertex attribute: {0}")]
    InvalidVertexAttribute(String),

    #[error("unexpected error while creating program: {0}")]
    Unexpected(String),
}

/// An error that occurred while creating an object.
#[derive(Debug, Clone, Error)]
pub enum CreateError {
    #[error("buffer error: {0}")]
    Buffer(#[from] BufferError),

    #[error("program error: {0}")]
    Program(#[from] ProgramError),

    #[error("texture error: {0}")]
    Texture(#[from] TextureError),

    #[error("framebuffer error: {0}")]
    Framebuffer(#[from] FramebufferError),

    #[error("vertex array error: {0}")]
    VertexArray(#[from] VertexArrayError),
}

/// An error that occurred while drawing.
#[derive(Debug, Clone, Error)]
pub enum DrawError {
    #[error("create error: {0}")]
    Create(#[from] CreateError),

    #[error("general OpenGL error: {0}")]
    Error(String),
}

impl From<BufferError> for DrawError {
    fn from(value: BufferError) -> Self {
        Self::Create(value.into())
    }
}

impl From<ProgramError> for DrawError {
    fn from(value: ProgramError) -> Self {
        Self::Create(value.into())
    }
}

impl From<TextureError> for DrawError {
    fn from(value: TextureError) -> Self {
        Self::Create(value.into())
    }
}

impl From<FramebufferError> for DrawError {
    fn from(value: FramebufferError) -> Self {
        Self::Create(value.into())
    }
}

impl From<VertexArrayError> for DrawError {
    fn from(value: VertexArrayError) -> Self {
        Self::Create(value.into())
    }
}
