use std::marker::PhantomData;

use crate::{sl::Varying, FragmentInterface, Gl, ResourceInterface, Sl, VertexInterface};

use super::{DrawParams, Element, SurfaceBinding, VertexStream};

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

    pub fn draw<BindR, BindV, BindF, E>(
        &self,
        resource: BindR,
        vertices: VertexStream<BindV, E>,
        surface: SurfaceBinding<BindF>,
        draw_params: &DrawParams,
    ) where
        BindR: ResourceInterface<Gl, InSl = R>,
        BindV: VertexInterface<Gl, InSl = V>,
        BindF: FragmentInterface<Gl, InSl = F>,
        E: Element,
    {
    }
}
