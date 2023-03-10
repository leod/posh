use glow::HasContext;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ComparisonFunc {
    Always,
    Equal,
    Greater,
    GreaterOrEqual,
    Less,
    LessOrEqual,
    Never,
    NotEqual,
}

impl ComparisonFunc {
    pub const fn to_gl(self) -> u32 {
        use ComparisonFunc::*;

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

#[derive(Debug, Clone, PartialEq)]
pub struct DrawParams {
    depth_test: Option<ComparisonFunc>,
}

impl Default for DrawParams {
    fn default() -> Self {
        Self { depth_test: None }
    }
}

impl DrawParams {
    pub(super) fn set_delta(&self, gl: &glow::Context, current: &DrawParams) {
        if self.depth_test != current.depth_test {
            if let Some(func) = self.depth_test {
                let func = func.to_gl();

                unsafe {
                    gl.enable(glow::DEPTH_TEST);
                    gl.depth_func(func);
                }
            } else {
                unsafe {
                    gl.disable(glow::DEPTH_TEST);
                }
            }
        }
    }
}
