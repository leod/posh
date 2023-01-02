use std::marker::PhantomData;

use crate::{internal::VertexInterfaceVisitor, Gl, Sl, Vertex, VertexInputRate, VertexInterface};

use super::{
    untyped::{self, VertexDataEntryInfo},
    Context, CreateVertexDataError, VertexBufferBinding,
};

pub struct VertexData<V: VertexInterface<Sl>> {
    untyped: untyped::VertexData,
    _phantom: PhantomData<V>,
}

impl<V: VertexInterface<Sl>> VertexData<V> {
    // TODO: Allow construction from `untyped::VertexData`?

    pub fn new(context: &Context, bindings: V::InGl) -> Result<Self, CreateVertexDataError> {
        let mut visitor = BindingVisitor::default();
        bindings.visit(&mut visitor);

        let untyped = context
            .untyped()
            .create_vertex_data(&visitor.bindings_and_entry_infos)?;

        Ok(VertexData {
            untyped,
            _phantom: PhantomData,
        })
    }
}

#[derive(Default)]
struct BindingVisitor {
    bindings_and_entry_infos: Vec<(untyped::BufferBinding, VertexDataEntryInfo)>,
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

        self.bindings_and_entry_infos
            .push((vertex.untyped.clone(), entry_info));
    }
}
