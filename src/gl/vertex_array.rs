use std::{marker::PhantomData, ops::Range, rc::Rc};

use crate::{
    internal::VertexDataVisitor,
    program_def::{VertexDef, VertexInputRate},
    Block, Logical, Physical, VertexData,
};

use super::{
    raw, vertex_buffer::vertex_size, Element, ElementOrUnit, ElementSource, PrimitiveType,
    VertexArrayError, VertexBuffer,
};

/// Combines buffers so that they can be used in [draw
/// calls](crate::gl::Program::draw).
///
/// Instances of `VertexArray` can be created with
/// [`Context::create_vertex_array`](crate::gl::Context::create_vertex_array) or
/// [`Context::create_simple_vertex_array`](crate::gl::Context::create_simple_vertex_array).
#[derive(Clone)]
pub struct VertexArray<V, E = ()>
where
    V: VertexData<Logical>,
    E: ElementOrUnit,
{
    raw: Rc<raw::VertexArray>,
    vertex_buffers: Rc<V::Physical>,
    element_source: Rc<E::Source>,
    _phantom: PhantomData<V>,
}

impl<V, E> VertexArray<V, E>
where
    V: VertexData<Logical>,
    E: ElementOrUnit,
{
    pub(super) fn new(
        context: &raw::Context,
        vertex_buffers: V::Physical,
        element_source: E::Source,
    ) -> Result<Self, VertexArrayError> {
        let mut visitor = VertexBufferVisitor::default();

        // TODO: Don't hardcode path names.
        vertex_buffers.visit("vertex_input", &mut visitor);

        let raw = context.create_vertex_array(&visitor.raw_vertex_buffers, element_source.raw())?;

        Ok(VertexArray {
            raw: Rc::new(raw),
            vertex_buffers: Rc::new(vertex_buffers),
            element_source: Rc::new(element_source),
            _phantom: PhantomData,
        })
    }

    pub fn vertex_buffers(&self) -> &V::Physical {
        &self.vertex_buffers
    }

    pub fn element_source(&self) -> &E::Source {
        &self.element_source
    }

    pub fn range_binding(
        &self,
        element_range: Range<usize>,
        primitive_type: PrimitiveType,
    ) -> VertexArrayBinding<V::Physical> {
        VertexArrayBinding {
            raw: self.raw.range_binding(element_range, primitive_type),
            _vertex_buffers: self.vertex_buffers.clone(),
        }
    }
}

impl<V, E> VertexArray<V, E>
where
    V: VertexData<Logical>,
    E: Element,
{
    pub fn binding(&self, primitive_type: PrimitiveType) -> VertexArrayBinding<V::Physical> {
        self.range_binding(0..self.element_source().len(), primitive_type)
    }
}

/// A stream of vertices together with a primitive type.
///
/// A vertex array binding provides vertex data for [draw
/// calls](crate::gl::Program::draw). Vertex array bindings can be obtained with
/// [`VertexArray::bind`](VertexArray::bind) or
/// [`VertexArray::bind_range`](VertexArray::bind_range).
pub struct VertexArrayBinding<V> {
    pub(super) raw: raw::VertexArrayBinding,
    _vertex_buffers: Rc<V>,
}

#[derive(Default)]
struct VertexBufferVisitor<'a> {
    raw_vertex_buffers: Vec<(&'a raw::Buffer, VertexDef)>,
}

impl<'a> VertexDataVisitor<'a, Physical> for VertexBufferVisitor<'a> {
    fn accept<V: Block<Logical>>(
        &mut self,
        path: &str,
        input_rate: VertexInputRate,
        vertex: &'a VertexBuffer<V>,
    ) {
        let stride = vertex_size::<V>();
        let attributes = V::vertex_attribute_defs(path);
        let vertex_def = VertexDef {
            input_rate,
            stride,
            attributes,
        };

        self.raw_vertex_buffers.push((&vertex.raw, vertex_def));
    }
}
