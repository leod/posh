use crevice::std140::AsStd140;
use sealed::sealed;

use crate::{
    dag::{NumericTy, PrimitiveTy},
    Gl, Uniform,
};

/// A primitive type: one of `bool`, `i32`, `u32`, or `f32`.
#[sealed]
pub trait Primitive: Copy + ToString + Uniform<Gl> {
    const PRIMITIVE_TY: PrimitiveTy;

    type Vec2: Uniform<Gl> + AsStd140;
}

#[sealed]
impl Primitive for bool {
    const PRIMITIVE_TY: PrimitiveTy = PrimitiveTy::Bool;

    type Vec2 = mint::Vector2<bool>;
}

#[sealed]
impl Primitive for i32 {
    const PRIMITIVE_TY: PrimitiveTy = PrimitiveTy::Numeric(NumericTy::Int);

    type Vec2 = mint::Vector2<i32>;
}

#[sealed]
impl Primitive for u32 {
    const PRIMITIVE_TY: PrimitiveTy = PrimitiveTy::Numeric(NumericTy::UInt);

    type Vec2 = mint::Vector2<u32>;
}

#[sealed]
impl Primitive for f32 {
    const PRIMITIVE_TY: PrimitiveTy = PrimitiveTy::Numeric(NumericTy::Float);

    type Vec2 = mint::Vector2<f32>;
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
