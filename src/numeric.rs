use bytemuck::Pod;
use crevice::std140::AsStd140;
use sealed::sealed;

use crate::{
    dag::{NumericType, PrimitiveType},
    interface::ToPod,
    sl::{Scalar, ToValue},
    Gl, Uniform, Vertex,
};

/// A primitive type: one of `bool`, `f32`, `i32`, `u32`.
#[sealed]
pub trait Primitive:
    ToPod + ToString + Uniform<Gl> + Vertex<Gl> + ToValue<Output = Scalar<Self>>
{
    const PRIMITIVE_TYPE: PrimitiveType;

    #[doc(hidden)]
    type Vec2: Uniform<Gl> + Vertex<Gl> + AsStd140 + ToPod + ToValue;
}

#[sealed]
impl Primitive for bool {
    const PRIMITIVE_TYPE: PrimitiveType = PrimitiveType::Bool;

    type Vec2 = mint::Vector2<bool>;
}

#[sealed]
impl Primitive for i32 {
    const PRIMITIVE_TYPE: PrimitiveType = PrimitiveType::Numeric(NumericType::Int);

    type Vec2 = mint::Vector2<i32>;
}

#[sealed]
impl Primitive for u32 {
    const PRIMITIVE_TYPE: PrimitiveType = PrimitiveType::Numeric(NumericType::UInt);

    type Vec2 = mint::Vector2<u32>;
}

#[sealed]
impl Primitive for f32 {
    const PRIMITIVE_TYPE: PrimitiveType = PrimitiveType::Numeric(NumericType::Float);

    type Vec2 = mint::Vector2<f32>;
}

/// A numeric type: one of `f32`, `i32`, `u32`.
#[sealed]
pub trait Numeric: Pod + ToPod + Primitive + Vertex<Gl> {
    const NUMERIC_TYPE: NumericType;

    #[doc(hidden)]
    type Vec2: Vertex<Gl> + ToPod;
}

#[sealed]
impl Numeric for f32 {
    const NUMERIC_TYPE: NumericType = NumericType::Float;

    type Vec2 = mint::Vector2<f32>;
}

#[sealed]
impl Numeric for i32 {
    const NUMERIC_TYPE: NumericType = NumericType::Int;

    type Vec2 = mint::Vector2<i32>;
}

#[sealed]
impl Numeric for u32 {
    const NUMERIC_TYPE: NumericType = NumericType::UInt;

    type Vec2 = mint::Vector2<u32>;
}
