use std::{ops::Range, rc::Rc};

use glow::HasContext;

use crate::{
    gl::raw::{
        error::check_gl_error,
        vertex_layout::{VertexAttributeLayout, VertexAttributeType},
    },
    sl::program_def::{VertexBlockDef, VertexInputRate},
};

use super::{context::ContextShared, Buffer};

#[derive(Debug, Copy, Clone)]
pub enum ElementType {
    U16,
    U32,
}

impl ElementType {
    pub fn to_gl(self) -> u32 {
        use ElementType::*;

        match self {
            U16 => glow::UNSIGNED_SHORT,
            U32 => glow::UNSIGNED_INT,
        }
    }

    pub fn size(self) -> usize {
        use ElementType::*;

        match self {
            U16 => 2,
            U32 => 4,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Mode {
    Points,
    Lines,
    LineStrip,
    LineLoop,
    Triangles,
    TriangleStrip,
    TriangleFan,
}

impl Mode {
    pub const fn to_gl(self) -> u32 {
        use Mode::*;

        match self {
            Points => glow::POINTS,
            Lines => glow::LINES,
            LineStrip => glow::LINE_STRIP,
            LineLoop => glow::LINE_LOOP,
            Triangles => glow::TRIANGLES,
            TriangleStrip => glow::TRIANGLE_STRIP,
            TriangleFan => glow::TRIANGLE_FAN,
        }
    }
}

// TODO: VertexBlockDef should eventually be split into program definition and
// vertex stream definition.

// TODO: Instancing.

#[derive(Clone)]
pub struct PrimitiveStream {
    pub vertices: Vec<(Rc<Buffer>, VertexBlockDef)>,
    pub elements: Option<(Rc<Buffer>, ElementType)>,
    pub primitive: Mode,
    pub range: Range<usize>,
    pub num_instances: usize,
}

impl PrimitiveStream {
    pub fn is_compatible(&self, vertex_block_defs: &[VertexBlockDef]) -> bool {
        // TODO: Check vertex stream compatibility. This is already ensured by
        // the typed interface on top of `raw`, but `raw` should be correct by
        // iself.
        true
    }

    fn bind(&self, ctx: &ContextShared) {
        let mut index = 0;

        let gl = ctx.gl();

        for (buffer, vertex_def) in &self.vertices {
            assert!(vertex_def.stride > 0);
            assert_eq!(buffer.len() % vertex_def.stride, 0);
            assert!(buffer.context().ref_eq(ctx));

            unsafe {
                gl.bind_buffer(glow::ARRAY_BUFFER, Some(buffer.id()));
            }

            for attribute in &vertex_def.attributes {
                let attribute_info =
                    VertexAttributeLayout::new(attribute.ty).expect("invalid vertex attribute");

                for i in 0..attribute_info.locations {
                    let offset = attribute.offset + i * attribute_info.location_size();

                    assert!(offset + attribute_info.location_size() <= vertex_def.stride);

                    unsafe {
                        gl.enable_vertex_attrib_array(index);
                    }

                    match vertex_def.input_rate {
                        VertexInputRate::Vertex => (),
                        VertexInputRate::Instance => unsafe {
                            gl.vertex_attrib_divisor(index, 1);
                        },
                    }

                    let size = i32::try_from(attribute_info.components).unwrap();
                    let data_type = attribute_info.ty.to_gl();
                    let stride = i32::try_from(vertex_def.stride).unwrap();
                    let offset = i32::try_from(offset).unwrap();

                    match attribute_info.ty {
                        VertexAttributeType::F32 => unsafe {
                            gl.vertex_attrib_pointer_f32(
                                index, size, data_type, false, stride, offset,
                            )
                        },
                        VertexAttributeType::I32 | VertexAttributeType::U32 => unsafe {
                            gl.vertex_attrib_pointer_i32(index, size, data_type, stride, offset)
                        },
                    }

                    index += 1;
                }
            }
        }

        if let Some((buffer, _)) = self.elements.as_ref() {
            assert!(buffer.context().ref_eq(ctx));

            unsafe {
                gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, Some(buffer.id()));
            }
        }

        check_gl_error(gl).expect("OpenGL error after PrimitiveStream::bind()");
    }

    fn unbind(&self, ctx: &ContextShared) {
        let gl = ctx.gl();

        let mut index = 0;

        for (_, vertex_def) in &self.vertices {
            for attribute in &vertex_def.attributes {
                let attribute_info =
                    VertexAttributeLayout::new(attribute.ty).expect("invalid vertex attribute");

                for _ in 0..attribute_info.locations {
                    unsafe {
                        gl.disable_vertex_attrib_array(index);
                    }

                    index += 1;
                }
            }
        }

        if self.elements.is_some() {
            unsafe {
                gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, None);
            }
        }

        check_gl_error(gl).expect("OpenGL error after PrimitiveStream::unbind()");
    }

    pub(super) fn draw(&self, ctx: &ContextShared) {
        assert!(self.range.start <= self.range.end);

        let gl = ctx.gl();

        self.bind(ctx);

        let mode = self.primitive.to_gl();
        let first = self.range.start;
        let count = self.range.end - self.range.start;

        if let Some((buffer, element_type)) = &self.elements {
            let element_size = element_type.size();
            let element_type = element_type.to_gl();

            let offset = first.checked_mul(element_size).unwrap();

            // Safety: check element range.
            {
                let size = count.checked_mul(element_size).unwrap();

                assert!(offset.checked_add(size).unwrap() <= buffer.len());
            }

            let count = count.try_into().expect("count is out of i32 range");
            let offset = offset.try_into().expect("offset is out of i32 range");

            // Safety: this is only safe if the element buffer does not have any
            // elements which are out of bound for one of the vertex buffers.
            // Here, we assume that this is checked by the caller.
            unsafe {
                gl.draw_elements(mode, count, element_type, offset);
            }
        } else {
            // Safety: check vertex buffer sizes.
            let end = first.checked_add(count).unwrap();

            for (buffer, vertex_def) in &self.vertices {
                let num_vertices = buffer.len() / vertex_def.stride;
                assert!(num_vertices >= end);
            }

            let first = first.try_into().expect("first is out of i32 range");
            let count = count.try_into().expect("count is out of i32 range");

            unsafe {
                gl.draw_arrays(mode, first, count);
            }
        }

        // TODO: Remove overly conservative unbinding.
        self.unbind(ctx);

        check_gl_error(gl).expect("OpenGL error in VertexStream::draw()");
    }
}
