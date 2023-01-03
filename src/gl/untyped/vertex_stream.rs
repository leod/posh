use std::{mem::size_of, rc::Rc};

use glow::HasContext;

use crate::{
    dag::{BaseType, NumericType, PrimitiveType, Type},
    gl::{CreateVertexStreamError, ElementType},
    VertexAttribute, VertexInputRate,
};

use super::Buffer;

#[derive(Debug, Clone)]
pub struct VertexStreamBufferInfo {
    pub input_rate: VertexInputRate,
    pub stride: usize,
    pub attributes: Vec<VertexAttribute>,
}

struct VertexStreamShared {
    gl: Rc<glow::Context>,
    id: glow::VertexArray,
    vertex_infos: Vec<VertexStreamBufferInfo>,
    element_type: Option<ElementType>,

    // Safety: Keep the referenced vertex buffers alive, so that we do not end
    // up with dangling pointers in our vertex array.
    _vertex_buffers: Vec<Buffer>,
    _element_buffer: Option<Buffer>,
}

pub struct VertexStream {
    shared: Rc<VertexStreamShared>,
}

impl VertexStream {
    /// # Panics
    ///
    /// Panics if any of the buffers do not belong to `gl`, or if any of the
    /// vertex attribute types are not supported by `posh`, or if any of the
    /// buffers have a mismatched size.
    pub fn new(
        gl: Rc<glow::Context>,
        vertex_buffers: &[(Buffer, VertexStreamBufferInfo)],
        element_buffer: Option<(Buffer, ElementType)>,
    ) -> Result<Self, CreateVertexStreamError> {
        // TODO: How do we want to handle `buffers.is_empty()`?

        let id = unsafe { gl.create_vertex_array() }.map_err(CreateVertexStreamError)?;

        unsafe {
            gl.bind_vertex_array(Some(id));
        }

        let mut index = 0;

        for (buffer, buffer_info) in vertex_buffers {
            assert!(buffer_info.stride > 0);
            assert_eq!(buffer.len() % buffer_info.stride, 0);
            assert!(Rc::ptr_eq(buffer.gl(), &gl));

            unsafe {
                gl.bind_buffer(glow::ARRAY_BUFFER, Some(buffer.id()));
            }

            for attribute in &buffer_info.attributes {
                let info = VertexBindingAttributeInfo::new(&attribute.name, &attribute.ty);

                for i in 0..info.num_locations {
                    use NumericType::*;

                    let data_type = numeric_type_to_gl(info.ty);
                    let offset = attribute.offset + i * info.location_size();

                    assert!(offset + info.location_size() <= buffer_info.stride);

                    match info.ty {
                        F32 => unsafe {
                            gl.vertex_attrib_pointer_f32(
                                index,
                                info.num_components as i32,
                                data_type,
                                false,
                                buffer_info.stride as i32,
                                offset as i32,
                            )
                        },
                        I32 | U32 => unsafe {
                            gl.vertex_attrib_pointer_i32(
                                index,
                                info.num_components as i32,
                                data_type,
                                buffer_info.stride as i32,
                                offset as i32,
                            )
                        },
                    }

                    index += 1;
                }
            }
        }

        let (element_buffer, element_type) = if let Some((buffer, ty)) = element_buffer {
            unsafe {
                gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, Some(buffer.id()));
            }

            (Some(buffer), Some(ty))
        } else {
            (None, None)
        };

        unsafe {
            gl.bind_vertex_array(None);
            gl.bind_buffer(glow::ARRAY_BUFFER, None);
        }

        let vertex_infos = vertex_buffers
            .iter()
            .map(|(_, entry)| entry.clone())
            .collect();

        let vertex_buffers = vertex_buffers
            .iter()
            .map(|(buffer, _)| buffer.clone())
            .collect();

        let shared = Rc::new(VertexStreamShared {
            gl,
            id,
            vertex_infos,
            element_type,
            _vertex_buffers: vertex_buffers,
            _element_buffer: element_buffer,
        });

        Ok(Self { shared })
    }

    pub fn vertex_infos(&self) -> &[VertexStreamBufferInfo] {
        &self.shared.vertex_infos
    }

    pub fn element_type(&self) -> Option<ElementType> {
        self.shared.element_type
    }
}

impl Drop for VertexStreamShared {
    fn drop(&mut self) {
        unsafe {
            self.gl.delete_vertex_array(self.id);
        }
    }
}

pub(crate) struct VertexBindingAttributeInfo {
    pub name: String,
    pub ty: NumericType,
    pub num_components: usize,
    pub num_locations: usize,
}

impl VertexBindingAttributeInfo {
    pub fn new(name: &str, ty: &Type) -> Self {
        use BaseType::*;
        use Type::*;

        match ty {
            Base(ty) => match ty {
                Scalar(ty) => Self {
                    name: name.to_string(),
                    ty: get_numeric_type(*ty),
                    num_components: 1,
                    num_locations: 1,
                },
                Vec2(ty) => Self {
                    name: name.to_string(),
                    ty: get_numeric_type(*ty),
                    num_components: 2,
                    num_locations: 1,
                },
                Vec3(ty) => Self {
                    name: name.to_string(),
                    ty: get_numeric_type(*ty),
                    num_components: 3,
                    num_locations: 1,
                },
                Vec4(ty) => Self {
                    name: name.to_string(),
                    ty: get_numeric_type(*ty),
                    num_components: 4,
                    num_locations: 1,
                },
                Struct(_) => panic!("`VertexData` does not support struct types"),
                Sampler2d(_) => panic!("`VertexData` does not support sampler types"),
            },
            Array(_, _) => todo!(),
        }
    }

    pub fn location_size(&self) -> usize {
        self.num_components * numeric_type_size(self.ty)
    }
}

fn numeric_type_size(ty: NumericType) -> usize {
    use NumericType::*;

    match ty {
        F32 => size_of::<f32>(),
        I32 => size_of::<i32>(),
        U32 => size_of::<u32>(),
    }
}

fn numeric_type_to_gl(ty: NumericType) -> u32 {
    use NumericType::*;

    match ty {
        F32 => glow::FLOAT,
        I32 => glow::INT,
        U32 => glow::UNSIGNED_INT,
    }
}

fn get_numeric_type(ty: PrimitiveType) -> NumericType {
    use PrimitiveType::*;

    match ty {
        Numeric(ty) => ty,
        Bool => panic!("`VertexData` does not support `bool`"),
    }
}
