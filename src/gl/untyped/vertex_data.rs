use std::{mem::size_of, rc::Rc};

use glow::HasContext;

use crate::{
    dag::{BaseType, NumericType, PrimitiveType, Type},
    gl::{CreateVertexDataError, GeometryType},
    VertexAttribute, VertexInputRate,
};

use super::Buffer;

#[derive(Debug, Clone)]
pub struct VertexDataEntryInfo {
    pub input_rate: VertexInputRate,
    pub stride: usize,
    pub attributes: Vec<VertexAttribute>,
}

struct VertexDataShared {
    gl: Rc<glow::Context>,
    id: glow::VertexArray,
    entry_infos: Vec<VertexDataEntryInfo>,

    // Safety: Keep the referenced vertex buffers alive, so that we do not end
    // up with dangling pointers in our vertex array.
    _bindings: Vec<Buffer>,
}

pub struct VertexData {
    shared: Rc<VertexDataShared>,
}

#[derive(Clone)]
pub struct VertexDataBinding {
    shared: Rc<VertexDataShared>,
    geometry_type: GeometryType,
}

impl VertexData {
    /// # Panics
    ///
    /// Panics if any of the buffers do not belong to `gl`, or if any of the
    /// vertex attribute types are not supported by `posh`, or if any of the
    /// buffers have a mismatched size.
    pub fn new(
        gl: Rc<glow::Context>,
        vertex_bindings_and_entry_infos: &[(Buffer, VertexDataEntryInfo)],
    ) -> Result<Self, CreateVertexDataError> {
        // TODO: How do we want to handle `buffers.is_empty()`?

        let id = unsafe { gl.create_vertex_array() }.map_err(CreateVertexDataError)?;

        unsafe {
            gl.bind_vertex_array(Some(id));
        }

        let mut index = 0;

        for (binding, entry_info) in vertex_bindings_and_entry_infos {
            assert!(entry_info.stride > 0);
            assert_eq!(binding.len() % entry_info.stride, 0);
            assert!(Rc::ptr_eq(binding.gl(), &gl));

            unsafe {
                gl.bind_buffer(glow::ARRAY_BUFFER, Some(binding.id()));
            }

            for attribute in &entry_info.attributes {
                let info = VertexDataAttributeInfo::new(&attribute.name, &attribute.ty);

                for i in 0..info.num_locations {
                    use NumericType::*;

                    let data_type = numeric_type_to_gl(info.ty);
                    let offset = attribute.offset + i * info.location_size();

                    assert!(offset + info.location_size() <= entry_info.stride);

                    match info.ty {
                        F32 => unsafe {
                            gl.vertex_attrib_pointer_f32(
                                index,
                                info.num_components as i32,
                                data_type,
                                false,
                                entry_info.stride as i32,
                                offset as i32,
                            )
                        },
                        I32 | U32 => unsafe {
                            gl.vertex_attrib_pointer_i32(
                                index,
                                info.num_components as i32,
                                data_type,
                                entry_info.stride as i32,
                                offset as i32,
                            )
                        },
                    }

                    index += 1;
                }
            }
        }

        unsafe {
            gl.bind_vertex_array(None);
            gl.bind_buffer(glow::ARRAY_BUFFER, None);
        }

        let entry_infos = vertex_bindings_and_entry_infos
            .iter()
            .map(|(_, entry)| entry.clone())
            .collect();

        let bindings = vertex_bindings_and_entry_infos
            .iter()
            .map(|(buffer, _)| buffer.clone())
            .collect();

        let shared = Rc::new(VertexDataShared {
            gl,
            id,
            entry_infos,
            _bindings: bindings,
        });

        Ok(Self { shared })
    }

    pub fn entry_infos(&self) -> &[VertexDataEntryInfo] {
        &self.shared.entry_infos
    }

    pub fn bind(&self, geometry_type: GeometryType) -> VertexDataBinding {
        VertexDataBinding {
            shared: self.shared.clone(),
            geometry_type,
        }
    }
}

impl Drop for VertexDataShared {
    fn drop(&mut self) {
        unsafe {
            self.gl.delete_vertex_array(self.id);
        }
    }
}

pub(crate) struct VertexDataAttributeInfo {
    pub name: String,
    pub ty: NumericType,
    pub num_components: usize,
    pub num_locations: usize,
}

impl VertexDataAttributeInfo {
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
