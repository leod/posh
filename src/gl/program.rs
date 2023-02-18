use std::{marker::PhantomData, rc::Rc};

use crate::{
    interface::UniformDataVisitor, sl, Block, FragmentData, Logical, Physical, UniformData,
    VertexData,
};

use super::{raw, DrawParams, Sampler2d, Surface, UniformBufferBinding, VertexArrayBinding};

#[derive(Clone)]
pub struct Program<UData, VData, FData = sl::Vec4> {
    raw: Rc<raw::Program>,
    _phantom: PhantomData<(UData, VData, FData)>,
}

impl<UData, VData, FData> Program<UData, VData, FData>
where
    UData: UniformData<Logical>,
    VData: VertexData<Logical>,
    FData: FragmentData<Logical>,
{
    pub(super) fn unchecked_from_raw(raw: raw::Program) -> Self {
        Program {
            raw: Rc::new(raw),
            _phantom: PhantomData,
        }
    }

    pub fn draw<S>(
        &self,
        uniforms: UData::Physical,
        vertices: VertexArrayBinding<VData::Physical>,
        surface: &S,
        draw_params: &DrawParams,
    ) where
        S: Surface<FData>,
    {
        // TODO: Surface stuff.

        // TODO: These allocations can be avoided once stable has allocators.
        let mut uniform_visitor = UniformVisitor::default();
        uniforms.visit("", &mut uniform_visitor);

        // FIXME: Safety: check that all vertex buffers are large enough for the
        // values in the element buffer (if we have one).

        unsafe {
            self.raw.draw(
                &uniform_visitor.raw_uniform_buffers,
                &uniform_visitor.raw_samplers,
                vertices.raw,
            );
        }
    }
}

#[derive(Default)]
struct UniformVisitor<'a> {
    raw_uniform_buffers: Vec<&'a raw::Buffer>,
    raw_samplers: Vec<raw::Sampler>,
}

impl<'a> UniformDataVisitor<'a, Physical> for UniformVisitor<'a> {
    fn accept_sampler2d(&mut self, path: &str, sampler: &Sampler2d) {
        self.raw_samplers
            .push(raw::Sampler::Sampler2d(sampler.raw.clone()))
    }

    fn accept_block<B: Block<Logical, Logical = B>>(
        &mut self,
        _: &str,
        uniform: &'a UniformBufferBinding<B>,
    ) {
        self.raw_uniform_buffers.push(&uniform.raw);
    }
}
