use std::{marker::PhantomData, ops::Range};

use crate::{internal::VertexInterfaceVisitor, Gl, Sl, Vertex, VertexInputRate, VertexInterface};

use super::{
    untyped::{self, VertexInfo},
    Context, CreateVertexArrayError, Element, ElementOrUnit, ElementSource, GeometryStream,
    GeometryType, VertexBuffer,
};

#[derive(Clone)]
pub struct VertexArray<V: VertexInterface<Sl>, E = ()>
where
    V: VertexInterface<Sl>,
    E: ElementOrUnit,
{
    untyped: untyped::VertexArray,
    vertex_buffers: V::InGl,
    element_source: E::Source,
    _phantom: PhantomData<V>,
}

impl<V, E> VertexArray<V, E>
where
    V: VertexInterface<Sl>,
    E: ElementOrUnit,
{
    // TODO: Allow construction from `untyped::VertexData`?
    pub(crate) fn new(
        context: &Context,
        vertex_buffers: V::InGl,
        element_source: E::Source,
    ) -> Result<Self, CreateVertexArrayError> {
        let mut visitor = VertexBufferVisitor::default();
        vertex_buffers.visit("vertex_input", &mut visitor);

        let untyped = context
            .untyped
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

    pub fn element_source(&self) -> &E::Source {
        &self.element_source
    }

    pub fn stream_range(
        &self,
        element_range: Range<usize>,
        geometry_type: GeometryType,
    ) -> GeometryStream<V> {
        GeometryStream {
            untyped: self.untyped.stream_range(element_range, geometry_type),
            _phantom: PhantomData,
        }
    }
}

impl<V, E> VertexArray<V, E>
where
    V: VertexInterface<Sl>,
    E: Element,
{
    pub fn stream(&self, geometry_type: GeometryType) -> GeometryStream<V> {
        self.stream_range(0..self.element_source().len(), geometry_type)
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
