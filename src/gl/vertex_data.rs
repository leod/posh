use std::marker::PhantomData;

use crate::{internal::VertexInterfaceVisitor, Gl, Sl, Vertex, VertexInputRate, VertexInterface};

use super::{
    untyped::{self, VertexDataEntryInfo},
    Context, CreateVertexDataError, GeometryType, VertexBufferBinding,
};

pub struct VertexData<V: VertexInterface<Sl>> {
    untyped: untyped::VertexData,
    _phantom: PhantomData<V>,
}

pub struct VertexDataBinding<V: VertexInterface<Sl>> {
    untyped: untyped::VertexDataBinding,
    _phantom: PhantomData<V>,
}

impl<V: VertexInterface<Sl>> VertexData<V> {
    // TODO: Allow construction from `untyped::VertexData`?

    pub fn new(context: &Context, vertex_bindings: V::InGl) -> Result<Self, CreateVertexDataError> {
        let mut visitor = BindingVisitor::default();
        vertex_bindings.visit(&mut visitor);

        let untyped = context
            .untyped()
            .create_vertex_data(&visitor.vertex_bindings_and_entry_infos)?;

        Ok(VertexData {
            untyped,
            _phantom: PhantomData,
        })
    }

    pub fn bind(&self, geometry_type: GeometryType) -> VertexDataBinding<V> {
        VertexDataBinding {
            untyped: self.untyped.bind(geometry_type),
            _phantom: PhantomData,
        }
    }
}

#[derive(Default)]
struct BindingVisitor {
    vertex_bindings_and_entry_infos: Vec<(untyped::BufferBinding, VertexDataEntryInfo)>,
}

impl VertexInterfaceVisitor<Gl> for BindingVisitor {
    fn accept<V: Vertex<Sl>>(
        &mut self,
        path: &str,
        input_rate: VertexInputRate,
        vertex: &VertexBufferBinding<V>,
    ) {
        let stride = std::mem::size_of::<V::Pod>();
        let attributes = V::attributes(path);
        let entry_info = VertexDataEntryInfo {
            input_rate,
            stride,
            attributes,
        };

        self.vertex_bindings_and_entry_infos
            .push((vertex.untyped.clone(), entry_info));
    }
}
