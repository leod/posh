use std::{marker::PhantomData, rc::Rc};

use crate::{Sl, VertexInterface};

pub struct VertexData<V: VertexInterface<Sl>> {
    _phantom: PhantomData<V>,
}

impl<V: VertexInterface<Sl>> VertexData<V> {
    pub fn new(gl: Rc<glow::Context>) {}
}
