use std::marker::PhantomData;

use crate::{Attributes, Sl};

pub struct VertexArray<V: Attributes<Sl>> {
    _phantom: PhantomData<V>,
}
