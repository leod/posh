use std::rc::Rc;

use super::untyped;

/// A stream of vertices together with a geometry type.
///
/// A geometry stream provides vertex data for [draw
/// calls](crate::gl::Program::draw). Geometry streams can be obtained with
/// [`VertexArray::stream`](crate::gl::VertexArray::stream) or
/// [`VertexArray::stream_range`](crate::gl::VertexArray::stream_range).
pub struct GeometryStream<V> {
    pub(crate) untyped: untyped::GeometryStream,
    pub(crate) _vertex_buffers: Rc<V>,
}
