use crate::Attributes;

use crate::Gl;

use super::{Element, ElementBuffer, VertexArray};

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
pub struct VertexStream<'a, A: Attributes<Gl>, E: Element> {
    pub vertices: &'a VertexArray<A>,
    pub primitive: Primitive,
    pub elements: Option<&'a ElementBuffer<E>>,
}
