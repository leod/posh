use crate::Attributes;

use crate::Gl;

use super::{Element, ElementBufferBinding};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Primitive {
    Points,
    Lines,
    LineStrip,
    LineLoop,
    Triangles,
    TriangleStrip,
    TriangleFan,
}

#[derive(Clone)]
pub struct VertexStream<A: Attributes<Gl>, E: Element> {
    pub attribute: A,
    pub primitive: Primitive,
    pub elements: Option<ElementBufferBinding<E>>,
}
