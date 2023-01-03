use std::marker::PhantomData;

use crate::{sl::Varying, FragmentInterface, ResourceInterface, Sl, VertexInterface};

use super::{DrawParams, ElementSource, GeometryType, Surface, VertexArray};

pub struct Program<R, A, F> {
    _phantom: PhantomData<(R, A, F)>,
}

impl<R, V, F> Program<R, V, F>
where
    R: ResourceInterface<Sl>,
    V: VertexInterface<Sl>,
    F: FragmentInterface<Sl>,
{
    pub fn new<W>(vertex_shader: fn(R, V) -> W, fragment_shader: fn(R, W) -> F) -> Self
    where
        W: Varying,
    {
        Program {
            _phantom: PhantomData,
        }
    }

    pub fn draw<E, S>(
        &self,
        resource: R::InGl,
        vertices: &VertexArray<V, E>,
        geometry_type: GeometryType,
        surface: &S,
        draw_params: &DrawParams,
    ) where
        E: ElementSource,
        S: Surface<F>,
    {
    }
}
