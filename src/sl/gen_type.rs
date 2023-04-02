macro_rules! impl_gen_type {
    ($ty:ident, $tyb:ident) => {
        impl $ty {
            pub fn length(self) -> Self {
                super::primitives::built_in_1("length", self)
            }

            pub fn distance(self, y: impl super::ToValue<Output = Self>) -> Self {
                super::primitives::built_in_2("distance", self, y.to_value())
            }

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

            pub fn exp(self) -> Self {
                super::primitives::built_in_1("exp", self)
            }

            pub fn log(self) -> Self {
                super::primitives::built_in_1("log", self)
            }

            pub fn exp2(self) -> Self {
                super::primitives::built_in_1("exp2", self)
            }

            pub fn log2(self) -> Self {
                super::primitives::built_in_1("log2", self)
            }

            pub fn sqrt(self) -> Self {
                super::primitives::built_in_1("sqrt", self)
            }

            pub fn abs(self) -> Self {
                super::primitives::built_in_1("abs", self)
            }

            pub fn signum(self) -> Self {
                super::primitives::built_in_1("signum", self)
            }

            pub fn floor(self) -> Self {
                super::primitives::built_in_1("floor", self)
            }

            pub fn round(self) -> Self {
                super::primitives::built_in_1("round", self)
            }

            pub fn ceil(self) -> Self {
                super::primitives::built_in_1("ceil", self)
            }

            pub fn fract(self) -> Self {
                super::primitives::built_in_1("fract", self)
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

            pub fn dot(self, y: impl super::ToValue<Output = Self>) -> super::F32 {
                super::primitives::built_in_2("dot", self, y.to_value())
            }

            pub fn min(self, y: impl super::ToValue<Output = Self>) -> Self {
                super::primitives::built_in_2("min", self, y.to_value())
            }

            pub fn max(self, y: impl super::ToValue<Output = Self>) -> Self {
                super::primitives::built_in_2("max", self, y.to_value())
            }

            pub fn modulus(self, y: impl super::ToValue<Output = Self>) -> Self {
                super::primitives::built_in_2("mod", self, y.to_value())
            }

            pub fn modulus_with_f32(self, y: impl super::ToValue<Output = F32>) -> Self {
                super::primitives::built_in_2("mod", self, y.to_value())
            }

            pub fn clamp(
                self,
                min: impl super::ToValue<Output = Self>,
                max: impl super::ToValue<Output = Self>,
            ) -> Self {
                super::primitives::built_in_3("clamp", self, min.to_value(), max.to_value())
            }

            pub fn clamp_with_f32(
                self,
                min: impl super::ToValue<Output = F32>,
                max: impl super::ToValue<Output = F32>,
            ) -> Self {
                super::primitives::built_in_3("clamp", self, min.to_value(), max.to_value())
            }

            pub fn mix(
                self,
                y: impl super::ToValue<Output = Self>,
                a: impl super::ToValue<Output = Self>,
            ) -> Self {
                super::primitives::built_in_3("mix", self, y.to_value(), a.to_value())
            }

            pub fn select(
                a: impl super::ToValue<Output = $tyb>,
                x: impl super::ToValue<Output = Self>,
                y: impl super::ToValue<Output = Self>,
            ) -> Self {
                super::primitives::built_in_3("mix", x.to_value(), y.to_value(), a.to_value())
            }

            pub fn step(self, edge: impl super::ToValue<Output = Self>) -> Self {
                super::primitives::built_in_2("step", edge.to_value(), self)
            }

            pub fn step_with_f32(self, edge: impl super::ToValue<Output = F32>) -> Self {
                super::primitives::built_in_2("step", edge.to_value(), self)
            }

            pub fn smoothstep(
                self,
                edge1: impl super::ToValue<Output = Self>,
                edge2: impl super::ToValue<Output = Self>,
            ) -> Self {
                super::primitives::built_in_3(
                    "smoothstep",
                    edge1.to_value(),
                    edge2.to_value(),
                    self,
                )
            }

            pub fn smoothstep_with_f32(
                self,
                edge1: impl super::ToValue<Output = F32>,
                edge2: impl super::ToValue<Output = F32>,
            ) -> Self {
                super::primitives::built_in_3(
                    "smoothstep",
                    edge1.to_value(),
                    edge2.to_value(),
                    self,
                )
            }

            // TODO: mod, mix, step, smoothstep?
        }
    };
}
