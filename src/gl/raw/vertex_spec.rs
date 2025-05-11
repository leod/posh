use std::{ops::Range, rc::Rc};

use glow::HasContext;
use smallvec::SmallVec;

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
pub enum PrimitiveMode {
    Points,
    Lines,
    LineStrip,
    LineLoop,
    Triangles,
    TriangleStrip,
    TriangleFan,
}

impl PrimitiveMode {
    pub const fn to_gl(self) -> u32 {
        use PrimitiveMode::*;

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

    pub fn as_vertex_spec_with_range(
        self,
        vertex_range: Range<usize>,
    ) -> crate::gl::VertexSpec<()> {
        crate::gl::VertexSpec::new(self).with_vertex_range(vertex_range)
    }
}

// TODO: VertexBlockDef should eventually be split into program definition and
// vertex stream definition.

#[derive(Clone)]
pub struct VertexBufferBinding {
    pub buffer: Rc<Buffer>,
    pub block_def: VertexBlockDef,
    pub input_rate: VertexInputRate,
    pub stride: usize,
}

pub type VertexBufferBindingVec = SmallVec<[VertexBufferBinding; 8]>;

#[derive(Clone)]
pub struct VertexSpec {
    pub vertex_data: VertexBufferBindingVec,
    pub element_data: Option<(Rc<Buffer>, ElementType)>,
    pub mode: PrimitiveMode,
    pub index_range: Range<usize>,
    pub num_instances: usize,
}

impl VertexSpec {
    pub fn is_compatible(&self, _vertex_block_defs: &[VertexBlockDef]) -> bool {
        // TODO: Check vertex stream compatibility. This is already ensured by
        // the typed interface on top of `raw`, but `raw` should be correct by
        // iself.
        true
    }

    fn bind(&self, ctx: &ContextShared) {
        let mut index = 0;

        let gl = ctx.gl();

        for VertexBufferBinding {
            buffer,
            block_def,
            input_rate,
            stride,
        } in &self.vertex_data
        {
            assert!(*stride > 0);
            assert_eq!(buffer.len() % stride, 0);
            assert!(buffer.context().ref_eq(ctx));

            unsafe {
                gl.bind_buffer(glow::ARRAY_BUFFER, Some(buffer.id()));
            }

            for attribute in &block_def.attributes {
                let attribute_info =
                    VertexAttributeLayout::new(attribute.ty).expect("invalid vertex attribute");

                for i in 0..attribute_info.locations {
                    let offset = attribute.offset + i * attribute_info.location_size();

                    assert!(offset + attribute_info.location_size() <= *stride);

                    unsafe {
                        gl.enable_vertex_attrib_array(index);
                    }

                    let divisor = match input_rate {
                        VertexInputRate::Vertex => 0,
                        VertexInputRate::Instance => 1,
                    };

                    unsafe {
                        gl.vertex_attrib_divisor(index, divisor);
                    }

                    let size = i32::try_from(attribute_info.components).unwrap();
                    let data_type = attribute_info.ty.to_gl();
                    let stride = i32::try_from(*stride).unwrap();
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

        if let Some((buffer, _)) = self.element_data.as_ref() {
            assert!(buffer.context().ref_eq(ctx));

            unsafe {
                gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, Some(buffer.id()));
            }
        }

        #[cfg(debug_assertions)]
        check_gl_error(gl, "").expect("OpenGL error after VertexSpec::bind()");
    }

    fn unbind(&self, ctx: &ContextShared) {
        let gl = ctx.gl();

        let mut index = 0;

        for VertexBufferBinding { block_def, .. } in &self.vertex_data {
            for attribute in &block_def.attributes {
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

        if self.element_data.is_some() {
            unsafe {
                gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, None);
            }
        }

        #[cfg(debug_assertions)]
        check_gl_error(gl, "").expect("OpenGL error after VertexSpec::unbind()");
    }

    pub(super) fn draw(&self, ctx: &ContextShared) {
        if self.index_range.start >= self.index_range.end {
            return;
        }

        if self.num_instances == 0 {
            return;
        }

        let gl = ctx.gl();

        self.bind(ctx);

        let mode = self.mode.to_gl();
        let first = self.index_range.start;
        let count = self.index_range.end - self.index_range.start;

        if let Some((buffer, element_type)) = &self.element_data {
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
            let num_instances = self
                .num_instances
                .try_into()
                .expect("num_instance is out of i32 range");

            for VertexBufferBinding {
                stride,
                buffer,
                input_rate,
                ..
            } in &self.vertex_data
            {
                let num = buffer.len() / stride;
                match input_rate {
                    VertexInputRate::Vertex => {
                        // Safety: this is only safe if the element buffer does not have any
                        // elements which are out of bound for one of the vertex buffers.
                        // Here, we assume that this is checked by the caller.
                    }
                    VertexInputRate::Instance => {
                        assert!(num >= self.num_instances);
                    }
                }
            }

            unsafe {
                gl.draw_elements_instanced(mode, count, element_type, offset, num_instances);
            }
        } else {
            // Safety: check vertex buffer sizes.
            let end = first.checked_add(count).unwrap();

            for VertexBufferBinding {
                stride,
                buffer,
                input_rate,
                ..
            } in &self.vertex_data
            {
                let num = buffer.len() / stride;
                match input_rate {
                    VertexInputRate::Vertex => {
                        assert!(num >= end);
                    }
                    VertexInputRate::Instance => {
                        assert!(num >= self.num_instances);
                    }
                }
            }

            let first = first.try_into().expect("first is out of i32 range");
            let count = count.try_into().expect("count is out of i32 range");
            let num_instances = self
                .num_instances
                .try_into()
                .expect("num_instance is out of i32 range");
            unsafe {
                gl.draw_arrays_instanced(mode, first, count, num_instances);
            }
        }

        #[cfg(debug_assertions)]
        check_gl_error(gl, "").expect("OpenGL error in VertexStream::draw()");

        // TODO: Remove overly conservative unbinding.
        self.unbind(ctx);
    }
}
