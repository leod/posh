use sealed::sealed;

use crate::dag::{NumericTy, PrimitiveTy};

/// A primitive type: one of `bool`, `i32`, `u32`, or `f32`.
#[sealed]
pub trait Primitive: Copy + ToString {
    const PRIMITIVE_TY: PrimitiveTy;
}

#[sealed]
impl Primitive for bool {
    const PRIMITIVE_TY: PrimitiveTy = PrimitiveTy::Bool;
}

#[sealed]
impl Primitive for i32 {
    const PRIMITIVE_TY: PrimitiveTy = PrimitiveTy::Numeric(NumericTy::Int);
}

#[sealed]
impl Primitive for u32 {
    const PRIMITIVE_TY: PrimitiveTy = PrimitiveTy::Numeric(NumericTy::UInt);
}

#[sealed]
impl Primitive for f32 {
    const PRIMITIVE_TY: PrimitiveTy = PrimitiveTy::Numeric(NumericTy::Float);
}

/// A numeric type: one of `i32`, `u32`, or `f32`.
#[sealed]
pub trait Numeric: Primitive {
    const NUMERIC_TY: NumericTy;
}

#[sealed]
impl Numeric for i32 {
    const NUMERIC_TY: NumericTy = NumericTy::Int;
}

#[sealed]
impl Numeric for u32 {
    const NUMERIC_TY: NumericTy = NumericTy::UInt;
}

#[sealed]
impl Numeric for f32 {
    const NUMERIC_TY: NumericTy = NumericTy::Float;
}
