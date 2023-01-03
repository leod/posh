use std::marker::PhantomData;

use crate::{internal::VertexInterfaceVisitor, Gl, Sl, Vertex, VertexInputRate, VertexInterface};

use super::{
    untyped::{self, VertexStreamBufferInfo},
    Context, CreateVertexStreamError, ElementSource, VertexBuffer,
};

pub struct VertexStream<V: VertexInterface<Sl>, E: ElementSource> {
    untyped: untyped::VertexStream,
    vertex_buffers: V::InGl,
    element_source: E,
    _phantom: PhantomData<V>,
}

impl<V: VertexInterface<Sl>, E: ElementSource> VertexStream<V, E> {
    // TODO: Allow construction from `untyped::VertexData`?
    pub fn new(
        context: &Context,
        vertex_buffers: V::InGl,
        element_source: E,
    ) -> Result<Self, CreateVertexStreamError> {
        let mut visitor = VertexBufferVisitor::default();
        vertex_buffers.visit(&mut visitor);

        let untyped = context
            .untyped()
            .create_vertex_stream(&visitor.vertex_buffers, element_source.buffer())?;

        Ok(VertexStream {
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
    vertex_buffers: Vec<(untyped::Buffer, VertexStreamBufferInfo)>,
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
        let entry_info = VertexStreamBufferInfo {
            input_rate,
            stride,
            attributes,
        };

        self.vertex_buffers
            .push((vertex.untyped.clone(), entry_info));
    }
}
