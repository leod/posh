use std::marker::PhantomData;

use crate::{
    sl::{self, FragmentInput, FragmentOutput, Varying, VertexInput, VertexOutput},
    FragmentInterface, ResourceInterface, Sl, VertexInterface,
};

use super::{
    untyped::{self, UniformBlockInfo},
    Context, CreateProgramError, DrawParams, GeometryStream, Surface,
};

pub struct Program<R, A, F> {
    _phantom: PhantomData<(R, A, F)>,
}

impl<R, V, F> Program<R, V, F>
where
    R: ResourceInterface<Sl, InSl = R>,
    V: VertexInterface<Sl, InSl = V>,
    F: FragmentInterface<Sl, InSl = F>,
{
    pub(crate) fn new<W>(
        context: &Context,
        vertex_shader: fn(R, VertexInput<V>) -> VertexOutput<W>,
        fragment_shader: fn(R, FragmentInput<W>) -> FragmentOutput<F>,
    ) -> Result<Self, CreateProgramError>
    where
        W: Varying,
    {
        let typed_program_def = sl::ProgramDef::new(vertex_shader, fragment_shader);
        let untpyed_program_def = untyped::ProgramDef {
            uniform_block_infos: typed_program_def
                .uniform_block_defs
                .into_iter()
                .map(|def| UniformBlockInfo {
                    name: def.block_name,
                    location: def.location,
                })
                .collect(),
            sampler_infos: Vec::new(), // TODO
            vertex_infos: typed_program_def.vertex_infos,
            vertex_shader_source: typed_program_def.vertex_shader_source,
            fragment_shader_source: typed_program_def.fragment_shader_source,
        };

        Ok(Program {
            _phantom: PhantomData,
        })
    }

    pub fn draw<S>(
        &self,
        resource: R::InGl,
        geometry: GeometryStream<V>,
        surface: &S,
        draw_params: &DrawParams,
    ) where
        S: Surface<F>,
    {
    }
}
