use std::marker::PhantomData;

use crate::{sl::Varying, Fragment, Gl, Resource, Sl, Vertex};

use super::{DrawParams, Element, SurfaceBinding, VertexStream};

pub struct Program<R, A, F> {
    _phantom: PhantomData<(R, A, F)>,
}

impl<R, A, F> Program<R, A, F>
where
    R: Resource<Sl>,
    A: Vertex<Sl>,
    F: Fragment<Sl>,
{
    pub fn new<V>(vertex_shader: fn(R, A) -> V, fragment_shader: fn(R, V) -> F) -> Self
    where
        V: Varying,
    {
        Program {
            _phantom: PhantomData,
        }
    }

    pub fn draw<BindR, BindA, BindF, E>(
        &self,
        resource: BindR,
        vertices: VertexStream<BindA, E>,
        surface: SurfaceBinding<BindF>,
        draw_params: &DrawParams,
    ) where
        BindR: Resource<Gl, InSl = R>,
        BindA: Vertex<Gl, InSl = A>,
        BindF: Fragment<Gl, InSl = F>,
        E: Element,
    {
    }
}
