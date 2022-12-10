use std::marker::PhantomData;

use crate::{Gl, Vertex};

pub struct VertexBuffer<V: Vertex<Gl>> {
    _phantom: PhantomData<V>,
}

pub struct VertexBufferBinding<V: Vertex<Gl>> {
    _phantom: PhantomData<V>,
}
