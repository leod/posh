use std::marker::PhantomData;

use crate::{sl::Varying, FragmentInterface, ResourceInterface, Sl, VertexInterface};

use super::{DrawParams, ElementSource, GeometryType, SurfaceBinding, VertexBinding};

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

    pub fn draw<E>(
        &self,
        resource: R::InGl,
        vertices: VertexBinding<V, E>,
        geometry_type: GeometryType,
        surface: SurfaceBinding<F>,
        draw_params: &DrawParams,
    ) where
        E: ElementSource,
    {
    }
}
