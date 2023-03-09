use std::{marker::PhantomData, rc::Rc};

use crate::{
    interface::UniformVisitor,
    sl::{self, Sample},
    Block, Fragment, GlView, SlView, Uniform, Vertex,
};

use super::{
    raw, vertex_stream::VertexStream, DrawParams, FramebufferBinding, Texture2dBinding,
    UniformBufferBinding,
};

#[derive(Clone)]
pub struct Program<U, V, F = sl::Vec4> {
    raw: Rc<raw::Program>,
    _phantom: PhantomData<(U, V, F)>,
}

impl<U, V, F> Program<U, V, F>
where
    U: Uniform<SlView>,
    V: Vertex<SlView>,
    F: Fragment<SlView>,
{
    pub(super) fn unchecked_from_raw(raw: raw::Program) -> Self {
        Program {
            raw: Rc::new(raw),
            _phantom: PhantomData,
        }
    }

    pub fn draw(
        &self,
        uniforms: U::GlView,
        vertices: VertexStream<V::GlView>,
        framebuffer: FramebufferBinding<F::GlView>,
        draw_params: DrawParams,
    ) {
        // TODO: Surface stuff.

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
                framebuffer.raw(),
            );
        }
    }
}

#[derive(Default)]
struct CollectUniforms<'a> {
    raw_uniform_buffers: Vec<&'a raw::Buffer>,
    raw_samplers: Vec<raw::TextureBinding>,
}

impl<'a> UniformVisitor<'a, GlView> for CollectUniforms<'a> {
    fn accept_sampler2d<S: Sample>(&mut self, path: &str, sampler: &Texture2dBinding<S>) {
        self.raw_samplers
            .push(raw::TextureBinding::Texture2d(sampler.raw().clone()))
    }

    fn accept_block<B: Block<SlView, SlView = B>>(
        &mut self,
        _: &str,
        uniform: &'a UniformBufferBinding<B>,
    ) {
        self.raw_uniform_buffers.push(&uniform.raw());
    }
}
