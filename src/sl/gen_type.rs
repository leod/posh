macro_rules! impl_gen_type {
    ($ty:ident, $tyb:ident) => {
        impl $ty {
            pub fn length(self) -> super::F32 {
                super::primitives::built_in_1("length", self)
            }

            pub fn length_squared(self) -> super::F32 {
                self.dot(self)
            }

            pub fn distance(self, y: impl super::ToSl<Output = Self>) -> Self {
                super::primitives::built_in_2("distance", self, y.to_sl())
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

            pub fn sin_cos(self) -> (Self, Self) {
                (self.sin(), self.cos())
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

            pub fn atan2(self, x: impl super::ToSl<Output = Self>) -> Self {
                super::primitives::built_in_2("atan", self, x.to_sl())
            }

            pub fn pow(self, y: impl super::ToSl<Output = Self>) -> Self {
                super::primitives::built_in_2("pow", self, y.to_sl())
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
                super::primitives::built_in_1("sign", self)
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

            pub fn dot(self, y: impl super::ToSl<Output = Self>) -> super::F32 {
                super::primitives::built_in_2("dot", self, y.to_sl())
            }

            pub fn min(self, y: impl super::ToSl<Output = Self>) -> Self {
                super::primitives::built_in_2("min", self, y.to_sl())
            }

            pub fn max(self, y: impl super::ToSl<Output = Self>) -> Self {
                super::primitives::built_in_2("max", self, y.to_sl())
            }

            pub fn modulus(self, y: impl super::ToSl<Output = Self>) -> Self {
                super::primitives::built_in_2("mod", self, y.to_sl())
            }

            pub fn modulus_with_f32(self, y: impl super::ToSl<Output = F32>) -> Self {
                super::primitives::built_in_2("mod", self, y.to_sl())
            }

            pub fn cmpclamp(
                self,
                min: impl super::ToSl<Output = Self>,
                max: impl super::ToSl<Output = Self>,
            ) -> Self {
                super::primitives::built_in_3("clamp", self, min.to_sl(), max.to_sl())
            }

            pub fn clamp(
                self,
                min: impl super::ToSl<Output = F32>,
                max: impl super::ToSl<Output = F32>,
            ) -> Self {
                super::primitives::built_in_3("clamp", self, min.to_sl(), max.to_sl())
            }

            pub fn cmpmix(
                x: impl super::ToSl<Output = Self>,
                y: impl super::ToSl<Output = Self>,
                a: impl super::ToSl<Output = Self>,
            ) -> Self {
                super::primitives::built_in_3("mix", x.to_sl(), y.to_sl(), a.to_sl())
            }

            pub fn mix(
                x: impl super::ToSl<Output = Self>,
                y: impl super::ToSl<Output = Self>,
                a: impl super::ToSl<Output = F32>,
            ) -> Self {
                super::primitives::built_in_3("mix", x.to_sl(), y.to_sl(), a.to_sl())
            }

            pub fn select(
                x: impl super::ToSl<Output = Self>,
                y: impl super::ToSl<Output = Self>,
                a: impl super::ToSl<Output = $tyb>,
            ) -> Self {
                super::primitives::built_in_3("mix", x.to_sl(), y.to_sl(), a.to_sl())
            }

            pub fn cmpstep(
                edge: impl super::ToSl<Output = Self>,
                x: impl super::ToSl<Output = Self>,
            ) -> Self {
                super::primitives::built_in_2("step", edge.to_sl(), x.to_sl())
            }

            pub fn step(
                edge: impl super::ToSl<Output = F32>,
                x: impl super::ToSl<Output = Self>,
            ) -> Self {
                super::primitives::built_in_2("step", edge.to_sl(), x.to_sl())
            }

            pub fn cmpsmoothstep(
                edge1: impl super::ToSl<Output = Self>,
                edge2: impl super::ToSl<Output = Self>,
                x: impl super::ToSl<Output = Self>,
            ) -> Self {
                super::primitives::built_in_3("smoothstep", edge1.to_sl(), edge2.to_sl(), x.to_sl())
            }

            pub fn smoothstep(
                self,
                edge1: impl super::ToSl<Output = F32>,
                edge2: impl super::ToSl<Output = F32>,
                x: impl super::ToSl<Output = F32>,
            ) -> Self {
                super::primitives::built_in_3("smoothstep", edge1.to_sl(), edge2.to_sl(), x.to_sl())
            }
        }
    };
}
