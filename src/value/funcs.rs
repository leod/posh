use crate::{IntoValue, Po};

use super::{builtin1, builtin2, Constructible, Vec3};

pub trait GenValue: Constructible + Sized {
    fn normalize(self) -> Self {
        builtin1("normalize", self)
    }

    fn sin(self) -> Self {
        builtin1("sin", self)
    }

    fn cos(self) -> Self {
        builtin1("cos", self)
    }

    fn tan(self) -> Self {
        builtin1("tan", self)
    }

    fn asin(self) -> Self {
        builtin1("asin", self)
    }

    fn acos(self) -> Self {
        builtin1("acos", self)
    }

    fn atan2(self, x: impl IntoValue<Type = Self>) -> Self {
        builtin2("atan", self, x)
    }

    fn atan(self) -> Self {
        builtin1("atan", self)
    }

    fn sinh(self) -> Self {
        builtin1("sinh", self)
    }

    fn cosh(self) -> Self {
        builtin1("cosh", self)
    }

    fn tanh(self) -> Self {
        builtin1("tanh", self)
    }

    fn asinh(self) -> Self {
        builtin1("asinh", self)
    }

    fn acosh(self) -> Self {
        builtin1("acosh", self)
    }

    fn atanh(self) -> Self {
        builtin1("atanh", self)
    }
}

impl GenValue for Po<f32> {}
impl GenValue for Vec3<f32> {}
