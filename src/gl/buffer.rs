#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BufferUsage {
    StreamDraw,
    StreamRead,
    StaticDraw,
    StaticRead,
    DynamicDraw,
    DynamicRead,
}

impl BufferUsage {
    pub fn to_gl(self) -> u32 {
        use BufferUsage::*;
        match self {
            StreamDraw => glow::STREAM_DRAW,
            StreamRead => glow::STREAM_READ,
            StaticDraw => glow::STATIC_DRAW,
            StaticRead => glow::STATIC_READ,
            DynamicDraw => glow::DYNAMIC_DRAW,
            DynamicRead => glow::DYNAMIC_READ,
        }
    }
}
