use std::{marker::PhantomData, rc::Rc};

use crate::{
    interface::UniformVisitor,
    sl::{self, ColorSample},
    Block, Fragment, Gl, Sl, Uniform, Vertex,
};

use super::{
    raw, vertex_stream::VertexStream, DrawError, DrawParams, Framebuffer, Sampler2d,
    UniformBufferBinding,
};

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
    pub fn draw(
        &self,
        uniforms: U::Gl,
        vertices: VertexStream<V::Gl>,
        framebuffer: impl Framebuffer<F::Gl>,
        draw_params: DrawParams,
    ) -> Result<(), DrawError> {
        // TODO: These allocations can be avoided once stable has allocators.
        // TODO: Remove hardcoded path names.
        let mut uniform_visitor = CollectUniforms::default();
        uniforms.visit("", &mut uniform_visitor);

        // FIXME: Safety: check that all vertex buffers are large enough for the
        // values in the element buffer (if we have one).

        unsafe {
            self.raw.draw(
                &uniform_visitor.raw_uniform_buffers,
                &uniform_visitor.raw_samplers,
                &vertices.raw(),
                &framebuffer.raw(),
                &draw_params,
            )
        }
    }

    pub(super) fn unchecked_from_raw(raw: raw::Program) -> Self {
        Program {
            raw: Rc::new(raw),
            _phantom: PhantomData,
        }
    }
}

#[derive(Default)]
struct CollectUniforms<'a> {
    raw_uniform_buffers: Vec<&'a raw::Buffer>,
    raw_samplers: Vec<raw::Sampler>,
}

impl<'a> UniformVisitor<'a, Gl> for CollectUniforms<'a> {
    fn accept_sampler2d<S: ColorSample>(&mut self, _: &str, sampler: &Sampler2d<S>) {
        self.raw_samplers
            .push(raw::Sampler::Sampler2d(sampler.raw().clone()))
    }

    fn accept_block<B: Block<Sl, Sl = B>>(
        &mut self,
        _: &str,
        uniform: &'a UniformBufferBinding<B>,
    ) {
        self.raw_uniform_buffers.push(uniform.raw());
    }
}
