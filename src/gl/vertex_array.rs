use std::{marker::PhantomData, ops::Range, rc::Rc};

use crate::{
    internal::VertexInterfaceVisitor,
    program_def::{VertexDef, VertexInputRate},
    Gl, Sl, Vertex, VertexInterface,
};

use super::{
    untyped, vertex_buffer::vertex_size, CreateVertexArrayError, Element, ElementOrUnit,
    ElementSource, GeometryStream, GeometryType, VertexBuffer,
};

#[derive(Clone)]
pub struct VertexArray<V: VertexInterface<Sl>, E = ()>
where
    V: VertexInterface<Sl>,
    E: ElementOrUnit,
{
    untyped: Rc<untyped::VertexArray>,
    vertex_buffers: Rc<V::InGl>,
    element_source: Rc<E::Source>,
    _phantom: PhantomData<V>,
}

impl<V, E> VertexArray<V, E>
where
    V: VertexInterface<Sl>,
    E: ElementOrUnit,
{
    pub(crate) fn new(
        context: &untyped::Context,
        vertex_buffers: V::InGl,
        element_source: E::Source,
    ) -> Result<Self, CreateVertexArrayError> {
        let mut visitor = VertexBufferVisitor::default();

        // TODO: Don't hardcode path names.
        vertex_buffers.visit("vertex_input", &mut visitor);

        let untyped =
            context.create_vertex_array(&visitor.vertex_buffers, element_source.buffer())?;

        Ok(VertexArray {
            untyped: Rc::new(untyped),
            vertex_buffers: Rc::new(vertex_buffers),
            element_source: Rc::new(element_source),
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
    ) -> GeometryStream<V::InGl> {
        GeometryStream {
            untyped: self.untyped.stream_range(element_range, geometry_type),
            _vertex_buffers: self.vertex_buffers.clone(),
        }
    }
}

impl<V, E> VertexArray<V, E>
where
    V: VertexInterface<Sl>,
    E: Element,
{
    pub fn stream(&self, geometry_type: GeometryType) -> GeometryStream<V::InGl> {
        self.stream_range(0..self.element_source().len(), geometry_type)
    }
}

#[derive(Default)]
struct VertexBufferVisitor<'a> {
    vertex_buffers: Vec<(&'a untyped::Buffer, VertexDef)>,
}

impl<'a> VertexInterfaceVisitor<'a, Gl> for VertexBufferVisitor<'a> {
    fn accept<V: Vertex<Sl>>(
        &mut self,
        path: &str,
        input_rate: VertexInputRate,
        vertex: &'a VertexBuffer<V>,
    ) {
        let stride = vertex_size::<V>();
        let attributes = V::attribute_defs(path);
        let vertex_def = VertexDef {
            input_rate,
            stride,
            attributes,
        };

        self.vertex_buffers.push((&vertex.untyped, vertex_def));
    }
}
