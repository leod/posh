use std::{mem::size_of, rc::Rc};

use crevice::std140::AsStd140;

use crate::{
    gl::VertexBufferBinding,
    interface::VertexVisitor,
    sl::program_def::{VertexBlockDef, VertexInputRate},
    Block, Gl, Sl, Vertex,
};

use super::{raw, Elements, PrimitiveType};

#[derive(Clone)]
pub struct VertexStream<V> {
    pub vertices: V,
    pub elements: Elements,
    pub primitive: PrimitiveType,
}

impl<V: Vertex<Gl>> VertexStream<V> {
    pub(super) fn raw(&self) -> raw::VertexStream {
        use Elements::*;

        match &self.elements {
            BufferBinding(elements) => raw::VertexStream {
                vertices: raw_vertices(&self.vertices),
                elements: Some((elements.raw().clone(), elements.ty())),
                primitive: self.primitive,
                range: elements.range(),
                num_instances: 1,
            },
            Range(range) => raw::VertexStream {
                vertices: raw_vertices(&self.vertices),
                elements: None,
                primitive: self.primitive,
                range: range.clone(),
                num_instances: 1,
            },
        }
    }
}

fn raw_vertices<V: Vertex<Gl>>(vertices: &V) -> Vec<(Rc<raw::Buffer>, VertexBlockDef)> {
    // TODO: Reduce per-draw-call allocations.
    struct Visitor(Vec<(Rc<raw::Buffer>, VertexBlockDef)>);

    impl<'a> VertexVisitor<'a, Gl> for Visitor {
        fn accept<B: Block<Sl>>(
            &mut self,
            path: &str,
            input_rate: VertexInputRate,
            buffer: &'a VertexBufferBinding<B>,
        ) {
            let block_def = VertexBlockDef {
                input_rate,
                stride: size_of::<<B::Gl as AsStd140>::Output>(),
                attributes: B::vertex_attribute_defs(path),
            };

            self.0.push((buffer.raw().clone(), block_def));
        }
    }

    // TODO: Remove hardcoded path names.
    let mut visitor = Visitor(Vec::new());
    vertices.visit("vertex_input", &mut visitor);

    visitor.0
}
