use bytemuck::Pod;
use crevice::std140::AsStd140;
use sealed::sealed;

use crate::{
    dag::{NumericTy, PrimitiveTy},
    interface::ToPod,
    Gl, Uniform, Vertex,
};

/// A primitive type: one of `bool`, `f32`, `i32`, `u32`.
#[sealed]
pub trait Primitive: ToPod + ToString + Uniform<Gl> + Vertex<Gl> {
    const PRIMITIVE_TY: PrimitiveTy;

    #[doc(hidden)]
    type Vec2: Uniform<Gl> + Vertex<Gl> + AsStd140 + ToPod;
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

/// A numeric type: one of `f32`, `i32`, `u32`.
#[sealed]
pub trait Numeric: Pod + ToPod + Primitive + Vertex<Gl> {
    const NUMERIC_TY: NumericTy;

    #[doc(hidden)]
    type Vec2: Vertex<Gl> + ToPod;
}

#[sealed]
impl Numeric for f32 {
    const NUMERIC_TY: NumericTy = NumericTy::Float;

    type Vec2 = mint::Vector2<f32>;
}

#[sealed]
impl Numeric for i32 {
    const NUMERIC_TY: NumericTy = NumericTy::Int;

    type Vec2 = mint::Vector2<i32>;
}

#[sealed]
impl Numeric for u32 {
    const NUMERIC_TY: NumericTy = NumericTy::UInt;

    type Vec2 = mint::Vector2<u32>;
}
