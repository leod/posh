use std::{mem::size_of, ops::Range};

use crevice::std140::AsStd140;

use crate::{
    gl::VertexBufferBinding,
    interface::VertexVisitor,
    sl::program_def::{VertexBlockDef, VertexInputRate},
    Block, GlView, SlView, Vertex,
};

use super::{raw, ElementBufferBinding, PrimitiveType};

#[derive(Clone)]
pub enum VertexStream<V> {
    Indexed {
        vertices: V,
        elements: ElementBufferBinding,
        primitive: PrimitiveType,
    },
    Unindexed {
        vertices: V,
        range: Range<usize>,
        primitive: PrimitiveType,
    },
}

impl<V: Vertex<GlView>> VertexStream<V> {
    pub(super) fn raw(&self) -> raw::VertexStream {
        use VertexStream::*;

        match self {
            Indexed {
                vertices,
                elements,
                primitive,
            } => raw::VertexStream {
                vertices: raw_vertices(vertices),
                elements: Some((elements.raw(), elements.ty())),
                primitive: *primitive,
                range: elements.range().clone(),
                num_instances: 1,
            },
            Unindexed {
                vertices,
                range,
                primitive,
            } => raw::VertexStream {
                vertices: raw_vertices(vertices),
                elements: None,
                primitive: *primitive,
                range: range.clone(),
                num_instances: 1,
            },
        }
    }
}

fn raw_vertices<V: Vertex<GlView>>(vertices: &V) -> Vec<(&raw::Buffer, VertexBlockDef)> {
    // TODO: Reduce per-draw-call allocations.
    struct Visitor<'a>(Vec<(&'a raw::Buffer, VertexBlockDef)>);

    impl<'a> VertexVisitor<'a, GlView> for Visitor<'a> {
        fn accept<B: Block<SlView>>(
            &mut self,
            path: &str,
            input_rate: VertexInputRate,
            buffer: &'a VertexBufferBinding<B>,
        ) {
            let block_def = VertexBlockDef {
                input_rate,
                stride: size_of::<<B::GlView as AsStd140>::Output>(),
                attributes: B::vertex_attribute_defs(path),
            };

            self.0.push((buffer.raw(), block_def));
        }
    }

    // TODO: Remove hardcoded path names.
    let mut visitor = Visitor(Vec::new());
    vertices.visit("vertex_input", &mut visitor);

    visitor.0
}
