use std::{
    iter::{Product, Sum},
    ops::{Add, BitAnd, BitOr, BitXor, Div, Mul, Neg, Not, Rem, Shl, Shr, Sub},
    rc::Rc,
};

use crate::ToSl;

use super::{
    dag::{BinaryOp, BuiltInType, Expr, Type, UnaryOp},
    primitives::{
        binary, built_in_1, built_in_2, cast, common_field_base, field, unary, value_arg,
    },
    Bool, Mat2, Mat3, Mat4, Object, Value, ValueNonArray, F32, I32, U32,
};

// Implements `Object` and `Value` for `$vec`.
macro_rules! impl_value {
    ($vec:ident, $mint:ty, $scalar:ident, $($member:ident),+) => {
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

        #[cfg(feature = "glam")]
        impl ToSl for glam::$vec {
            type Output = $vec;

            fn to_sl(self) -> Self::Output {
                Self::Output {
                    $(
                        $member: self.$member.to_sl()
                    ),+
                }
            }
        }

        #[cfg(feature = "mint")]
        impl ToSl for $mint {
            type Output = $vec;

            fn to_sl(self) -> Self::Output {
                Self::Output {
                    $(
                        $member: self.$member.to_sl()
                    ),+
                }
            }
        }

        impl ToSl for $vec {
            type Output = Self;

            fn to_sl(self) -> Self::Output {
                self
            }
        }

        impl $vec {
            pub fn eq(self, right: impl ToSl<Output = Self>) -> Bool {
                <Self as Value>::eq(self, right)
            }

            pub fn ne(self, right: impl ToSl<Output = Self>) -> Bool {
                <Self as Value>::ne(self, right)
            }

            pub fn get(self, index: impl ToSl<Output = U32>) -> F32 {
                // FIXME: Prevent out-of-bounds access.
                let base = self.expr();
                let index = index.to_sl().expr();
                let ty = F32::ty();

                let expr = Expr::Subscript { base, index, ty };

                F32::from_expr(expr)
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

// Implements `$vec <op> $vec` and `$vec <op> $scalar`.
macro_rules! impl_asymmetric_binary_op {
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
    };
}

// Implements numeric ops for `$vec`.
macro_rules! impl_numeric_ops {
    ($vec:ident, $bvec:ident, $scalar:ident) => {
        impl_binary_op!($vec, $scalar, Add, add);
        impl_binary_op!($vec, $scalar, Div, div);
        impl_binary_op!($vec, $scalar, Mul, mul);
        impl_binary_op!($vec, $scalar, Sub, sub);

        impl Neg for $vec {
            type Output = Self;

            fn neg(self) -> Self {
                unary(UnaryOp::Neg, self)
            }
        }

        impl Sum for $vec {
            fn sum<I>(iter: I) -> Self
            where
                I: Iterator<Item = Self>,
            {
                iter.fold(Self::ZERO, Self::add)
            }
        }

        impl Product for $vec {
            fn product<I>(iter: I) -> Self
            where
                I: Iterator<Item = Self>,
            {
                iter.fold(Self::ONE, Self::mul)
            }
        }

        impl $vec {
            pub fn cmpeq(self, rhs: Self) -> $bvec {
                built_in_2("equal", self, rhs)
            }

            pub fn cmpne(self, rhs: Self) -> $bvec {
                built_in_2("notEqual", self, rhs)
            }

            pub fn cmplt(self, rhs: Self) -> $bvec {
                built_in_2("lessThan", self, rhs)
            }

            pub fn cmple(self, rhs: Self) -> $bvec {
                built_in_2("lessThanEqual", self, rhs)
            }

            pub fn cmpge(self, rhs: Self) -> $bvec {
                built_in_2("greaterThanEqual", self, rhs)
            }

            pub fn cmpgt(self, rhs: Self) -> $bvec {
                built_in_2("greaterThan", self, rhs)
            }
        }
    };
}

// Implements integral ops for `$vec`.
macro_rules! impl_integral_ops {
    ($vec:ident, $scalar:ident) => {
        // For shl and shr, if the first operand is a scalar, the second operand
        // has to be a scalar as well.
        impl_asymmetric_binary_op!($vec, $scalar, Shl, shl);
        impl_asymmetric_binary_op!($vec, $scalar, Shr, shr);

        impl_binary_op!($vec, $scalar, BitAnd, bitand);
        impl_binary_op!($vec, $scalar, BitOr, bitor);
        impl_binary_op!($vec, $scalar, BitXor, bitxor);
        impl_binary_op!($vec, $scalar, Rem, rem);

        impl Not for $vec {
            type Output = Self;

            fn not(self) -> Self {
                unary(UnaryOp::Not, self)
            }
        }
    };
}

// Implements logical ops for a boolean `$vec`.
macro_rules! impl_boolean_ops {
    ($vec:ident) => {
        impl $vec {
            pub fn any(self) -> Bool {
                built_in_1("any", self)
            }

            pub fn all(self) -> Bool {
                built_in_1("all", self)
            }
        }

        impl Not for $vec {
            type Output = Self;

            fn not(self) -> Self {
                built_in_1("not", self)
            }
        }
    };
}

// Implements ops for `$vec`.
macro_rules! impl_ops {
    ($vec:ident, $bvec:ident, F32) => {
        impl_numeric_ops!($vec, $bvec, F32);
    };
    ($vec:ident, $bvec:ident, I32) => {
        impl_numeric_ops!($vec, $bvec, I32);
        impl_integral_ops!($vec, I32);
    };
    ($vec:ident, $bvec:ident, U32) => {
        impl_numeric_ops!($vec, $bvec, U32);
        impl_integral_ops!($vec, U32);
    };
    ($vec:ident, $bvec:ident, Bool) => {
        impl_boolean_ops!($vec);
    };
}

// Implements two-dimensional `$vec`.
macro_rules! impl_vec2 {
    ($vec:ident, $mint:ty, $bvec:ident, $vec_lower:ident, $scalar:ident, $vec3:ident) => {
        #[doc = concat!("A two-dimensional ", scalar_name!($scalar), " vector.")]
        #[derive(Debug, Copy, Clone, Default)]
        pub struct $vec {
            pub x: $scalar,
            pub y: $scalar,
        }

        impl $vec {
            /// Creates a new vector.
            pub fn new(x: impl ToSl<Output = $scalar>, y: impl ToSl<Output = $scalar>) -> Self {
                Self {
                    x: x.to_sl(),
                    y: y.to_sl(),
                }
            }

            /// Creates a vector with all elements set to `v`.
            pub fn splat(v: impl ToSl<Output = $scalar>) -> Self {
                let v = v.to_sl();

                Self { x: v, y: v }
            }

            /// Creates a three-dimensional vector from `self` and the given `z` value.
            pub fn extend(self, z: impl ToSl<Output = $scalar>) -> $vec3 {
                $vec3 {
                    x: self.x,
                    y: self.y,
                    z: z.to_sl(),
                }
            }
        }

        #[doc = concat!("Creates a two-dimensional ", scalar_name!($scalar), " vector.")]
        pub fn $vec_lower(x: impl ToSl<Output = $scalar>, y: impl ToSl<Output = $scalar>) -> $vec {
            $vec::new(x, y)
        }

        impl_value!($vec, $mint, $scalar, x, y);
        impl_ops!($vec, $bvec, $scalar);
    };
}

// Implements three-dimensional `$vec`.
macro_rules! impl_vec3 {
    (
        $vec:ident,
        $mint:ty,
        $bvec:ident,
        $vec_lower:ident,
        $scalar:ident,
        $vec2:ident,
        $vec4:ident
    ) => {
        #[doc = concat!("A three-dimensional ", scalar_name!($scalar), " vector.")]
        #[derive(Debug, Copy, Clone, Default)]
        pub struct $vec {
            pub x: $scalar,
            pub y: $scalar,
            pub z: $scalar,
        }

        impl $vec {
            /// Creates a new vector.
            pub fn new(
                x: impl ToSl<Output = $scalar>,
                y: impl ToSl<Output = $scalar>,
                z: impl ToSl<Output = $scalar>,
            ) -> Self {
                Self {
                    x: x.to_sl(),
                    y: y.to_sl(),
                    z: z.to_sl(),
                }
            }

            /// Creates a vector with all elements set to `v`.
            pub fn splat(v: impl ToSl<Output = $scalar>) -> Self {
                let v = v.to_sl();

                Self { x: v, y: v, z: v }
            }

            /// Creates a four-dimensional vector from `self` and the given `w` value.
            pub fn extend(self, w: impl ToSl<Output = $scalar>) -> $vec4 {
                $vec4 {
                    x: self.x,
                    y: self.y,
                    z: self.z,
                    w: w.to_sl(),
                }
            }

            pub fn xy(&self) -> $vec2 {
                $vec2 {
                    x: self.x,
                    y: self.y,
                }
            }

            pub fn yz(&self) -> $vec2 {
                $vec2 {
                    x: self.y,
                    y: self.z,
                }
            }
        }

        #[doc = concat!("Creates a three-dimensional ", scalar_name!($scalar), " vector.")]
        pub fn $vec_lower(
            x: impl ToSl<Output = $scalar>,
            y: impl ToSl<Output = $scalar>,
            z: impl ToSl<Output = $scalar>,
        ) -> $vec {
            $vec::new(x, y, z)
        }

        impl_value!($vec, $mint, $scalar, x, y, z);
        impl_ops!($vec, $bvec, $scalar);
    };
}

// Implements four-dimensional `$vec`.
macro_rules! impl_vec4 {
    (
        $vec:ident,
        $mint:ty,
        $bvec:ident,
        $vec_lower:ident,
        $scalar:ident,
        $vec2:ident,
        $vec3:ident
    ) => {
        #[doc = concat!("A four-dimensional ", scalar_name!($scalar), " vector.")]
        #[derive(Debug, Copy, Clone, Default)]
        pub struct $vec {
            pub x: $scalar,
            pub y: $scalar,
            pub z: $scalar,
            pub w: $scalar,
        }

        impl $vec {
            /// Creates a new vector.
            pub fn new(
                x: impl ToSl<Output = $scalar>,
                y: impl ToSl<Output = $scalar>,
                z: impl ToSl<Output = $scalar>,
                w: impl ToSl<Output = $scalar>,
            ) -> Self {
                Self {
                    x: x.to_sl(),
                    y: y.to_sl(),
                    z: z.to_sl(),
                    w: w.to_sl(),
                }
            }

            /// Creates a vector with all elements set to `v`.
            pub fn splat(v: impl ToSl<Output = $scalar>) -> Self {
                let v = v.to_sl();

                Self {
                    x: v,
                    y: v,
                    z: v,
                    w: v,
                }
            }

            pub fn xyz(&self) -> $vec3 {
                $vec3 {
                    x: self.x,
                    y: self.y,
                    z: self.z,
                }
            }

            pub fn yzw(&self) -> $vec3 {
                $vec3 {
                    x: self.y,
                    y: self.z,
                    z: self.w,
                }
            }

            pub fn xy(&self) -> $vec2 {
                $vec2 {
                    x: self.x,
                    y: self.y,
                }
            }

            pub fn yz(&self) -> $vec2 {
                $vec2 {
                    x: self.y,
                    y: self.z,
                }
            }

            pub fn zw(&self) -> $vec2 {
                $vec2 {
                    x: self.z,
                    y: self.w,
                }
            }

            pub fn zwxy(&self) -> Self {
                Self {
                    x: self.z,
                    y: self.w,
                    z: self.x,
                    w: self.y,
                }
            }
        }

        #[doc = concat!("Creates a four-dimensional ", scalar_name!($scalar), " vector.")]
        pub fn $vec_lower(
            x: impl ToSl<Output = $scalar>,
            y: impl ToSl<Output = $scalar>,
            z: impl ToSl<Output = $scalar>,
            w: impl ToSl<Output = $scalar>,
        ) -> $vec {
            $vec::new(x, y, z, w)
        }

        impl_value!($vec, $mint, $scalar, x, y, z, w);
        impl_ops!($vec, $bvec, $scalar);
    };
}

// Implements casts for vectors.
macro_rules! impl_casts {
    ($vec:ident, $($method:ident, $target:ident),+) => {
        impl $vec {
            $(
                pub fn $method(self) -> $target {
                    cast(self)
                }
            )+
        }
    };
}

impl_vec2!(Vec2, mint::Vector2<f32>, BVec2, vec2, F32, Vec3);
impl_vec3!(Vec3, mint::Vector3<f32>, BVec3, vec3, F32, Vec2, Vec4);
impl_vec4!(Vec4, mint::Vector4<f32>, BVec4, vec4, F32, Vec2, Vec3);

impl_vec2!(IVec2, mint::Vector2<i32>, BVec2, ivec2, I32, IVec3);
impl_vec3!(IVec3, mint::Vector3<i32>, BVec3, ivec3, I32, IVec2, IVec4);
impl_vec4!(IVec4, mint::Vector4<i32>, BVec4, ivec4, I32, IVec2, IVec3);

impl_vec2!(UVec2, mint::Vector2<u32>, BVec2, uvec2, U32, UVec3);
impl_vec3!(UVec3, mint::Vector3<u32>, BVec3, uvec3, U32, UVec2, UVec4);
impl_vec4!(UVec4, mint::Vector4<u32>, BVec4, uvec4, U32, UVec2, UVec3);

impl_vec2!(BVec2, mint::Vector2<bool>, BVec2, bvec2, Bool, BVec3);
impl_vec3!(BVec3, mint::Vector3<bool>, BVec3, bvec3, Bool, BVec2, BVec4);
impl_vec4!(BVec4, mint::Vector4<bool>, BVec4, bvec4, Bool, BVec2, BVec3);

impl_casts!(Vec2, as_ivec2, IVec2, as_uvec2, UVec2);
impl_casts!(Vec3, as_ivec3, IVec3, as_uvec3, UVec3);
impl_casts!(Vec4, as_ivec4, IVec4, as_uvec4, UVec4);

impl_casts!(IVec2, as_vec2, Vec2, as_uvec2, UVec2, as_bvec2, BVec2);
impl_casts!(IVec3, as_vec3, Vec3, as_uvec3, UVec3, as_bvec3, BVec3);
impl_casts!(IVec4, as_vec4, Vec4, as_uvec4, UVec4, as_bvec4, BVec4);

impl_casts!(UVec2, as_vec2, Vec2, as_ivec2, IVec2, as_bvec2, BVec2);
impl_casts!(UVec3, as_vec3, Vec3, as_ivec3, IVec3, as_bvec3, BVec3);
impl_casts!(UVec4, as_vec4, Vec4, as_ivec4, IVec4, as_bvec4, BVec4);

impl_casts!(BVec2, as_ivec2, IVec2, as_uvec2, UVec2);
impl_casts!(BVec3, as_ivec3, IVec3, as_uvec3, UVec3);
impl_casts!(BVec4, as_ivec4, IVec4, as_uvec4, UVec4);

impl_gen_type!(Vec2, BVec2);
impl_gen_type!(Vec3, BVec3);
impl_gen_type!(Vec4, BVec4);

impl Vec2 {
    // All zeroes.
    pub const ZERO: Self = Self {
        x: F32::ZERO,
        y: F32::ZERO,
    };

    // All ones.
    pub const ONE: Self = Self {
        x: F32::ONE,
        y: F32::ONE,
    };

    // All negative ones.
    pub const NEG_ONE: Self = Self {
        x: F32::NEG_ONE,
        y: F32::NEG_ONE,
    };

    // A unit-length vector pointing along the positive X axis.
    pub const X: Self = Self {
        x: F32::ONE,
        y: F32::ZERO,
    };

    // A unit-length vector pointing along the positive Y axis.
    pub const Y: Self = Self {
        x: F32::ZERO,
        y: F32::ONE,
    };

    // A unit-length vector pointing along the negative X axis.
    pub const NEG_X: Self = Self {
        x: F32::NEG_ONE,
        y: F32::ZERO,
    };

    // A unit-length vector pointing along the negative Y axis.
    pub const NEG_Y: Self = Self {
        x: F32::ZERO,
        y: F32::NEG_ONE,
    };

    pub fn from_angle(angle: F32) -> Self {
        let (sin, cos) = angle.sin_cos();
        Self { x: cos, y: sin }
    }

    pub fn outer_product(self, y: Vec2) -> Mat2 {
        built_in_2("outerProduct", self, y)
    }

    pub fn perp(self) -> Self {
        vec2(-self.y, self.x)
    }

    pub fn perp_dot(self, rhs: Self) -> F32 {
        (self.x * rhs.y) - (self.y * rhs.x)
    }

    pub fn rotate(self, rhs: Self) -> Self {
        vec2(
            self.x * rhs.x - self.y * rhs.y,
            self.y * rhs.x + self.x * rhs.y,
        )
    }
}

impl IVec2 {
    // All zeroes.
    pub const ZERO: Self = Self {
        x: I32::ZERO,
        y: I32::ZERO,
    };

    // All ones.
    pub const ONE: Self = Self {
        x: I32::ONE,
        y: I32::ONE,
    };

    // All negative ones.
    pub const NEG_ONE: Self = Self {
        x: I32::NEG_ONE,
        y: I32::NEG_ONE,
    };

    // A unit-length vector pointing along the positive X axis.
    pub const X: Self = Self {
        x: I32::ONE,
        y: I32::ZERO,
    };

    // A unit-length vector pointing along the positive Y axis.
    pub const Y: Self = Self {
        x: I32::ZERO,
        y: I32::ONE,
    };

    // A unit-length vector pointing along the negative X axis.
    pub const NEG_X: Self = Self {
        x: I32::NEG_ONE,
        y: I32::ZERO,
    };

    // A unit-length vector pointing along the negative Y axis.
    pub const NEG_Y: Self = Self {
        x: I32::ZERO,
        y: I32::NEG_ONE,
    };
}

impl UVec2 {
    // All zeroes.
    pub const ZERO: Self = Self {
        x: U32::ZERO,
        y: U32::ZERO,
    };

    // All ones.
    pub const ONE: Self = Self {
        x: U32::ONE,
        y: U32::ONE,
    };

    // A unit-length vector pointing along the positive X axis.
    pub const X: Self = Self {
        x: U32::ONE,
        y: U32::ZERO,
    };

    // A unit-length vector pointing along the positive Y axis.
    pub const Y: Self = Self {
        x: U32::ZERO,
        y: U32::ONE,
    };
}

impl BVec2 {
    // All false.
    pub const ZERO: Self = Self {
        x: Bool::FALSE,
        y: Bool::FALSE,
    };

    // All true.
    pub const ONE: Self = Self {
        x: Bool::TRUE,
        y: Bool::TRUE,
    };
}

impl Vec3 {
    // All zeroes.
    pub const ZERO: Self = Self {
        x: F32::ZERO,
        y: F32::ZERO,
        z: F32::ZERO,
    };

    // All ones.
    pub const ONE: Self = Self {
        x: F32::ONE,
        y: F32::ONE,
        z: F32::ONE,
    };

    // All negative ones.
    pub const NEG_ONE: Self = Self {
        x: F32::NEG_ONE,
        y: F32::NEG_ONE,
        z: F32::NEG_ONE,
    };

    // A unit-length vector pointing along the positive X axis.
    pub const X: Self = Self {
        x: F32::ONE,
        y: F32::ZERO,
        z: F32::ZERO,
    };

    // A unit-length vector pointing along the positive Y axis.
    pub const Y: Self = Self {
        x: F32::ZERO,
        y: F32::ONE,
        z: F32::ZERO,
    };

    // A unit-length vector pointing along the positive Z axis.
    pub const Z: Self = Self {
        x: F32::ZERO,
        y: F32::ZERO,
        z: F32::ONE,
    };

    // A unit-length vector pointing along the negative X axis.
    pub const NEG_X: Self = Self {
        x: F32::NEG_ONE,
        y: F32::ZERO,
        z: F32::ZERO,
    };

    // A unit-length vector pointing along the negative Y axis.
    pub const NEG_Y: Self = Self {
        x: F32::ZERO,
        y: F32::NEG_ONE,
        z: F32::ZERO,
    };

    // A unit-length vector pointing along the negative Z axis.
    pub const NEG_Z: Self = Self {
        x: F32::ZERO,
        y: F32::ZERO,
        z: F32::NEG_ONE,
    };

    pub fn cross(self, y: Vec3) -> Self {
        built_in_2("cross", self, y)
    }

    pub fn outer_product(self, y: Vec3) -> Mat3 {
        built_in_2("outerProduct", self, y)
    }
}

impl IVec3 {
    // All zeroes.
    pub const ZERO: Self = Self {
        x: I32::ZERO,
        y: I32::ZERO,
        z: I32::ZERO,
    };

    // All ones.
    pub const ONE: Self = Self {
        x: I32::ONE,
        y: I32::ONE,
        z: I32::ONE,
    };

    // All negative ones.
    pub const NEG_ONE: Self = Self {
        x: I32::NEG_ONE,
        y: I32::NEG_ONE,
        z: I32::NEG_ONE,
    };

    // A unit-length vector pointing along the positive X axis.
    pub const X: Self = Self {
        x: I32::ONE,
        y: I32::ZERO,
        z: I32::ZERO,
    };

    // A unit-length vector pointing along the positive Y axis.
    pub const Y: Self = Self {
        x: I32::ZERO,
        y: I32::ONE,
        z: I32::ZERO,
    };

    // A unit-length vector pointing along the positive Z axis.
    pub const Z: Self = Self {
        x: I32::ZERO,
        y: I32::ZERO,
        z: I32::ONE,
    };

    // A unit-length vector pointing along the negative X axis.
    pub const NEG_X: Self = Self {
        x: I32::NEG_ONE,
        y: I32::ZERO,
        z: I32::ZERO,
    };

    // A unit-length vector pointing along the negative Y axis.
    pub const NEG_Y: Self = Self {
        x: I32::ZERO,
        y: I32::NEG_ONE,
        z: I32::ZERO,
    };

    // A unit-length vector pointing along the negative Z axis.
    pub const NEG_Z: Self = Self {
        x: I32::ZERO,
        y: I32::ZERO,
        z: I32::NEG_ONE,
    };
}

impl UVec3 {
    // All zeroes.
    pub const ZERO: Self = Self {
        x: U32::ZERO,
        y: U32::ZERO,
        z: U32::ZERO,
    };

    // All ones.
    pub const ONE: Self = Self {
        x: U32::ONE,
        y: U32::ONE,
        z: U32::ONE,
    };

    // A unit-length vector pointing along the positive X axis.
    pub const X: Self = Self {
        x: U32::ONE,
        y: U32::ZERO,
        z: U32::ZERO,
    };

    // A unit-length vector pointing along the positive Y axis.
    pub const Y: Self = Self {
        x: U32::ZERO,
        y: U32::ONE,
        z: U32::ZERO,
    };

    // A unit-length vector pointing along the positive Z axis.
    pub const Z: Self = Self {
        x: U32::ZERO,
        y: U32::ZERO,
        z: U32::ONE,
    };
}

impl BVec3 {
    // All false.
    pub const FALSE: Self = Self {
        x: Bool::FALSE,
        y: Bool::FALSE,
        z: Bool::FALSE,
    };

    // All true.
    pub const TRUE: Self = Self {
        x: Bool::TRUE,
        y: Bool::TRUE,
        z: Bool::TRUE,
    };
}

impl Vec4 {
    // All zeroes.
    pub const ZERO: Self = Self {
        x: F32::ZERO,
        y: F32::ZERO,
        z: F32::ZERO,
        w: F32::ZERO,
    };

    // All ones.
    pub const ONE: Self = Self {
        x: F32::ONE,
        y: F32::ONE,
        z: F32::ONE,
        w: F32::ONE,
    };

    // All negative ones.
    pub const NEG_ONE: Self = Self {
        x: F32::NEG_ONE,
        y: F32::NEG_ONE,
        z: F32::NEG_ONE,
        w: F32::NEG_ONE,
    };

    // A unit-length vector pointing along the positive X axis.
    pub const X: Self = Self {
        x: F32::ONE,
        y: F32::ZERO,
        z: F32::ZERO,
        w: F32::ZERO,
    };

    // A unit-length vector pointing along the positive Y axis.
    pub const Y: Self = Self {
        x: F32::ZERO,
        y: F32::ONE,
        z: F32::ZERO,
        w: F32::ZERO,
    };

    // A unit-length vector pointing along the positive Z axis.
    pub const Z: Self = Self {
        x: F32::ZERO,
        y: F32::ZERO,
        z: F32::ONE,
        w: F32::ZERO,
    };

    // A unit-length vector pointing along the positive W axis.
    pub const W: Self = Self {
        x: F32::ZERO,
        y: F32::ZERO,
        z: F32::ZERO,
        w: F32::ONE,
    };

    // A unit-length vector pointing along the negative X axis.
    pub const NEG_X: Self = Self {
        x: F32::NEG_ONE,
        y: F32::ZERO,
        z: F32::ZERO,
        w: F32::ZERO,
    };

    // A unit-length vector pointing along the negative Y axis.
    pub const NEG_Y: Self = Self {
        x: F32::ZERO,
        y: F32::NEG_ONE,
        z: F32::ZERO,
        w: F32::ZERO,
    };

    // A unit-length vector pointing along the negative Z axis.
    pub const NEG_Z: Self = Self {
        x: F32::ZERO,
        y: F32::ZERO,
        z: F32::NEG_ONE,
        w: F32::ZERO,
    };

    // A unit-length vector pointing along the negative W axis.
    pub const NEG_W: Self = Self {
        x: F32::ZERO,
        y: F32::ZERO,
        z: F32::ZERO,
        w: F32::NEG_ONE,
    };

    pub fn outer_product(self, y: Vec4) -> Mat4 {
        built_in_2("outerProduct", self, y)
    }
}

impl IVec4 {
    // All zeroes.
    pub const ZERO: Self = Self {
        x: I32::ZERO,
        y: I32::ZERO,
        z: I32::ZERO,
        w: I32::ZERO,
    };

    // All ones.
    pub const ONE: Self = Self {
        x: I32::ONE,
        y: I32::ONE,
        z: I32::ONE,
        w: I32::ONE,
    };

    // All negative ones.
    pub const NEG_ONE: Self = Self {
        x: I32::NEG_ONE,
        y: I32::NEG_ONE,
        z: I32::NEG_ONE,
        w: I32::NEG_ONE,
    };

    // A unit-length vector pointing along the positive X axis.
    pub const X: Self = Self {
        x: I32::ONE,
        y: I32::ZERO,
        z: I32::ZERO,
        w: I32::ZERO,
    };

    // A unit-length vector pointing along the positive Y axis.
    pub const Y: Self = Self {
        x: I32::ZERO,
        y: I32::ONE,
        z: I32::ZERO,
        w: I32::ZERO,
    };

    // A unit-length vector pointing along the positive Z axis.
    pub const Z: Self = Self {
        x: I32::ZERO,
        y: I32::ZERO,
        z: I32::ONE,
        w: I32::ZERO,
    };

    // A unit-length vector pointing along the positive W axis.
    pub const W: Self = Self {
        x: I32::ZERO,
        y: I32::ZERO,
        z: I32::ZERO,
        w: I32::ONE,
    };

    // A unit-length vector pointing along the negative X axis.
    pub const NEG_X: Self = Self {
        x: I32::NEG_ONE,
        y: I32::ZERO,
        z: I32::ZERO,
        w: I32::ZERO,
    };

    // A unit-length vector pointing along the negative Y axis.
    pub const NEG_Y: Self = Self {
        x: I32::ZERO,
        y: I32::NEG_ONE,
        z: I32::ZERO,
        w: I32::ZERO,
    };

    // A unit-length vector pointing along the negative Z axis.
    pub const NEG_Z: Self = Self {
        x: I32::ZERO,
        y: I32::ZERO,
        z: I32::NEG_ONE,
        w: I32::ZERO,
    };

    // A unit-length vector pointing along the negative W axis.
    pub const NEG_W: Self = Self {
        x: I32::ZERO,
        y: I32::ZERO,
        z: I32::ZERO,
        w: I32::NEG_ONE,
    };
}

impl UVec4 {
    // All zeroes.
    pub const ZERO: Self = Self {
        x: U32::ZERO,
        y: U32::ZERO,
        z: U32::ZERO,
        w: U32::ZERO,
    };

    // All ones.
    pub const ONE: Self = Self {
        x: U32::ONE,
        y: U32::ONE,
        z: U32::ONE,
        w: U32::ONE,
    };

    // A unit-length vector pointing along the positive X axis.
    pub const X: Self = Self {
        x: U32::ONE,
        y: U32::ZERO,
        z: U32::ZERO,
        w: U32::ZERO,
    };

    // A unit-length vector pointing along the positive Y axis.
    pub const Y: Self = Self {
        x: U32::ZERO,
        y: U32::ONE,
        z: U32::ZERO,
        w: U32::ZERO,
    };

    // A unit-length vector pointing along the positive Z axis.
    pub const Z: Self = Self {
        x: U32::ZERO,
        y: U32::ZERO,
        z: U32::ONE,
        w: U32::ZERO,
    };

    // A unit-length vector pointing along the positive W axis.
    pub const W: Self = Self {
        x: U32::ZERO,
        y: U32::ZERO,
        z: U32::ZERO,
        w: U32::ONE,
    };
}

impl BVec4 {
    // All false.
    pub const FALSE: Self = Self {
        x: Bool::FALSE,
        y: Bool::FALSE,
        z: Bool::FALSE,
        w: Bool::FALSE,
    };

    // All true.
    pub const TRUE: Self = Self {
        x: Bool::TRUE,
        y: Bool::TRUE,
        z: Bool::TRUE,
        w: Bool::TRUE,
    };
}
