use std::marker::PhantomData;

use crate::{internal::VertexInterfaceVisitor, Gl, Sl, Vertex, VertexInputRate, VertexInterface};

use super::{
    untyped::{self, VertexBindingBufferInfo},
    Context, CreateVertexDataError, Element, ElementBuffer, VertexBuffer,
};

pub struct VertexBinding<V: VertexInterface<Sl>> {
    untyped: untyped::VertexBinding,
    vertex_buffers: V::InGl,
    _phantom: PhantomData<V>,
}

impl<V: VertexInterface<Sl>> VertexBinding<V> {
    // TODO: Allow construction from `untyped::VertexData`?

    pub fn new<E: Element>(
        context: &Context,
        vertex_buffers: V::InGl,
        element_buffer: Option<ElementBuffer<E>>,
    ) -> Result<Self, CreateVertexDataError> {
        let mut visitor = VertexBufferVisitor::default();
        vertex_buffers.visit(&mut visitor);

        let untyped = context.untyped().create_vertex_binding(
            &visitor.vertex_buffers,
            element_buffer.map(|buffer| todo!()),
        )?;

        Ok(VertexBinding {
            untyped,
            vertex_buffers,
            _phantom: PhantomData,
        })
    }

    pub fn vertex_buffers(&self) -> &V::InGl {
        &self.vertex_buffers
    }
}

#[derive(Default)]
struct VertexBufferVisitor {
    vertex_buffers: Vec<(untyped::Buffer, VertexBindingBufferInfo)>,
}

impl VertexInterfaceVisitor<Gl> for VertexBufferVisitor {
    fn accept<V: Vertex<Sl>>(
        &mut self,
        path: &str,
        input_rate: VertexInputRate,
        vertex: &VertexBuffer<V>,
    ) {
        let stride = std::mem::size_of::<V::Pod>();
        let attributes = V::attributes(path);
        let entry_info = VertexBindingBufferInfo {
            input_rate,
            stride,
            attributes,
        };

        self.vertex_buffers
            .push((vertex.untyped.clone(), entry_info));
    }
}
