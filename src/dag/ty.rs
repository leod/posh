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

#[derive(Debug, Clone, Eq)]
pub enum BaseType {
    Scalar(PrimitiveType),
    Vec2(PrimitiveType),
    Vec3(PrimitiveType),
    Vec4(PrimitiveType),
    Struct(Rc<StructType>),
    Sampler2d(NumericType),
}

impl PartialEq for BaseType {
    fn eq(&self, other: &Self) -> bool {
        use BaseType::*;

        match (self, other) {
            (Scalar(a), Scalar(b)) => a == b,
            (Vec2(a), Vec2(b)) => a == b,
            (Vec3(a), Vec3(b)) => a == b,
            (Vec4(a), Vec4(b)) => a == b,
            (Struct(a), Struct(b)) => Rc::ptr_eq(a, b),
            (Sampler2d(a), Sampler2d(b)) => a == b,
            _ => false,
        }
    }
}

impl BaseType {
    pub fn is_transparent(&self) -> bool {
        use BaseType::*;

        match self {
            Sampler2d(_) => false,
            _ => true,
        }
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
