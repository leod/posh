use std::{cell::RefCell, marker::PhantomData, rc::Rc};

use crate::{
    interface::ResourceInterfaceVisitor, sl, FragmentInterface, Gl, ResourceInterface, Sl,
    VertexInterface,
};

use super::{untyped, DrawParams, GeometryStream, Surface, UniformBufferBinding};

#[derive(Clone)]
pub struct Program<Res, Vert, Frag = sl::Vec4<f32>> {
    untyped: Rc<untyped::Program>,
    _phantom: PhantomData<(Res, Vert, Frag)>,
}

impl<Res, Vert, Frag> Program<Res, Vert, Frag>
where
    Res: ResourceInterface<Sl>,
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
        resource: Res::InGl,
        geometry: GeometryStream<Vert>,
        surface: &S,
        draw_params: &DrawParams,
    ) where
        S: Surface<Frag>,
    {
        // TODO: Surface stuff.

        // TODO: This allocation can be avoided once stable has allocators.
        let mut resource_visitor = ResourceVisitor::default();
        resource.visit("", &mut resource_visitor);

        // FIXME: Safety: Check element range.
        unsafe {
            self.untyped
                .draw(&resource_visitor.uniform_buffers, geometry.untyped);
        }
    }
}

#[derive(Default)]
struct ResourceVisitor<'a> {
    uniform_buffers: Vec<&'a untyped::Buffer>,
}

impl<'a> ResourceInterfaceVisitor<'a, Gl> for ResourceVisitor<'a> {
    fn accept_sampler2d<T: crate::Numeric>(
        &mut self,
        path: &str,
        sampler: &<Gl as crate::ResourceDomain>::Sampler2d<T>,
    ) {
        todo!()
    }

    fn accept_uniform<U: crate::Uniform<Sl, InSl = U>>(
        &mut self,
        _: &str,
        uniform: &'a UniformBufferBinding<U>,
    ) {
        self.uniform_buffers.push(&uniform.untyped);
    }
}
