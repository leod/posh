use std::marker::PhantomData;

use crate::{Sl, VertexInterface};

pub struct VertexData<V: VertexInterface<Sl>> {
    _phantom: PhantomData<V>,
}
