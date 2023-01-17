use std::{marker::PhantomData, rc::Rc};

use crate::{
    interface::UniformInterfaceVisitor, sl, FragmentInterface, Gl, Sl, UniformInterface,
    VertexInterface,
};

use super::{untyped, DrawParams, GeometryStream, Surface, UniformBufferBinding};

#[derive(Clone)]
pub struct Program<Unif, Vert, Frag = sl::Vec4<f32>> {
    untyped: Rc<untyped::Program>,
    _phantom: PhantomData<(Unif, Vert, Frag)>,
}

impl<Unif, Vert, Frag> Program<Unif, Vert, Frag>
where
    Unif: UniformInterface<Sl>,
    Vert: VertexInterface<Sl>,
    Frag: FragmentInterface<Sl>,
{
    pub(crate) fn unchecked_from_untyped(untyped: untyped::Program) -> Self {
        Program {
            untyped: Rc::new(untyped),
            _phantom: PhantomData,
        }
    }

    pub fn draw<S>(
        &self,
        uniforms: Unif::InGl,
        geometry: GeometryStream<Vert::InGl>,
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
            self.untyped
                .draw(&uniform_visitor.untyped_uniform_buffers, geometry.untyped);
        }
    }
}

#[derive(Default)]
struct UniformVisitor<'a> {
    untyped_uniform_buffers: Vec<&'a untyped::Buffer>,
}

impl<'a> UniformInterfaceVisitor<'a, Gl> for UniformVisitor<'a> {
    fn accept_sampler2d<T: crate::Numeric>(
        &mut self,
        path: &str,
        sampler: &<Gl as crate::UniformDomain>::Sampler2d<T>,
    ) {
        todo!()
    }

    fn accept_uniform<U: crate::Block<Sl, InSl = U>>(
        &mut self,
        _: &str,
        uniform: &'a UniformBufferBinding<U>,
    ) {
        self.untyped_uniform_buffers.push(&uniform.untyped);
    }
}
