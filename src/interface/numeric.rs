use bytemuck::Pod;
use crevice::std140::AsStd140;
use sealed::sealed;

use crate::{
    dag::{NumericType, PrimitiveType},
    sl::{Scalar, ToValue, Vec2, Vec3, Vec4},
    Block, Physical,
};

/// One of `bool`, `f32`, `i32`, or `u32`.
#[sealed]
pub trait Primitive:
    'static
    + AsStd140
    + Default
    + ToString
    + Block<Physical, Physical = Self, Logical = Scalar<Self>>
    + ToValue<Output = Scalar<Self>>
{
    #[doc(hidden)]
    const PRIMITIVE_TYPE: PrimitiveType;

    #[doc(hidden)]
    type Vec2: Block<Physical, Physical = Self::Vec2, Logical = Vec2<Self>>
        + AsStd140
        + ToValue<Output = Vec2<Self>>;

    #[doc(hidden)]
    type Vec3: Block<Physical, Physical = Self::Vec3, Logical = Vec3<Self>>
        + AsStd140
        + ToValue<Output = Vec3<Self>>;

    #[doc(hidden)]
    type Vec4: Block<Physical, Physical = Self::Vec4, Logical = Vec4<Self>>
        + AsStd140
        + ToValue<Output = Vec4<Self>>;
}

macro_rules! impl_primitive {
    ($ty:ty, $prim:expr) => {
        #[sealed]
        impl Primitive for $ty {
            const PRIMITIVE_TYPE: PrimitiveType = $prim;

            type Vec2 = mint::Vector2<$ty>;
            type Vec3 = mint::Vector3<$ty>;
            type Vec4 = mint::Vector4<$ty>;
        }
    };
}

impl_primitive!(bool, PrimitiveType::Bool);
impl_primitive!(i32, PrimitiveType::Numeric(NumericType::I32));
impl_primitive!(u32, PrimitiveType::Numeric(NumericType::U32));
impl_primitive!(f32, PrimitiveType::Numeric(NumericType::F32));

/// One of `f32`, `i32`, or `u32`.
#[sealed]
pub trait Numeric: Pod + Primitive {
    #[doc(hidden)]
    const NUMERIC_TYPE: NumericType;

    #[doc(hidden)]
    type Vec2: Block<Physical>;
}

#[sealed]
impl Numeric for f32 {
    const NUMERIC_TYPE: NumericType = NumericType::F32;

    type Vec2 = mint::Vector2<f32>;
}

#[sealed]
impl Numeric for i32 {
    const NUMERIC_TYPE: NumericType = NumericType::I32;

    type Vec2 = mint::Vector2<i32>;
}

#[sealed]
impl Numeric for u32 {
    const NUMERIC_TYPE: NumericType = NumericType::U32;

    type Vec2 = mint::Vector2<u32>;
}
