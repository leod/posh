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

pub struct DrawParams {
    depth_test: Option<ComparisonFunc>,
}

impl Default for DrawParams {
    fn default() -> Self {
        Self { depth_test: None }
    }
}
