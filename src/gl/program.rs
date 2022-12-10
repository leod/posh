use std::marker::PhantomData;

/*
use posh::{
    shader::{FragmentOutput, ResourceInput, VertexInput},
    Gpu,
};

use crate::{Bind, DrawParams, Element, Surface, VertexStream};
*/

pub struct Program<R, V, F> {
    _phantom: PhantomData<(R, V, F)>,
}

/*
impl<R, V, F> Program<R, V, F>
where
    R: ResourceInput<Space = Gpu>,
    V: VertexInput<Gpu>,
    F: FragmentOutput<Space = Gpu>,
{
    pub fn draw<'a, BindR, BindV, BindF>(
        &self,
        resource: BindR,
        vertices: VertexStream<BindV, impl Element>,
        surface: &impl Surface<BindF>,
        draw_params: &DrawParams,
    ) where
        BindR: ResourceInput<Space = Bind<'a>, InPosh = R>,
        BindV: VertexInput<Bind<'a>, InPosh = V>,
        BindF: FragmentOutput<Posh = F>,
    {
    }
}
*/
