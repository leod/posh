use glow::HasContext;

#[derive(Debug, Copy, Clone)]
pub struct Caps {
    pub max_texture_size: u32,
}

impl Caps {
    pub fn new(gl: &glow::Context) -> Self {
        let max_texture_size = unsafe { gl.get_parameter_i32(glow::MAX_TEXTURE_SIZE) };

        Caps {
            max_texture_size: u32::try_from(max_texture_size).unwrap(),
        }
    }
}
