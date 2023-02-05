use std::rc::Rc;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum NumericType {
    F32,
    I32,
    U32,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum PrimitiveType {
    Numeric(NumericType),
    Bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StructType {
    pub name: String,
    pub fields: Vec<(String, Type)>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SamplerType {
    Sampler2d { dimension: usize, ty: NumericType },
}

#[derive(Debug, Clone, Eq)]
pub enum BaseType {
    Scalar(PrimitiveType),
    Vec2(PrimitiveType),
    Vec3(PrimitiveType),
    Vec4(PrimitiveType),
    Mat2,
    Mat3,
    Mat4,
    Struct(Rc<StructType>),
    Sampler(SamplerType),
}

impl BaseType {
    pub fn is_mat(&self) -> bool {
        use BaseType::*;

        match self {
            Mat2 => true,
            Mat3 => true,
            Mat4 => true,
            Scalar(_) | Vec2(_) | Vec3(_) | Vec4(_) | Struct(_) | Sampler(_) => false,
        }
    }
}

impl PartialEq for BaseType {
    fn eq(&self, other: &Self) -> bool {
        use BaseType::*;

        match (self, other) {
            (Scalar(a), Scalar(b)) => a == b,
            (Vec2(a), Vec2(b)) => a == b,
            (Vec3(a), Vec3(b)) => a == b,
            (Vec4(a), Vec4(b)) => a == b,
            (Mat2, Mat2) => true,
            (Mat3, Mat3) => true,
            (Mat4, Mat4) => true,
            (Struct(a), Struct(b)) => Rc::ptr_eq(a, b),
            (Sampler(a), Sampler(b)) => a == b,
            _ => false,
        }
    }
}

impl BaseType {
    pub fn is_transparent(&self) -> bool {
        use BaseType::*;

        !matches!(self, Sampler(_))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    Base(BaseType),
    Array(BaseType, usize),
}

impl Type {
    pub fn is_transparent(&self) -> bool {
        use Type::*;

        let (Base(ty) | Array(ty, _)) = self;

        ty.is_transparent()
    }
}
