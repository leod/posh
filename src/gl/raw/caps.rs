use glow::HasContext;

#[derive(Debug, Copy, Clone)]
pub struct Caps {
    pub max_texture_size: u32,
    pub max_color_attachments: u32,
}

impl Caps {
    pub fn new(gl: &glow::Context) -> Self {
        let max_texture_size = unsafe { gl.get_parameter_i32(glow::MAX_TEXTURE_SIZE) };
        let max_color_attachments = unsafe { gl.get_parameter_i32(glow::MAX_COLOR_ATTACHMENTS) };

        assert!(max_texture_size > 0);
        assert!(max_color_attachments > 0);

        Caps {
            max_texture_size: u32::try_from(max_texture_size).unwrap(),
            max_color_attachments: u32::try_from(max_color_attachments).unwrap(),
        }
    }
}
