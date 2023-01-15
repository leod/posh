use bytemuck::{Pod, Zeroable};
use crevice::std140::AsStd140;
use sealed::sealed;

use crate::{
    dag::{NumericType, PrimitiveType},
    interface::ToPod,
    sl::{Scalar, ToValue, Vec2, Vec3, Vec4},
    Gl, Uniform, Vertex,
};

/// One of `bool`, `f32`, `i32`, or `u32`.
#[sealed]
pub trait Primitive:
    'static
    + AsStd140
    + Default
    + ToPod
    + ToString
    + Uniform<Gl, InGl = Self, InSl = Scalar<Self>>
    + Vertex<Gl, InGl = Self, InSl = Scalar<Self>>
    + ToValue<Output = Scalar<Self>>
{
    #[doc(hidden)]
    const PRIMITIVE_TYPE: PrimitiveType;

    #[doc(hidden)]
    type Vec2: Uniform<Gl, InGl = Self::Vec2, InSl = Vec2<Self>>
        + Vertex<Gl, InGl = Self::Vec2, InSl = Vec2<Self>>
        + AsStd140
        + ToPod
        + ToValue<Output = Vec2<Self>>;

    #[doc(hidden)]
    type Vec3: Uniform<Gl, InGl = Self::Vec3, InSl = Vec3<Self>>
        + Vertex<Gl, InGl = Self::Vec3, InSl = Vec3<Self>>
        + AsStd140
        + ToPod
        + ToValue<Output = Vec3<Self>>;

    #[doc(hidden)]
    type Vec4: Uniform<Gl, InGl = Self::Vec4, InSl = Vec4<Self>>
        + Vertex<Gl, InGl = Self::Vec4, InSl = Vec4<Self>>
        + AsStd140
        + ToPod
        + ToValue<Output = Vec4<Self>>;
}

#[sealed]
impl Primitive for bool {
    const PRIMITIVE_TYPE: PrimitiveType = PrimitiveType::Bool;

    type Vec2 = mint::Vector2<bool>;
    type Vec3 = mint::Vector3<bool>;
    type Vec4 = mint::Vector4<bool>;
}

#[sealed]
impl Primitive for i32 {
    const PRIMITIVE_TYPE: PrimitiveType = PrimitiveType::Numeric(NumericType::I32);

    type Vec2 = mint::Vector2<i32>;
    type Vec3 = mint::Vector3<i32>;
    type Vec4 = mint::Vector4<i32>;
}

#[sealed]
impl Primitive for u32 {
    const PRIMITIVE_TYPE: PrimitiveType = PrimitiveType::Numeric(NumericType::U32);

    type Vec2 = mint::Vector2<u32>;
    type Vec3 = mint::Vector3<u32>;
    type Vec4 = mint::Vector4<u32>;
}

#[sealed]
impl Primitive for f32 {
    const PRIMITIVE_TYPE: PrimitiveType = PrimitiveType::Numeric(NumericType::F32);

    type Vec2 = mint::Vector2<f32>;
    type Vec3 = mint::Vector3<f32>;
    type Vec4 = mint::Vector4<f32>;
}

/// One of `f32`, `i32`, or `u32`.
#[sealed]
pub trait Numeric: Pod + ToPod + Primitive + Vertex<Gl> {
    #[doc(hidden)]
    const NUMERIC_TYPE: NumericType;

    #[doc(hidden)]
    type Vec2: Vertex<Gl> + ToPod;
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

#[derive(Clone, Copy, Eq, PartialEq, Pod, Zeroable)]
#[repr(transparent)]
pub struct BoolField(u32);

unsafe impl ToPod for BoolField {
    type Output = Self;

    fn to_pod(self) -> Self::Output {
        self
    }
}

unsafe impl ToPod for bool {
    type Output = BoolField;

    fn to_pod(self) -> Self::Output {
        BoolField(self as u32)
    }
}

unsafe impl ToPod for f32 {
    type Output = Self;

    fn to_pod(self) -> Self::Output {
        self
    }
}

unsafe impl ToPod for i32 {
    type Output = Self;

    fn to_pod(self) -> Self::Output {
        self
    }
}

unsafe impl ToPod for u32 {
    type Output = Self;

    fn to_pod(self) -> Self::Output {
        self
    }
}

unsafe impl<T: Pod + ToPod> ToPod for [T; 2] {
    type Output = Self;

    fn to_pod(self) -> Self::Output {
        self
    }
}

unsafe impl<T: Primitive> ToPod for mint::Vector2<T> {
    type Output = [<T as ToPod>::Output; 2];

    fn to_pod(self) -> Self::Output {
        [self.x.to_pod(), self.y.to_pod()]
    }
}

unsafe impl<T: Primitive> ToPod for mint::Vector3<T> {
    type Output = [<T as ToPod>::Output; 3];

    fn to_pod(self) -> Self::Output {
        [self.x.to_pod(), self.y.to_pod(), self.z.to_pod()]
    }
}

unsafe impl<T: Primitive> ToPod for mint::Vector4<T> {
    type Output = [<T as ToPod>::Output; 4];

    fn to_pod(self) -> Self::Output {
        [
            self.x.to_pod(),
            self.y.to_pod(),
            self.z.to_pod(),
            self.w.to_pod(),
        ]
    }
}
