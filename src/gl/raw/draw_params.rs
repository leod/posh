use glow::HasContext;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum CompareFunction {
    Always,
    Equal,
    Greater,
    GreaterOrEqual,
    Less,
    LessOrEqual,
    Never,
    NotEqual,
}

impl CompareFunction {
    pub const fn to_gl(self) -> u32 {
        use CompareFunction::*;

        match self {
            Always => glow::ALWAYS,
            Equal => glow::EQUAL,
            Greater => glow::GREATER,
            GreaterOrEqual => glow::GEQUAL,
            Less => glow::LESS,
            LessOrEqual => glow::LEQUAL,
            Never => glow::NEVER,
            NotEqual => glow::NOTEQUAL,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct DrawParams {
    pub clear_color: Option<glam::Vec4>,
    pub clear_depth: Option<f32>,
    pub clear_stencil: Option<u8>,
    pub depth_compare: Option<CompareFunction>,
}

impl Default for DrawParams {
    fn default() -> Self {
        Self {
            clear_color: None,
            clear_depth: None,
            clear_stencil: None,
            depth_compare: None,
        }
    }
}

impl DrawParams {
    pub(super) fn set_delta(&self, gl: &glow::Context, current: &DrawParams) {
        let mut clear_mask = 0;

        if let Some(c) = self.clear_color {
            unsafe { gl.clear_color(c.x, c.y, c.z, c.w) };

            clear_mask |= glow::COLOR_BUFFER_BIT;
        }

        if let Some(c) = self.clear_depth {
            unsafe { gl.clear_depth_f32(c) };

            clear_mask |= glow::DEPTH_BUFFER_BIT;
        }

        if let Some(c) = self.clear_stencil {
            unsafe { gl.clear_stencil(c as i32) };

            clear_mask |= glow::STENCIL_BUFFER_BIT;
        }

        if clear_mask > 0 {
            unsafe { gl.clear(clear_mask) };
        }

        if self.depth_compare != current.depth_compare {
            if let Some(func) = self.depth_compare {
                let func = func.to_gl();

                unsafe { gl.enable(glow::DEPTH_TEST) };
                unsafe { gl.depth_func(func) };
            } else {
                unsafe { gl.disable(glow::DEPTH_TEST) };
            }
        }
    }

    pub fn with_depth_compare(mut self, depth_compare: CompareFunction) -> Self {
        self.depth_compare = Some(depth_compare);
        self
    }

    pub fn with_clear_color(mut self, clear_color: glam::Vec4) -> Self {
        self.clear_color = Some(clear_color);
        self
    }

    pub fn with_clear_depth(mut self, clear_depth: f32) -> Self {
        self.clear_depth = Some(clear_depth);
        self
    }

    pub fn with_clear_stencil(mut self, clear_stencil: u8) -> Self {
        self.clear_stencil = Some(clear_stencil);
        self
    }
}
