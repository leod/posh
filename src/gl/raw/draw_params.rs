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

#[derive(Debug, Clone, PartialEq)]
pub struct DrawParams {
    pub depth_compare: Option<CompareFunction>,
}

impl Default for DrawParams {
    fn default() -> Self {
        Self {
            depth_compare: None,
        }
    }
}

impl DrawParams {
    pub(super) fn set_delta(&self, gl: &glow::Context, current: &DrawParams) {
        if self.depth_compare != current.depth_compare {
            if let Some(func) = self.depth_compare {
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

    pub fn with_depth_compare(mut self, func: CompareFunction) -> Self {
        self.depth_compare = Some(func);
        self
    }
}
