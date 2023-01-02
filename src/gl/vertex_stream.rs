use crate::{Sl, VertexInterface};

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
pub struct VertexStream<V: VertexInterface<Sl>, E: Element> {
    pub vertices: V,
    pub geometry_type: GeometryType,
    pub elements: Option<ElementBufferBinding<E>>,
}
