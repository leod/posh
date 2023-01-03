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

#[derive(Debug, Copy, Clone)]
pub enum ElementType {
    U16,
    U32,
}

impl ElementType {
    pub fn to_gl(self) -> u32 {
        use ElementType::*;

        match self {
            U16 => glow::UNSIGNED_SHORT,
            U32 => glow::UNSIGNED_INT,
        }
    }

    pub fn size(self) -> usize {
        use ElementType::*;

        match self {
            U16 => 2,
            U32 => 4,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum GeometryType {
    Points,
    Lines,
    LineStrip,
    LineLoop,
    Triangles,
    TriangleStrip,
    TriangleFan,
}

impl GeometryType {
    pub fn to_gl(self) -> u32 {
        use GeometryType::*;

        match self {
            Points => glow::POINTS,
            Lines => glow::LINES,
            LineStrip => glow::LINE_STRIP,
            LineLoop => glow::LINE_LOOP,
            Triangles => glow::TRIANGLES,
            TriangleStrip => glow::TRIANGLE_STRIP,
            TriangleFan => glow::TRIANGLE_FAN,
        }
    }
}
