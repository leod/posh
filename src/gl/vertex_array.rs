use std::marker::PhantomData;

use crate::{Sl, Vertex};

pub struct VertexArray<V: Vertex<Sl>> {
    _phantom: PhantomData<V>,
}
