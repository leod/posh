#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum NumericType {
    Int,
    UInt,
    Float,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum PrimitiveType {
    Numeric(NumericType),
    Bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct StructType {
    pub name: &'static str,
    pub fields: &'static [(&'static str, Type)],
    pub is_built_in: bool,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum BaseType {
    Scalar(PrimitiveType),
    Vec2(PrimitiveType),
    Vec3(PrimitiveType),
    Vec4(PrimitiveType),
    Struct(&'static StructType),
    Sampler2d(NumericType),
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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
