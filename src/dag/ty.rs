use std::rc::Rc;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum SamplerType {
    Sampler2d,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum BuiltInType {
    F32,
    I32,
    U32,
    Bool,
    Vec2,
    IVec2,
    UVec2,
    BVec2,
    Vec3,
    IVec3,
    UVec3,
    BVec3,
    Vec4,
    IVec4,
    UVec4,
    BVec4,
    Mat2,
    Mat3,
    Mat4,
    Sampler(SamplerType),
}

impl BuiltInType {
    pub fn is_mat(&self) -> bool {
        use BuiltInType::*;

        match self {
            Mat2 => true,
            Mat3 => true,
            Mat4 => true,
            _ => false,
        }
    }
}

impl BuiltInType {
    pub fn is_transparent(&self) -> bool {
        use BuiltInType::*;

        match self {
            Sampler(_) => false,
            _ => true,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StructType {
    pub name: String,
    pub fields: Vec<(String, Type)>,
}

#[derive(Debug, Clone, Eq)]
pub enum Type {
    BuiltIn(BuiltInType),
    Struct(Rc<StructType>),
    Array(Box<Type>, usize),
}

impl Type {
    pub fn is_transparent(&self) -> bool {
        use Type::*;

        match self {
            BuiltIn(ty) => ty.is_transparent(),
            Struct(_) => true,
            Array(ty, _) => ty.is_transparent(),
        }
    }

    pub fn built_in_type(&self) -> Option<BuiltInType> {
        use Type::*;

        match self {
            BuiltIn(ty) => Some(ty.clone()),
            _ => None,
        }
    }
}

impl PartialEq for Type {
    fn eq(&self, other: &Self) -> bool {
        use Type::*;

        match (self, other) {
            (BuiltIn(a), BuiltIn(b)) => a == b,
            (Struct(a), Struct(b)) => Rc::ptr_eq(a, b),
            (Array(a, n), Array(b, m)) => a == b && n == m,
            _ => false,
        }
    }
}
