use std::marker::PhantomData;

use crate::{Fragment, Resource, Sl, Vertex};

use super::Varying;

pub struct ProgramDef<R, A, F> {
    _phantom: PhantomData<(R, A, F)>,
}

impl<R, A, F> ProgramDef<R, A, F>
where
    R: Resource<Sl>,
    A: Vertex<Sl>,
    F: Fragment<Sl>,
{
    pub fn new<V>(vertex_shader: fn(R, A) -> V, fragment_shader: fn(R, V) -> F) -> Self
    where
        V: Varying,
    {
        Self {
            _phantom: PhantomData,
        }
    }
}
