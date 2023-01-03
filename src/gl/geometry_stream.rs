use std::marker::PhantomData;

use super::untyped;

pub struct GeometryStream<V> {
    pub(crate) untyped: untyped::GeometryStream,
    pub(crate) _phantom: PhantomData<V>,
}
