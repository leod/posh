use std::marker::PhantomData;

use crate::{Attributes, Gl};

pub struct VertexArray<V: Attributes<Gl>> {
    _phantom: PhantomData<V>,
}
