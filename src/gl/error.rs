#[derive(Debug, Clone)]
pub struct CreateBufferError(pub String);

#[derive(Debug, Clone)]
pub struct CreateVertexArrayError(pub String);

#[derive(Debug, Clone)]
pub enum CreateShaderError {
    CreateShader(String),
    CreateProgram(String),
    CompilationError {
        vertex_shader_info: String,
        fragment_shader_info: String,
        program_info: String,
    },
}
