use std::{marker::PhantomData, rc::Rc};

use crate::{
    interface::UniformVisitor,
    sl::{self, ColorSample},
    Block, FsInterface, Gl, Sl, UniformInterface, VsInterface,
};

use super::{
    raw, ColorSampler2d, ComparisonSampler2d, DrawError, DrawSettings, Framebuffer,
    UniformBufferBinding, VertexSpec,
};

pub struct DrawInputs<U, V>
where
    U: UniformInterface<Sl>,
    V: VsInterface<Sl>,
{
    pub uniforms: U::Gl,
    pub vertex_spec: VertexSpec<V>,
    pub settings: DrawSettings,
}

pub struct Program<U, V, F = sl::Vec4> {
    raw: Rc<raw::Program>,
    _phantom: PhantomData<(U, V, F)>,
}

impl<U, V, F> Program<U, V, F>
where
    U: UniformInterface<Sl>,
    V: VsInterface<Sl>,
    F: FsInterface<Sl>,
{
    pub(super) fn unchecked_from_raw(raw: raw::Program) -> Self {
        Program {
            raw: Rc::new(raw),
            _phantom: PhantomData,
        }
    }

    pub fn draw(
        &self,
        inputs: &DrawInputs<U, V>,
        framebuffer: impl Into<Framebuffer<F>>,
    ) -> Result<(), DrawError> {
        // TODO: These allocations can be avoided once stable has allocators.
        // TODO: Remove hardcoded path names.
        let mut uniform_visitor = CollectUniforms::default();
        inputs.uniforms.visit("", &mut uniform_visitor);

        // FIXME: Safety: check that all vertex buffers are large enough for the
        // values in the element buffer (if we have one).

        unsafe {
            self.raw.draw(
                &uniform_visitor.raw_uniform_buffers,
                &uniform_visitor.raw_samplers,
                &inputs.vertex_spec.raw(),
                &framebuffer.into().raw(),
                &inputs.settings,
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
