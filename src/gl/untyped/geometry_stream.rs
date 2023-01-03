use std::ops::Range;

use glow::HasContext;

use crate::gl::GeometryType;

use super::VertexArray;

// TODO: Instancing.

pub struct GeometryStream {
    vertex_array: VertexArray,
    element_range: Range<usize>,
    geometry_type: GeometryType,
}

impl GeometryStream {
    /// # Panics
    ///
    /// TODO
    ///
    /// # Safety
    ///
    /// TODO
    pub(crate) unsafe fn draw(&self) {
        assert!(self.element_range.start <= self.element_range.end);

        let gl = self.vertex_array.gl();
        let mode = self.geometry_type.to_gl();
        let first = self.element_range.start;
        let count = self.element_range.end - self.element_range.start;

        unsafe {
            gl.bind_vertex_array(Some(self.vertex_array.id()));
        }

        if let Some((element_buffer, element_type)) = self.vertex_array.element_buffer() {
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
            for (buffer, vertex_info) in self.vertex_array.vertex_buffers() {
                let num_vertices = buffer.len() / vertex_info.stride;

                assert!(first.checked_add(count).unwrap() <= num_vertices);
            }

            let first = i32::try_from(first).unwrap();
            let count = i32::try_from(count).unwrap();

            unsafe {
                gl.draw_arrays(mode, first, count);
            }
        }

        unsafe {
            gl.bind_vertex_array(None);
        }
    }
}
