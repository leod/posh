use glow::HasContext;

#[derive(Debug, Copy, Clone)]
pub struct Caps {
    pub max_texture_size: u32,
    pub max_color_attachments: u32,
    pub max_draw_buffers: u32,
    pub disjoint_timer_query_webgl2: bool,
}

impl Caps {
    pub fn new(gl: &glow::Context) -> Self {
        let max_texture_size = unsafe { gl.get_parameter_i32(glow::MAX_TEXTURE_SIZE) };
        let max_color_attachments = unsafe { gl.get_parameter_i32(glow::MAX_COLOR_ATTACHMENTS) };
        let max_draw_buffers = unsafe { gl.get_parameter_i32(glow::MAX_DRAW_BUFFERS) };

        assert!(max_texture_size > 0);
        assert!(max_color_attachments > 0);
        assert!(max_draw_buffers > 0);

        Caps {
            max_texture_size: max_texture_size.try_into().unwrap(),
            max_color_attachments: max_color_attachments.try_into().unwrap(),
            max_draw_buffers: max_draw_buffers.try_into().unwrap(),
            disjoint_timer_query_webgl2: gl
                .supported_extensions()
                .contains("EXT_disjoint_timer_query_webgl2"),
        }
    }
}
