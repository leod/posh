use std::{marker::PhantomData, rc::Rc};

use crate::{
    interface::UniformInterfaceVisitor,
    sl::{self, Sample},
    Block, FragmentInterface, Gl, Sl, UniformInterface, VertexInterface,
};

use super::{raw, DrawParams, Sampler2d, Surface, UniformBufferBinding, VertexArrayBinding};

#[derive(Clone)]
pub struct Program<Unif, Vert, Frag = sl::Vec4<f32>> {
    raw: Rc<raw::Program>,
    _phantom: PhantomData<(Unif, Vert, Frag)>,
}

impl<Unif, Vert, Frag> Program<Unif, Vert, Frag>
where
    Unif: UniformInterface<Sl>,
    Vert: VertexInterface<Sl>,
    Frag: FragmentInterface<Sl>,
{
    pub(super) fn unchecked_from_raw(raw: raw::Program) -> Self {
        Program {
            raw: Rc::new(raw),
            _phantom: PhantomData,
        }
    }

    pub fn draw<S>(
        &self,
        uniforms: Unif::InGl,
        vertices: VertexArrayBinding<Vert::InGl>,
        surface: &S,
        draw_params: &DrawParams,
    ) where
        S: Surface<Frag>,
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

impl<'a> UniformInterfaceVisitor<'a, Gl> for UniformVisitor<'a> {
    fn accept_sampler2d<S: Sample>(&mut self, path: &str, sampler: &Sampler2d<S>) {
        self.raw_samplers
            .push(raw::Sampler::Sampler2d(sampler.raw.clone()))
    }

    fn accept_uniform<U: Block<Sl, InSl = U>>(
        &mut self,
        _: &str,
        uniform: &'a UniformBufferBinding<U>,
    ) {
        self.raw_uniform_buffers.push(&uniform.raw);
    }
}
