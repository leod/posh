use std::marker::PhantomData;

use crate::{Sl, VertexInterface};

use super::untyped;

pub struct GeometryStream<V: VertexInterface<Sl>> {
    pub(crate) untyped: untyped::GeometryStream,
    pub(crate) _phantom: PhantomData<V>,
}
