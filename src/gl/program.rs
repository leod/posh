use std::{cell::RefCell, marker::PhantomData};

use crate::{
    interface::ResourceInterfaceVisitor, program_def::ProgramDef, sl, FragmentInterface, Gl,
    ResourceInterface, Sl, VertexInterface,
};

use super::{
    untyped, Context, CreateProgramError, DrawParams, GeometryStream, Surface, UniformBufferBinding,
};

pub struct Program<Res, Vert, Frag = sl::Vec4<f32>> {
    untyped: untyped::Program,
    uniform_buffers: RefCell<Vec<untyped::Buffer>>,
    _phantom: PhantomData<(Res, Vert, Frag)>,
}

impl<Res, Vert, Frag> Program<Res, Vert, Frag>
where
    Res: ResourceInterface<Sl, InSl = Res>,
    Vert: VertexInterface<Sl, InSl = Vert>,
    Frag: FragmentInterface<Sl, InSl = Frag>,
{
    pub(crate) fn unchecked_from_untyped_program_def(
        context: &Context,
        program_def: ProgramDef,
    ) -> Result<Self, CreateProgramError> {
        let untyped = context.untyped.create_program(program_def)?;

        Ok(Program {
            untyped,
            uniform_buffers: RefCell::new(Vec::new()),
            _phantom: PhantomData,
        })
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

        let mut uniform_buffers = self.uniform_buffers.borrow_mut();
        let mut resource_visitor = ResourceVisitor {
            uniform_buffers: &mut uniform_buffers,
        };
        resource.visit("", &mut resource_visitor);

        // FIXME: Safety: Check element range.
        unsafe {
            self.untyped.draw(&uniform_buffers, geometry.untyped);
        }

        uniform_buffers.clear();
    }
}

struct ResourceVisitor<'a> {
    uniform_buffers: &'a mut Vec<untyped::Buffer>,
}

impl<'a> ResourceInterfaceVisitor<Gl> for ResourceVisitor<'a> {
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
        uniform: &UniformBufferBinding<U>,
    ) {
        self.uniform_buffers.push(uniform.untyped.clone());
    }
}
