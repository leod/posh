use std::marker::PhantomData;

use crate::{internal::VertexInterfaceVisitor, Gl, Sl, Vertex, VertexInputRate, VertexInterface};

use super::{
    untyped::{self, VertexDataEntryInfo},
    Context, CreateVertexDataError, GeometryType, VertexBuffer,
};

pub struct VertexData<V: VertexInterface<Sl>> {
    untyped: untyped::VertexData,
    vertex_buffers: V::InGl,
    _phantom: PhantomData<V>,
}

pub struct VertexDataBinding<V: VertexInterface<Sl>> {
    untyped: untyped::VertexDataBinding,
    _phantom: PhantomData<V>,
}

impl<V: VertexInterface<Sl>> VertexData<V> {
    // TODO: Allow construction from `untyped::VertexData`?

    pub fn new(context: &Context, vertex_buffers: V::InGl) -> Result<Self, CreateVertexDataError> {
        let mut visitor = BufferVisitor::default();
        vertex_buffers.visit(&mut visitor);

        let untyped = context
            .untyped()
            .create_vertex_data(&visitor.vertex_buffers_and_entry_infos)?;

        Ok(VertexData {
            untyped,
            vertex_buffers,
            _phantom: PhantomData,
        })
    }

    pub fn vertex_buffers(&self) -> &V::InGl {
        &self.vertex_buffers
    }

    pub fn bind(&self, geometry_type: GeometryType) -> VertexDataBinding<V> {
        VertexDataBinding {
            untyped: self.untyped.bind(geometry_type),
            _phantom: PhantomData,
        }
    }
}

#[derive(Default)]
struct BufferVisitor {
    vertex_buffers_and_entry_infos: Vec<(untyped::Buffer, VertexDataEntryInfo)>,
}

impl VertexInterfaceVisitor<Gl> for BufferVisitor {
    fn accept<V: Vertex<Sl>>(
        &mut self,
        path: &str,
        input_rate: VertexInputRate,
        vertex: &VertexBuffer<V>,
    ) {
        let stride = std::mem::size_of::<V::Pod>();
        let attributes = V::attributes(path);
        let entry_info = VertexDataEntryInfo {
            input_rate,
            stride,
            attributes,
        };

        self.vertex_buffers_and_entry_infos
            .push((vertex.untyped.clone(), entry_info));
    }
}
