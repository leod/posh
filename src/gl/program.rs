use std::{marker::PhantomData, rc::Rc};

use crate::{
    interface::UniformVisitor,
    sl::{self, ColorSample},
    Block, Fragment, Gl, Sl, Uniform, Vertex,
};

use super::{
    raw, ColorSampler2d, ComparisonSampler2d, DrawError, DrawSettings, Framebuffer,
    UniformBufferBinding, VertexSpec,
};

pub struct DrawInput<'a, U, V, F = sl::Vec4>
where
    V: Vertex<Sl>,
{
    pub uniform: &'a U,
    pub vertex_spec: &'a VertexSpec<V>,
    pub framebuffer: &'a F,
    pub settings: &'a DrawSettings,
}

pub struct Program<U, V, F = sl::Vec4> {
    raw: Rc<raw::Program>,
    _phantom: PhantomData<(U, V, F)>,
}

impl<U, V, F> Program<U, V, F>
where
    U: Uniform<Sl>,
    V: Vertex<Sl>,
    F: Fragment<Sl>,
{
    pub(super) fn unchecked_from_raw(raw: raw::Program) -> Self {
        Program {
            raw: Rc::new(raw),
            _phantom: PhantomData,
        }
    }

    pub fn draw<FBuf>(&self, input: DrawInput<U::Gl, V, FBuf>) -> Result<(), DrawError>
    where
        FBuf: Framebuffer<F>,
    {
        // TODO: These allocations can be avoided once stable has allocators.
        // TODO: Remove hardcoded path names.
        let mut uniform_visitor = CollectUniforms::default();
        input.uniform.visit("", &mut uniform_visitor);

        // FIXME: Safety: check that all vertex buffers are large enough for the
        // values in the element buffer (if we have one).

        unsafe {
            self.raw.draw(
                &uniform_visitor.raw_uniform_buffers,
                &uniform_visitor.raw_samplers,
                &input.vertex_spec.raw(),
                &input.framebuffer.raw(),
                input.settings,
            )
        }?;

        Ok(())
    }
}

#[derive(Default)]
struct CollectUniforms<'a> {
    raw_uniform_buffers: Vec<&'a raw::Buffer>,
    raw_samplers: Vec<raw::Sampler>,
}

impl<'a> UniformVisitor<'a, Gl> for CollectUniforms<'a> {
    fn accept_block<B: Block<Sl, Sl = B>>(
        &mut self,
        _: &str,
        uniform: &'a UniformBufferBinding<B>,
    ) {
        self.raw_uniform_buffers.push(uniform.raw());
    }

    fn accept_color_sampler_2d<S: ColorSample>(&mut self, _: &str, sampler: &ColorSampler2d<S>) {
        self.raw_samplers
            .push(raw::Sampler::Sampler2d(sampler.raw().clone()))
    }

    fn accept_comparison_sampler_2d(&mut self, _: &str, sampler: &ComparisonSampler2d) {
        self.raw_samplers
            .push(raw::Sampler::Sampler2d(sampler.raw().clone()))
    }
}
