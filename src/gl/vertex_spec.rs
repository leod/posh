use std::{mem::size_of, ops::Range};

use crevice::std140::AsStd140;

use crate::{
    gl::VertexBufferBinding,
    interface::VertexVisitor,
    sl::program_def::{VertexBlockDef, VertexInputRate},
    Block, Gl, Sl, Vertex,
};

use super::{raw, ElementBufferBinding, PrimitiveMode};

#[derive(Clone)]
pub struct VertexSpec<V> {
    pub mode: PrimitiveMode,
    pub vertices: V,
    pub vertex_range: Range<usize>,
    pub num_instances: usize,
    pub elements: Option<ElementBufferBinding>,
}

impl VertexSpec<()> {
    pub fn new(mode: PrimitiveMode) -> Self {
        Self {
            mode,
            vertices: (),
            vertex_range: 0..0,
            num_instances: 0,
            elements: None,
        }
    }

    pub fn with_vertices<V>(self, vertices: V) -> VertexSpec<V>
    where
        V: Vertex<Gl>,
    {
        let Counts {
            num_vertices,
            num_instances,
        } = get_counts(&vertices);

        VertexSpec {
            mode: self.mode,
            vertices,
            vertex_range: 0..num_vertices.unwrap_or(0),
            num_instances: num_instances.unwrap_or(1),
            elements: self.elements,
        }
    }
}

impl<V: Vertex<Gl>> VertexSpec<V> {
    pub fn with_vertex_range(mut self, vertex_range: Range<usize>) -> Self {
        // NOTE: The stored `vertex_range` is ignored if an element buffer is
        // passed as well.
        self.vertex_range = vertex_range;
        self
    }

    pub fn with_num_instances(mut self, num_instances: usize) -> Self {
        self.num_instances = num_instances;
        self
    }

    pub fn with_elements(mut self, elements: ElementBufferBinding) -> Self {
        self.elements = Some(elements);
        self
    }

    pub(super) fn raw(&self) -> raw::VertexSpec {
        raw::VertexSpec {
            vertices: raw_vertices(&self.vertices),
            elements: self
                .elements
                .as_ref()
                .map(|elements| (elements.raw().clone(), elements.ty())),
            mode: self.mode,
            index_range: self
                .elements
                .as_ref()
                .map_or_else(|| self.vertex_range.clone(), |binding| binding.range()),
            num_instances: self.num_instances,
        }
    }
}

fn raw_vertices<V: Vertex<Gl>>(vertices: &V) -> Vec<raw::VertexBufferBinding> {
    // TODO: Reduce per-draw-call allocations.
    struct Visitor(Vec<raw::VertexBufferBinding>);

    impl<'a> VertexVisitor<'a, Gl> for Visitor {
        fn accept<B: Block<Sl>>(&mut self, path: &str, binding: &'a VertexBufferBinding<B>) {
            self.0.push(raw::VertexBufferBinding {
                buffer: binding.raw().clone(),
                block_def: VertexBlockDef {
                    attributes: B::vertex_attribute_defs(path),
                },
                input_rate: binding.input_rate(),
                stride: size_of::<<B::Gl as AsStd140>::Output>(),
            });
        }
    }

    // TODO: Remove hardcoded path names.
    let mut visitor = Visitor(Vec::new());
    vertices.visit("vertex_input", &mut visitor);

    visitor.0
}

#[derive(Clone)]
struct Counts {
    num_vertices: Option<usize>,
    num_instances: Option<usize>,
}

impl<'a> VertexVisitor<'a, Gl> for Counts {
    fn accept<B: Block<Sl>>(&mut self, _: &str, binding: &'a VertexBufferBinding<B>) {
        let len = binding.len();

        match binding.input_rate() {
            VertexInputRate::Vertex => {
                if let Some(num_vertices) = self.num_vertices {
                    assert!(num_vertices == len);
                }
                self.num_vertices = Some(len);
            }
            VertexInputRate::Instance => {
                if let Some(num_instances) = self.num_instances {
                    assert!(num_instances == len);
                }
                self.num_instances = Some(len);
            }
        }
    }
}

fn get_counts<V: Vertex<Gl>>(vertices: &V) -> Counts {
    // TODO: Remove hardcoded path names.
    let mut counts = Counts {
        num_vertices: None,
        num_instances: None,
    };
    vertices.visit("vertex_input", &mut counts);

    counts
}
