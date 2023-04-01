use std::{mem::size_of, ops::Range, rc::Rc};

use crevice::std140::AsStd140;

use crate::{
    gl::VertexBufferBinding,
    interface::VertexVisitor,
    sl::program_def::{VertexBlockDef, VertexInputRate},
    Block, Gl, Sl, Vertex,
};

use super::{raw, ElementBufferBinding, Mode};

#[derive(Clone)]
pub struct VertexSpec<V> {
    vertices: V,
    elements: Option<ElementBufferBinding>,
    mode: Mode,
    element_range: Range<usize>,
    instance_range: Range<usize>,
}

impl<V: Vertex<Gl>> VertexSpec<V> {
    pub fn vertices(vertices: V, mode: Mode) -> Self {
        let element_range = 0..min_num_vertices(&vertices).0;

        Self {
            vertices,
            elements: None,
            mode,
            element_range,
            instance_range: 0..1,
        }
    }

    pub fn indexed(vertices: V, elements: ElementBufferBinding, mode: Mode) -> Self {
        let element_range = 0..elements.len();

        Self {
            vertices,
            elements: Some(elements),
            mode,
            element_range,
            instance_range: 0..1,
        }
    }

    pub fn with_element_range(mut self, element_range: Range<usize>) -> Self {
        assert!(
            element_range.end
                <= self.elements.as_ref().map_or_else(
                    || min_num_vertices(&self.vertices).0,
                    |elements| elements.len()
                )
        );

        self.element_range = element_range;
        self
    }

    pub fn with_instance_range(mut self, instance_range: Range<usize>) -> Self {
        assert!(instance_range.end <= min_num_vertices(&self.vertices).1);

        self.instance_range = instance_range;
        self
    }

    pub(super) fn raw(self) -> raw::VertexSpec {
        raw::VertexSpec {
            vertices: raw_vertices(&self.vertices),
            elements: self
                .elements
                .as_ref()
                .map(|elements| (elements.raw().clone(), elements.ty())),
            mode: self.mode,
            element_range: self.element_range.clone(),
            instance_range: self.instance_range.clone(),
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

fn min_num_vertices<V: Vertex<Gl>>(vertices: &V) -> (usize, usize) {
    struct Visitor(Option<usize>, Option<usize>);

    impl<'a> VertexVisitor<'a, Gl> for Visitor {
        fn accept<B: Block<Sl>>(
            &mut self,
            _: &str,
            input_rate: VertexInputRate,
            buffer: &'a VertexBufferBinding<B>,
        ) {
            let len = buffer.len();

            match input_rate {
                VertexInputRate::Vertex => self.0 = Some(self.0.map_or(len, |min| min.min(len))),
                VertexInputRate::Instance => self.1 = Some(self.0.map_or(len, |min| min.min(len))),
            }
        }
    }

    // TODO: Remove hardcoded path names.
    let mut visitor = Visitor(None, None);
    vertices.visit("vertex_input", &mut visitor);

    (visitor.0.unwrap_or(0), visitor.1.unwrap_or(1))
}
