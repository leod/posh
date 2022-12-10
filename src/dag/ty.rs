#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum NumericTy {
    Int,
    UInt,
    Float,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum PrimitiveTy {
    Numeric(NumericTy),
    Bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct StructTy {
    pub name: &'static str,
    pub fields: &'static [(&'static str, Ty)],
    pub is_built_in: bool,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum BaseTy {
    Scalar(PrimitiveTy),
    Vec2(PrimitiveTy),
    Vec3(PrimitiveTy),
    Vec4(PrimitiveTy),
    Struct(&'static StructTy),
    Sampler2d(NumericTy),
}

impl BaseTy {
    pub fn is_transparent(&self) -> bool {
        use BaseTy::*;

        match self {
            Sampler2d(_) => false,
            _ => true,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Ty {
    Base(BaseTy),
    Array(BaseTy, usize),
}

impl Ty {
    pub fn is_transparent(&self) -> bool {
        use Ty::*;

        let (Base(ty) | Array(ty, _)) = self;

        ty.is_transparent()
    }
}
