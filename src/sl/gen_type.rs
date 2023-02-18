macro_rules! impl_gen_type {
    ($ty:ident) => {
        impl $ty {
            pub fn normalize(self) -> Self {
                super::primitives::built_in_1("normalize", self)
            }

            pub fn sin(self) -> Self {
                super::primitives::built_in_1("sin", self)
            }

            pub fn cos(self) -> Self {
                super::primitives::built_in_1("cos", self)
            }

            pub fn tan(self) -> Self {
                super::primitives::built_in_1("tan", self)
            }

            pub fn asin(self) -> Self {
                super::primitives::built_in_1("asin", self)
            }

            pub fn acos(self) -> Self {
                super::primitives::built_in_1("acos", self)
            }

            pub fn atan2(self, x: impl super::ToValue<Output = Self>) -> Self {
                super::primitives::built_in_2("atan", self, x.to_value())
            }

            pub fn pow(self, y: impl super::ToValue<Output = Self>) -> Self {
                super::primitives::built_in_2("pow", self, y.to_value())
            }

            pub fn atan(self) -> Self {
                super::primitives::built_in_1("atan", self)
            }

            pub fn sinh(self) -> Self {
                super::primitives::built_in_1("sinh", self)
            }

            pub fn cosh(self) -> Self {
                super::primitives::built_in_1("cosh", self)
            }

            pub fn tanh(self) -> Self {
                super::primitives::built_in_1("tanh", self)
            }

            pub fn asinh(self) -> Self {
                super::primitives::built_in_1("asinh", self)
            }

            pub fn acosh(self) -> Self {
                super::primitives::built_in_1("acosh", self)
            }

            pub fn atanh(self) -> Self {
                super::primitives::built_in_1("atanh", self)
            }

            pub fn dot(self, y: impl super::ToValue<Output = Self>) -> Self {
                super::primitives::built_in_2("dot", self, y.to_value())
            }

            pub fn min(self, y: impl super::ToValue<Output = Self>) -> Self {
                super::primitives::built_in_2("min", self, y.to_value())
            }

            pub fn max(self, y: impl super::ToValue<Output = Self>) -> Self {
                super::primitives::built_in_2("min", self, y.to_value())
            }

            pub fn clamp(
                self,
                min: impl super::ToValue<Output = Self>,
                max: impl super::ToValue<Output = Self>,
            ) -> Self {
                super::primitives::built_in_3("clamp", self, min.to_value(), max.to_value())
            }
        }
    };
}
