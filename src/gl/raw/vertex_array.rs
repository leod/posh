use std::{ops::Range, rc::Rc};

use glow::HasContext;

use crate::{
    gl::{
        raw::vertex_layout::{VertexAttributeLayout, VertexAttributeType},
        VertexArrayError,
    },
    program_def::{VertexDef, VertexInputRate},
};

use super::{buffer::BufferShared, error::check_gl_error, Buffer};

#[derive(Debug, Copy, Clone)]
pub enum ElementType {
    U16,
    U32,
}

impl ElementType {
    pub const fn to_gl(self) -> u32 {
        use ElementType::*;

        match self {
            U16 => glow::UNSIGNED_SHORT,
            U32 => glow::UNSIGNED_INT,
        }
    }

    pub const fn size(self) -> usize {
        use ElementType::*;

        match self {
            U16 => 2,
            U32 => 4,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum GeometryType {
    Points,
    Lines,
    LineStrip,
    LineLoop,
    Triangles,
    TriangleStrip,
    TriangleFan,
}

struct VertexArrayShared {
    gl: Rc<glow::Context>,
    id: glow::VertexArray,
    vertex_buffers: Vec<(Rc<BufferShared>, VertexDef)>,
    element_buffer: Option<(Rc<BufferShared>, ElementType)>,
}

pub struct VertexArray {
    shared: Rc<VertexArrayShared>,
}

impl VertexArray {
    /// # Panics
    ///
    /// Panics if any of the buffers do not belong to `gl`, or if any of the
    /// vertex attribute types are not supported by `posh`, or if any of the
    /// buffers have a mismatched size.
    pub(super) fn new(
        gl: Rc<glow::Context>,
        vertex_buffers: &[(&Buffer, VertexDef)],
        element_buffer: Option<(&Buffer, ElementType)>,
    ) -> Result<Self, VertexArrayError> {
        // TODO: How do we want to handle `buffers.is_empty()`?

        let id = unsafe { gl.create_vertex_array() }.map_err(VertexArrayError::ObjectCreation)?;

        unsafe {
            gl.bind_vertex_array(Some(id));
        }

        let mut index = 0;

        for (vertex_buffer, vertex_def) in vertex_buffers {
            assert!(vertex_def.stride > 0);
            assert_eq!(vertex_buffer.len() % vertex_def.stride, 0);
            assert!(Rc::ptr_eq(vertex_buffer.gl(), &gl));

            unsafe {
                gl.bind_buffer(glow::ARRAY_BUFFER, Some(vertex_buffer.id()));
            }

            for attribute in &vertex_def.attributes {
                let attribute_info = VertexAttributeLayout::new(attribute.ty)
                    .map_err(VertexArrayError::InvalidVertexAttribute)?;

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

                    match attribute_info.ty {
                        VertexAttributeType::F32 => unsafe {
                            gl.vertex_attrib_pointer_f32(
                                index,
                                i32::try_from(attribute_info.components).unwrap(),
                                attribute_info.ty.to_gl(),
                                false,
                                i32::try_from(vertex_def.stride).unwrap(),
                                i32::try_from(offset).unwrap(),
                            )
                        },
                        VertexAttributeType::I32 | VertexAttributeType::U32 => unsafe {
                            gl.vertex_attrib_pointer_i32(
                                index,
                                i32::try_from(attribute_info.components).unwrap(),
                                attribute_info.ty.to_gl(),
                                i32::try_from(vertex_def.stride).unwrap(),
                                i32::try_from(offset).unwrap(),
                            )
                        },
                    }

                    index += 1;
                }
            }
        }

        if let Some((element_buffer, _)) = element_buffer.as_ref() {
            unsafe {
                gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, Some(element_buffer.id()));
            }
        };

        unsafe {
            gl.bind_vertex_array(None);
            gl.bind_buffer(glow::ARRAY_BUFFER, None);
        }

        check_gl_error(&gl).map_err(VertexArrayError::Unexpected)?;

        let vertex_buffers = vertex_buffers
            .iter()
            .map(|(vertex_buffer, vertex_def)| (vertex_buffer.shared.clone(), vertex_def.clone()))
            .collect();

        let element_buffer = element_buffer
            .map(|(element_buffer, element_type)| (element_buffer.shared.clone(), element_type));

        let shared = Rc::new(VertexArrayShared {
            gl,
            id,
            vertex_buffers,
            element_buffer,
        });

        Ok(Self { shared })
    }

    pub fn range_binding(
        &self,
        element_range: Range<usize>,
        geometry_type: GeometryType,
    ) -> VertexArrayBinding {
        VertexArrayBinding {
            vertex_array: self.shared.clone(),
            element_range,
            geometry_type,
        }
    }
}

impl Drop for VertexArrayShared {
    fn drop(&mut self) {
        unsafe {
            self.gl.delete_vertex_array(self.id);
        }
    }
}

impl GeometryType {
    pub const fn to_gl(self) -> u32 {
        use GeometryType::*;

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

pub struct VertexArrayBinding {
    vertex_array: Rc<VertexArrayShared>,
    element_range: Range<usize>,
    geometry_type: GeometryType,
}

impl VertexArrayBinding {
    pub fn gl(&self) -> &Rc<glow::Context> {
        &self.vertex_array.gl
    }

    /// TODO: Instancing.
    ///
    /// # Panics
    ///
    /// TODO
    ///
    /// # Safety
    ///
    /// TODO
    pub unsafe fn draw(&self) {
        assert!(self.element_range.start <= self.element_range.end);

        let gl = &self.vertex_array.gl;
        let mode = self.geometry_type.to_gl();
        let first = self.element_range.start;
        let count = self.element_range.end - self.element_range.start;

        unsafe {
            gl.bind_vertex_array(Some(self.vertex_array.id));
        }

        if let Some((element_buffer, element_type)) = &self.vertex_array.element_buffer {
            let element_size = element_type.size();
            let element_type = element_type.to_gl();

            let offset = first.checked_mul(element_size).unwrap();

            // Safety: check element range.
            {
                let size = count.checked_mul(element_size).unwrap();

                assert!(offset.checked_add(size).unwrap() <= element_buffer.len());
            }

            let count = i32::try_from(count).unwrap();
            let offset = i32::try_from(offset).unwrap();

            // Safety: this is only safe if the element buffer does not have any
            // elements which are out of bound for one of the vertex buffers.
            // Here, we assume that this is checked by the caller.
            unsafe {
                gl.draw_elements(mode, count, element_type, offset);
            }
        } else {
            // Safety: check vertex buffer sizes.
            let end = first.checked_add(count).unwrap();

            for (buffer, vertex_info) in &self.vertex_array.vertex_buffers {
                let num_vertices = buffer.len() / vertex_info.stride;

                assert!(num_vertices >= end);
            }

            let first = i32::try_from(first).unwrap();
            let count = i32::try_from(count).unwrap();

            unsafe {
                gl.draw_arrays(mode, first, count);
            }
        }

        // TODO: Remove overly conservative unbinding.
        unsafe {
            gl.bind_vertex_array(None);
        }

        check_gl_error(&gl).unwrap();
    }
}
