use std::{marker::PhantomData, rc::Rc};

use crate::{internal::VertexInterfaceVisitor, Gl, Sl, VertexInterface};

use super::{
    untyped::{self, VertexDataEntryInfo},
    CreateVertexDataError, VertexBuffer, VertexBufferBinding,
};

pub struct VertexData<V: VertexInterface<Sl>> {
    _phantom: PhantomData<V>,
}

impl<V: VertexInterface<Sl>> VertexData<V> {
    pub fn new(gl: Rc<glow::Context>, bindings: V::InGl) -> Result<Self, CreateVertexDataError> {
        //let buffer_entry_infos = todo();

        todo!()
    }
}

struct BindingVisitor<'a> {
    buffer_entry_infos: Vec<(&'a untyped::Buffer, VertexDataEntryInfo)>,
}

impl<'a> VertexInterfaceVisitor<Gl> for BindingVisitor<'a> {
    fn accept<V: crate::Vertex<Sl>>(
        &mut self,
        path: &str,
        input_rate: crate::VertexInputRate,
        vertex: &VertexBufferBinding<V>,
    ) {
        todo!()
    }
}
