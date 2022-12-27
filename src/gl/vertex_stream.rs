use crate::Vertex;

use crate::Gl;

use super::{Element, ElementBufferBinding};

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

#[derive(Clone)]
pub struct VertexStream<A: Vertex<Gl>, E: Element> {
    pub attribute: A,
    pub geometry_type: GeometryType,
    pub elements: Option<ElementBufferBinding<E>>,
}
