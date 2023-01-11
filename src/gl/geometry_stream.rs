use std::marker::PhantomData;

use super::untyped;

pub struct GeometryStream<V> {
    pub(crate) untyped: untyped::GeometryStream,
    // FIXME: Store ref to element buffer range and check before drawing.
    pub(crate) _phantom: PhantomData<V>,
}
