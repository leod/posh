use std::marker::PhantomData;

use crate::{Gl, Sl, Vertex};

pub struct VertexBuffer<V: Vertex<Sl>> {
    data: Vec<V::InGl>,
}

#[derive(Clone)]
pub struct VertexBufferBinding<V: Vertex<Gl>> {
    _phantom: PhantomData<V>,
}
