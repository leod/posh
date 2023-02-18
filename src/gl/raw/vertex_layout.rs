use std::mem::size_of;

use crate::dag::BuiltInType;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum VertexAttributeType {
    F32,
    I32,
    U32,
}

impl VertexAttributeType {
    pub fn size(self) -> usize {
        use VertexAttributeType::*;

        match self {
            F32 => size_of::<f32>(),
            I32 => size_of::<i32>(),
            U32 => size_of::<u32>(),
        }
    }

    pub fn to_gl(self) -> u32 {
        use VertexAttributeType::*;

        match self {
            F32 => glow::FLOAT,
            I32 => glow::INT,
            U32 => glow::UNSIGNED_INT,
        }
    }
}

pub struct VertexAttributeLayout {
    pub ty: VertexAttributeType,
    pub components: usize,
    pub locations: usize,
}

impl VertexAttributeLayout {
    pub fn new(ty: BuiltInType) -> Result<Self, String> {
        use VertexAttributeType::*;

        let (ty, components, locations) = match ty {
            BuiltInType::F32 => (F32, 1, 1),
            BuiltInType::I32 => (I32, 1, 1),
            BuiltInType::U32 => (U32, 1, 1),
            BuiltInType::Bool => (U32, 1, 1),
            BuiltInType::Vec2 => (F32, 2, 1),
            BuiltInType::IVec2 => (I32, 2, 1),
            BuiltInType::UVec2 => (U32, 2, 1),
            BuiltInType::Vec3 => (F32, 3, 1),
            BuiltInType::IVec3 => (I32, 3, 1),
            BuiltInType::UVec3 => (U32, 3, 1),
            BuiltInType::Vec4 => (F32, 4, 1),
            BuiltInType::IVec4 => (I32, 4, 1),
            BuiltInType::UVec4 => (U32, 4, 1),
            BuiltInType::Mat2 => (F32, 2, 2),
            BuiltInType::Mat3 => (F32, 3, 3),
            BuiltInType::Mat4 => (F32, 4, 4),
            BuiltInType::BVec2 | BuiltInType::BVec3 | BuiltInType::BVec4 => {
                return Err("boolean vectors are not supported".to_string())
            }
            BuiltInType::Sampler(_) => return Err("sampler types are not supported".to_string()),
        };

        Ok(VertexAttributeLayout {
            ty,
            components,
            locations,
        })
    }

    pub fn location_size(&self) -> usize {
        self.components * self.ty.size()
    }
}
