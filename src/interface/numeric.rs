use bytemuck::Pod;
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

unsafe impl ToPod for bool {
    type Output = u32;

    fn to_pod(self) -> Self::Output {
        self as u32
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
