use glow::HasContext;

#[derive(Debug, Copy, Clone)]
pub struct Capabilities {
    pub max_texture_size: usize,
}

impl Capabilities {
    pub fn new(gl: &glow::Context) -> Self {
        let max_texture_size = unsafe { gl.get_parameter_i32(glow::MAX_TEXTURE_SIZE) };

        Capabilities {
            max_texture_size: usize::try_from(max_texture_size).unwrap(),
        }
    }
}
