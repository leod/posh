use std::mem::size_of;

use crate::dag::{BaseType, NumericType, PrimitiveType, Type};

pub struct VertexAttributeLayout {
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
                Mat2 => Self {
                    ty: NumericType::F32,
                    num_components: 2,
                    num_locations: 2,
                },
                Mat3 => Self {
                    ty: NumericType::F32,
                    num_components: 3,
                    num_locations: 3,
                },
                Mat4 => Self {
                    ty: NumericType::F32,
                    num_components: 4,
                    num_locations: 4,
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

pub fn numeric_type_size(ty: NumericType) -> usize {
    use NumericType::*;

    match ty {
        F32 => size_of::<f32>(),
        I32 => size_of::<i32>(),
        U32 => size_of::<u32>(),
    }
}

pub fn numeric_type_to_gl(ty: NumericType) -> u32 {
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
