use sealed::sealed;

use crate::{IntoPosh, Posh};

use super::{built_in1, built_in2, Value, Vec2, Vec3, Vec4};

/// A representative of a `f32`-based scalar or vector type.
#[sealed]
pub trait GenValue: Value + Sized {
    fn normalize(self) -> Self {
        built_in1("normalize", self)
    }

    fn sin(self) -> Self {
        built_in1("sin", self)
    }

    fn cos(self) -> Self {
        built_in1("cos", self)
    }

    fn tan(self) -> Self {
        built_in1("tan", self)
    }

    fn asin(self) -> Self {
        built_in1("asin", self)
    }

    fn acos(self) -> Self {
        built_in1("acos", self)
    }

    fn atan2(self, x: impl IntoPosh<Rep = Self>) -> Self {
        built_in2("atan", self, x)
    }

    fn atan(self) -> Self {
        built_in1("atan", self)
    }

    fn sinh(self) -> Self {
        built_in1("sinh", self)
    }

    fn cosh(self) -> Self {
        built_in1("cosh", self)
    }

    fn tanh(self) -> Self {
        built_in1("tanh", self)
    }

    fn asinh(self) -> Self {
        built_in1("asinh", self)
    }

    fn acosh(self) -> Self {
        built_in1("acosh", self)
    }

    fn atanh(self) -> Self {
        built_in1("atanh", self)
    }
}

#[sealed]
impl GenValue for Posh<f32> {}

#[sealed]
impl GenValue for Vec2<f32> {}

#[sealed]
impl GenValue for Vec3<f32> {}

#[sealed]
impl GenValue for Vec4<f32> {}
