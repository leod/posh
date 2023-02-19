use std::{
    ops::{Add, Div, Mul, Sub},
    rc::Rc,
};

use crate::dag::{BinaryOp, BuiltInType, Expr, Type};

use super::{
    primitives::{binary, common_field_base, field, value_arg},
    Bool, Object, ToValue, Value, ValueNonArray, F32, I32, U32,
};

// Implements `Object` and `Value` for `$vec`.
macro_rules! impl_value {
    ($vec:ident, $scalar:ident, $($member:ident),+) => {
        impl Object for $vec {
            fn ty() -> Type {
                Type::BuiltIn(BuiltInType::$vec)
            }

            fn expr(&self) -> Rc<Expr> {
                let base = common_field_base(
                    &Self::ty(),
                    [$(std::stringify!($member)),+].into_iter(),
                    &[$(self.$member.expr()),+],
                );

                if let Some(base) = base {
                    base
                } else {
                    let ty = Self::ty();
                    let name = format!("{}", ty);
                    let args = vec![$(self.$member.expr()),+];

                    let expr = Expr::CallBuiltIn { ty, name, args };

                    Rc::new(expr)
                }
            }

            fn from_arg(path: &str) -> Self {
                value_arg(path)
            }
        }

        impl Value for $vec {
            fn from_expr(expr: Expr) -> Self {
                let base = Rc::new(expr);

                Self {
                    $(
                        $member: field(base.clone(), stringify!($member))
                    ),+
                }
            }
        }

        impl ValueNonArray for $vec {}

        impl ToValue for glam::$vec {
            type Output = $vec;

            fn to_value(self) -> Self::Output {
                Self::Output {
                    $(
                        $member: self.$member.to_value()
                    ),+
                }
            }
        }

        impl ToValue for $vec {
            type Output = Self;

            fn to_value(self) -> Self::Output {
                self
            }
        }
    };
}

// Implements `$vec <op> $vec` and `$vec <op> $scalar` and `$scalar <op> $vec`.
macro_rules! impl_binary_op {
    ($vec:ident, $scalar:ident, $op:ident, $fn:ident) => {
        impl $op<$vec> for $vec {
            type Output = Self;

            fn $fn(self, right: Self) -> Self {
                binary(self, BinaryOp::$op, right)
            }
        }

        impl $op<$scalar> for $vec {
            type Output = Self;

            fn $fn(self, right: $scalar) -> Self {
                binary(self, BinaryOp::$op, right)
            }
        }

        impl $op<scalar_physical!($scalar)> for $vec {
            type Output = Self;

            fn $fn(self, right: scalar_physical!($scalar)) -> Self {
                binary(self, BinaryOp::$op, right)
            }
        }

        impl $op<$vec> for $scalar {
            type Output = $vec;

            fn $fn(self, right: $vec) -> $vec {
                binary(self, BinaryOp::$op, right)
            }
        }

        impl $op<$vec> for scalar_physical!($scalar) {
            type Output = $vec;

            fn $fn(self, right: $vec) -> $vec {
                binary(self, BinaryOp::$op, right)
            }
        }
    };
}

// Implements numeric ops for `$vec`.
macro_rules! impl_numeric_ops {
    ($vec:ident, $scalar:ident) => {
        impl_binary_op!($vec, $scalar, Add, add);
        impl_binary_op!($vec, $scalar, Div, div);
        impl_binary_op!($vec, $scalar, Mul, mul);
        impl_binary_op!($vec, $scalar, Sub, sub);
    };
}

// Implements ops for `$vec`.
macro_rules! impl_ops {
    ($vec:ident, Bool, bool) => {};
    ($vec:ident, $logical:ident) => {
        impl_numeric_ops!($vec, $logical);
    };
}

// Implements two-dimensional `$vec`.
macro_rules! impl_vec2 {
    ($vec:ident, $vec_lower:ident, $scalar:ident, $extension:ident) => {
        #[doc = concat!("A two-dimensional ", scalar_name!($scalar), " vector.")]
        #[derive(Debug, Copy, Clone)]
        pub struct $vec {
            pub x: $scalar,
            pub y: $scalar,
        }

        impl $vec {
            /// Creates a new vector.
            pub fn new(
                x: impl ToValue<Output = $scalar>,
                y: impl ToValue<Output = $scalar>,
            ) -> Self {
                Self {
                    x: x.to_value(),
                    y: y.to_value(),
                }
            }

            /// Creates a vector with all elements set to `v`.
            pub fn splat(v: impl ToValue<Output = $scalar>) -> Self {
                let v = v.to_value();

                Self { x: v, y: v }
            }

            /// Creates a three-dimensional vector from `self` and the given `z` value.
            pub fn extend(self, z: impl ToValue<Output = $scalar>) -> $extension {
                $extension {
                    x: self.x,
                    y: self.y,
                    z: z.to_value(),
                }
            }
        }

        #[doc = concat!("Creates a two-dimensional ", scalar_name!($scalar), " vector.")]
        pub fn $vec_lower(
            x: impl ToValue<Output = $scalar>,
            y: impl ToValue<Output = $scalar>,
        ) -> $vec {
            $vec::new(x, y)
        }

        impl_value!($vec, $scalar, x, y);
        impl_ops!($vec, $scalar);
    };
}

// Implements three-dimensional `$vec`.
macro_rules! impl_vec3 {
    ($vec:ident, $vec_lower:ident, $scalar:ident, $extension:ident) => {
        #[doc = concat!("A three-dimensional ", scalar_name!($scalar), " vector.")]
        #[derive(Debug, Copy, Clone)]
        pub struct $vec {
            pub x: $scalar,
            pub y: $scalar,
            pub z: $scalar,
        }

        impl $vec {
            /// Creates a new vector.
            pub fn new(
                x: impl ToValue<Output = $scalar>,
                y: impl ToValue<Output = $scalar>,
                z: impl ToValue<Output = $scalar>,
            ) -> Self {
                Self {
                    x: x.to_value(),
                    y: y.to_value(),
                    z: z.to_value(),
                }
            }

            /// Creates a vector with all elements set to `v`.
            pub fn splat(v: impl ToValue<Output = $scalar>) -> Self {
                let v = v.to_value();

                Self { x: v, y: v, z: v }
            }

            /// Creates a four-dimensional vector from `self` and the given `w` value.
            pub fn extend(self, w: impl ToValue<Output = $scalar>) -> $extension {
                $extension {
                    x: self.x,
                    y: self.y,
                    z: self.z,
                    w: w.to_value(),
                }
            }
        }

        #[doc = concat!("Creates a three-dimensional ", scalar_name!($scalar), " vector.")]
        pub fn $vec_lower(
            x: impl ToValue<Output = $scalar>,
            y: impl ToValue<Output = $scalar>,
            z: impl ToValue<Output = $scalar>,
        ) -> $vec {
            $vec::new(x, y, z)
        }

        impl_value!($vec, $scalar, x, y, z);
        impl_ops!($vec, $scalar);
    };
}

// Implements four-dimensional `$vec`.
macro_rules! impl_vec4 {
    ($vec:ident, $vec_lower:ident, $scalar:ident) => {
        #[doc = concat!("A four-dimensional ", scalar_name!($scalar), " vector.")]
        #[derive(Debug, Copy, Clone)]
        pub struct $vec {
            pub x: $scalar,
            pub y: $scalar,
            pub z: $scalar,
            pub w: $scalar,
        }

        impl $vec {
            /// Creates a new vector.
            pub fn new(
                x: impl ToValue<Output = $scalar>,
                y: impl ToValue<Output = $scalar>,
                z: impl ToValue<Output = $scalar>,
                w: impl ToValue<Output = $scalar>,
            ) -> Self {
                Self {
                    x: x.to_value(),
                    y: y.to_value(),
                    z: z.to_value(),
                    w: w.to_value(),
                }
            }

            /// Creates a vector with all elements set to `v`.
            pub fn splat(v: impl ToValue<Output = $scalar>) -> Self {
                let v = v.to_value();

                Self {
                    x: v,
                    y: v,
                    z: v,
                    w: v,
                }
            }
        }

        #[doc = concat!("Creates a four-dimensional ", scalar_name!($scalar), " vector.")]
        pub fn $vec_lower(
            x: impl ToValue<Output = $scalar>,
            y: impl ToValue<Output = $scalar>,
            z: impl ToValue<Output = $scalar>,
            w: impl ToValue<Output = $scalar>,
        ) -> $vec {
            $vec::new(x, y, z, w)
        }

        impl_value!($vec, $scalar, x, y, z, w);
        impl_ops!($vec, $scalar);
    };
}

impl_vec2!(Vec2, vec2, F32, Vec3);
impl_vec2!(IVec2, ivec2, I32, IVec3);
impl_vec2!(UVec2, uvec2, U32, UVec3);
impl_vec2!(BVec2, bvec2, Bool, BVec3);

impl_vec3!(Vec3, vec3, F32, Vec4);
impl_vec3!(IVec3, ivec3, I32, IVec4);
impl_vec3!(UVec3, uvec3, U32, UVec4);
impl_vec3!(BVec3, bvec3, Bool, BVec4);

impl_vec4!(Vec4, vec4, F32);
impl_vec4!(IVec4, ivec4, I32);
impl_vec4!(UVec4, uvec4, U32);
impl_vec4!(BVec4, bvec4, Bool);

impl_gen_type!(Vec2);
impl_gen_type!(Vec3);
impl_gen_type!(Vec4);
