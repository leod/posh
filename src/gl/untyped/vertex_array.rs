use std::{mem::size_of, ops::Range, rc::Rc};

use glow::HasContext;

use crate::{
    dag::{BaseType, NumericType, PrimitiveType, Type},
    gl::{CreateVertexArrayError, ElementType, GeometryType},
    VertexAttribute, VertexInputRate,
};

use super::{Buffer, GeometryStream};

#[derive(Debug, Clone)]
pub struct VertexInfo {
    pub input_rate: VertexInputRate,
    pub stride: usize,
    pub attributes: Vec<VertexAttribute>,
}

struct VertexArrayShared {
    gl: Rc<glow::Context>,
    id: glow::VertexArray,
    vertex_buffers: Vec<(Buffer, VertexInfo)>,
    element_buffer: Option<(Buffer, ElementType)>,
}

#[derive(Clone)]
pub struct VertexArray {
    shared: Rc<VertexArrayShared>,
}

impl VertexArray {
    /// # Panics
    ///
    /// Panics if any of the buffers do not belong to `gl`, or if any of the
    /// vertex attribute types are not supported by `posh`, or if any of the
    /// buffers have a mismatched size.
    pub fn new(
        gl: Rc<glow::Context>,
        vertex_buffers: &[(Buffer, VertexInfo)],
        element_buffer: Option<(Buffer, ElementType)>,
    ) -> Result<Self, CreateVertexArrayError> {
        // TODO: How do we want to handle `buffers.is_empty()`?

        let id = unsafe { gl.create_vertex_array() }.map_err(CreateVertexArrayError)?;

        unsafe {
            gl.bind_vertex_array(Some(id));
        }

        let mut index = 0;

        for (buffer, vertex_info) in vertex_buffers {
            assert!(vertex_info.stride > 0);
            assert_eq!(buffer.len() % vertex_info.stride, 0);
            assert!(Rc::ptr_eq(buffer.gl(), &gl));

            unsafe {
                gl.bind_buffer(glow::ARRAY_BUFFER, Some(buffer.id()));
            }

            for attribute in &vertex_info.attributes {
                let attribute_info = VertexAttributeLayout::new(&attribute.ty);

                for i in 0..attribute_info.num_locations {
                    use NumericType::*;
                    use VertexInputRate::*;

                    let data_type = numeric_type_to_gl(attribute_info.ty);
                    let offset = attribute.offset + i * attribute_info.location_size();

                    assert!(offset + attribute_info.location_size() <= vertex_info.stride);

                    unsafe {
                        gl.enable_vertex_attrib_array(index);
                    }

                    match vertex_info.input_rate {
                        Vertex => (),
                        Instance => unsafe {
                            gl.vertex_attrib_divisor(index, 1);
                        },
                    }

                    match attribute_info.ty {
                        F32 => unsafe {
                            gl.vertex_attrib_pointer_f32(
                                index,
                                i32::try_from(attribute_info.num_components).unwrap(),
                                data_type,
                                false,
                                i32::try_from(vertex_info.stride).unwrap(),
                                i32::try_from(offset).unwrap(),
                            )
                        },
                        I32 | U32 => unsafe {
                            gl.vertex_attrib_pointer_i32(
                                index,
                                i32::try_from(attribute_info.num_components).unwrap(),
                                data_type,
                                i32::try_from(vertex_info.stride).unwrap(),
                                i32::try_from(offset).unwrap(),
                            )
                        },
                    }

                    index += 1;
                }
            }
        }

        if let Some((buffer, ty)) = element_buffer.as_ref() {
            unsafe {
                gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, Some(buffer.id()));
            }
        };

        unsafe {
            gl.bind_vertex_array(None);
            gl.bind_buffer(glow::ARRAY_BUFFER, None);
        }

        let vertex_buffers = vertex_buffers.to_vec();

        let shared = Rc::new(VertexArrayShared {
            gl,
            id,
            vertex_buffers,
            element_buffer,
        });

        Ok(Self { shared })
    }

    pub fn gl(&self) -> &Rc<glow::Context> {
        &self.shared.gl
    }

    pub fn id(&self) -> glow::VertexArray {
        self.shared.id
    }

    pub fn vertex_buffers(&self) -> &[(Buffer, VertexInfo)] {
        &self.shared.vertex_buffers
    }

    pub fn element_buffer(&self) -> Option<&(Buffer, ElementType)> {
        self.shared.element_buffer.as_ref()
    }

    pub fn stream(
        &self,
        element_range: Range<usize>,
        geometry_type: GeometryType,
    ) -> GeometryStream {
        GeometryStream {
            vertex_array: self.clone(),
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

pub(crate) struct VertexAttributeLayout {
    pub ty: NumericType,
    pub num_components: usize,
    pub num_locations: usize,
}

impl VertexAttributeLayout {
    pub fn new(ty: &Type) -> Self {
        use BaseType::*;
        use Type::*;

        match ty {
            Base(ty) => match ty {
                Scalar(ty) => Self {
                    ty: get_numeric_type(*ty),
                    num_components: 1,
                    num_locations: 1,
                },
                Vec2(ty) => Self {
                    ty: get_numeric_type(*ty),
                    num_components: 2,
                    num_locations: 1,
                },
                Vec3(ty) => Self {
                    ty: get_numeric_type(*ty),
                    num_components: 3,
                    num_locations: 1,
                },
                Vec4(ty) => Self {
                    ty: get_numeric_type(*ty),
                    num_components: 4,
                    num_locations: 1,
                },
                Struct(_) => panic!("`VertexArray` does not support struct types"),
                Sampler2d(_) => panic!("`VertexArray` does not support sampler types"),
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
        Bool => panic!("`VertexArray` does not support `bool`"),
    }
}
