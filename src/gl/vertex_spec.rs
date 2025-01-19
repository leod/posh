use std::{mem::size_of, ops::Range};

use crate::{
    gl::VertexBufferBinding,
    interface::VertexVisitor,
    sl::program_def::{VertexBlockDef, VertexInputRate},
    Block, Gl, Sl, VsInterface,
};

use super::{raw, ElementBufferBinding, PrimitiveMode};

#[derive(Clone)]
pub struct VertexSpec<V: VsInterface<Sl>> {
    mode: PrimitiveMode,
    vertex_data: V::Gl,
    vertex_range: Option<Range<usize>>,
    element_data: Option<ElementBufferBinding>,
    num_instances: Option<usize>,
}

impl VertexSpec<()> {
    pub fn new(mode: PrimitiveMode) -> Self {
        Self {
            mode,
            vertex_data: (),
            vertex_range: None,
            element_data: None,
            num_instances: None,
        }
    }

    pub fn with_vertex_data<V>(self, vertex_data: V::Gl) -> VertexSpec<V>
    where
        V: VsInterface<Sl>,
    {
        let Counts {
            num_vertices,
            num_instances,
        } = get_counts(&vertex_data);

        VertexSpec {
            mode: self.mode,
            vertex_data,
            vertex_range: if self.vertex_range.is_some() {
                self.vertex_range
            } else {
                num_vertices.map(|num_vertices| 0..num_vertices)
            },
            element_data: self.element_data,
            num_instances: if self.num_instances.is_some() {
                self.num_instances
            } else {
                num_instances
            },
        }
    }
}

impl<V: VsInterface<Sl>> VertexSpec<V> {
    pub fn with_vertex_range(mut self, vertex_range: Range<usize>) -> Self {
        // NOTE: The stored `vertex_range` is ignored if an element buffer is
        // passed as well.
        self.vertex_range = Some(vertex_range);
        self
    }

    pub fn with_element_data(mut self, elements: ElementBufferBinding) -> Self {
        self.element_data = Some(elements);
        self
    }

    pub fn with_num_instances(mut self, num_instances: usize) -> Self {
        self.num_instances = Some(num_instances);
        self
    }

    pub(super) fn raw(&self) -> raw::VertexSpec {
        raw::VertexSpec {
            vertex_data: raw_vertices(&self.vertex_data),
            element_data: self
                .element_data
                .as_ref()
                .map(|elements| (elements.raw().clone(), elements.ty())),
            mode: self.mode,
            index_range: self.element_data.as_ref().map_or_else(
                || self.vertex_range.clone().unwrap_or(0..0),
                |binding| binding.range(),
            ),
            num_instances: self.num_instances.unwrap_or(1),
        }
    }
}

fn raw_vertices<V: VsInterface<Gl>>(vertices: &V) -> raw::VertexBufferBindingVec {
    struct Visitor(raw::VertexBufferBindingVec);

    impl<'a> VertexVisitor<'a, Gl> for Visitor {
        fn accept<B: Block<Sl>>(&mut self, path: &str, binding: &'a VertexBufferBinding<B>) {
            self.0.push(raw::VertexBufferBinding {
                buffer: binding.raw().clone(),
                block_def: VertexBlockDef {
                    attributes: B::vertex_attribute_defs(path),
                },
                input_rate: binding.input_rate(),
                stride: size_of::<B::Gl>(),
            });
        }
    }

    // TODO: Remove hardcoded path names.
    let mut visitor = Visitor(Default::default());
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

fn get_counts<V: VsInterface<Gl>>(vertices: &V) -> Counts {
    // TODO: Remove hardcoded path names.
    let mut counts = Counts {
        num_vertices: None,
        num_instances: None,
    };
    vertices.visit("vertex_input", &mut counts);

    counts
}
