use std::marker::PhantomData;

use crate::{internal::VertexInterfaceVisitor, Gl, Sl, Vertex, VertexInputRate, VertexInterface};

use super::{
    untyped::{self, VertexInfo},
    Context, CreateVertexArrayError, ElementSource, VertexBuffer,
};

#[derive(Clone)]
pub struct VertexArray<V: VertexInterface<Sl>, E: ElementSource> {
    untyped: untyped::VertexArray,
    vertex_buffers: V::InGl,
    element_source: E,
    _phantom: PhantomData<V>,
}

impl<V: VertexInterface<Sl>, E: ElementSource> VertexArray<V, E> {
    // TODO: Allow construction from `untyped::VertexData`?
    pub fn new(
        context: &Context,
        vertex_buffers: V::InGl,
        element_source: E,
    ) -> Result<Self, CreateVertexArrayError> {
        let mut visitor = VertexBufferVisitor::default();
        vertex_buffers.visit(&mut visitor);

        let untyped = context
            .untyped()
            .create_vertex_array(&visitor.vertex_buffers, element_source.buffer())?;

        Ok(VertexArray {
            untyped,
            vertex_buffers,
            element_source,
            _phantom: PhantomData,
        })
    }

    pub fn vertex_buffers(&self) -> &V::InGl {
        &self.vertex_buffers
    }

    pub fn element_source(&self) -> &E {
        &self.element_source
    }
}

#[derive(Default)]
struct VertexBufferVisitor {
    vertex_buffers: Vec<(untyped::Buffer, VertexInfo)>,
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
        let entry_info = VertexInfo {
            input_rate,
            stride,
            attributes,
        };

        self.vertex_buffers
            .push((vertex.untyped.clone(), entry_info));
    }
}
