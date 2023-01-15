use std::rc::Rc;

use super::untyped;

pub struct GeometryStream<V> {
    pub(crate) untyped: untyped::GeometryStream,
    pub(crate) _vertex_buffers: Rc<V>,
}
