use std::{
    iter::{Product, Sum},
    ops::{Add, BitAnd, BitOr, BitXor, Div, Mul, Neg, Not, Rem, Shl, Shr, Sub},
    rc::Rc,
};

use crate::ToSl;

use super::{
    branches,
    dag::{BinaryOp, Expr, Trace, Type, UnaryOp},
    primitives::{binary, cast, unary, value_arg},
    FsInput, Object, Value, ValueNonArray,
};

// Maps from logical scalar type to physical scalar type.
macro_rules! scalar_physical {
    (F32) => {
        f32
    };
    (U32) => {
        u32
    };
    (I32) => {
        i32
    };
    (Bool) => {
        bool
    };
}

pub(crate) use scalar_physical;

// The built-in type of a scalar.
macro_rules! scalar_built_in_type {
    (F32) => {
        super::dag::BuiltInType::F32
    };
    (I32) => {
        super::dag::BuiltInType::I32
    };
    (U32) => {
        super::dag::BuiltInType::U32
    };
    (Bool) => {
        super::dag::BuiltInType::Bool
    };
}

// The name of a scalar as it appears in documentation.
macro_rules! scalar_name {
    (F32) => {
        "floating-point"
    };
    (I32) => {
        "signed integer"
    };
    (U32) => {
        "unsigned integer"
    };
    (Bool) => {
        "boolean"
    };
}

// Formatting scalar literals.
macro_rules! scalar_format {
    (F32) => {
        // Need to use `{:?}` since `{}` formats e.g. 1.0 as just 1, but GLSL ES
        // 3.0 always wants a decimal point for floats.
        "{:?}"
    };
    (I32) => {
        "{}"
    };
    (U32) => {
        "{}u"
    };
    (Bool) => {
        "{:?}"
    };
}

// Implements `$scalar <op> $scalar` and `$scalar <op> $physical` and `$physical
// <op> $scalar`.
macro_rules! impl_binary_op {
    ($scalar:ident, $op:ident, $fn:ident) => {
        impl $op<$scalar> for $scalar {
            type Output = Self;

            fn $fn(self, right: $scalar) -> Self {
                binary(self, BinaryOp::$op, right)
            }
        }

        impl $op<scalar_physical!($scalar)> for $scalar {
            type Output = Self;

            fn $fn(self, right: scalar_physical!($scalar)) -> Self {
                binary(self, BinaryOp::$op, right)
            }
        }

        impl $op<$scalar> for scalar_physical!($scalar) {
            type Output = $scalar;

            fn $fn(self, right: $scalar) -> $scalar {
                binary(self, BinaryOp::$op, right)
            }
        }
    };
}

// Implements numeric ops for `$scalar`.
macro_rules! impl_numeric_ops {
    ($scalar:ident) => {
        impl_binary_op!($scalar, Add, add);
        impl_binary_op!($scalar, Div, div);
        impl_binary_op!($scalar, Mul, mul);
        impl_binary_op!($scalar, Sub, sub);

        impl Neg for $scalar {
            type Output = Self;

            fn neg(self) -> Self {
                unary(UnaryOp::Neg, self)
            }
        }

        impl Sum for $scalar {
            fn sum<I>(iter: I) -> Self
            where
                I: Iterator<Item = Self>,
            {
                iter.fold(Self::ZERO, Self::add)
            }
        }

        impl Product for $scalar {
            fn product<I>(iter: I) -> Self
            where
                I: Iterator<Item = Self>,
            {
                iter.fold(Self::ONE, Self::mul)
            }
        }

        impl $scalar {
            pub fn lt(self, rhs: impl ToSl<Output = Self>) -> Bool {
                binary(self, BinaryOp::Lt, rhs)
            }

            pub fn le(self, rhs: impl ToSl<Output = Self>) -> Bool {
                binary(self, BinaryOp::Le, rhs)
            }

            pub fn ge(self, rhs: impl ToSl<Output = Self>) -> Bool {
                binary(self, BinaryOp::Ge, rhs)
            }

            pub fn gt(self, rhs: impl ToSl<Output = Self>) -> Bool {
                binary(self, BinaryOp::Gt, rhs)
            }
        }
    };
}

// Implements integral ops for `$scalar`.
macro_rules! impl_integral_ops {
    ($scalar:ident) => {
        impl_binary_op!($scalar, Shl, shl);
        impl_binary_op!($scalar, Shr, shr);
        impl_binary_op!($scalar, BitAnd, bitand);
        impl_binary_op!($scalar, BitOr, bitor);
        impl_binary_op!($scalar, BitXor, bitxor);
        impl_binary_op!($scalar, Rem, rem);

        // Bitwise negation is a special case, since it uses a different
        // operator in Rust than in GLSL.
        impl Not for $scalar {
            type Output = Self;

            fn not(self) -> Self {
                unary(UnaryOp::BitNot, self)
            }
        }
    };
}

// Implements a `$scalar` value.
macro_rules! impl_scalar {
    ($scalar:ident, $physical:ident) => {
        #[doc = concat!("A ", scalar_name!($scalar), " scalar.")]
        #[derive(Debug, Copy, Clone)]
        pub struct $scalar(Trace);

        impl Default for $scalar {
            fn default() -> Self {
                $physical::default().to_sl()
            }
        }

        impl Object for $scalar {
            fn ty() -> Type {
                Type::BuiltIn(scalar_built_in_type!($scalar))
            }

            fn expr(&self) -> Rc<Expr> {
                self.0.expr()
            }

            fn from_arg(path: &str) -> Self {
                value_arg(path)
            }
        }

        impl Value for $scalar {
            fn from_expr(expr: Expr) -> Self {
                assert!(expr.ty() == Self::ty());

                Self(Trace::new(expr))
            }
        }

        impl ValueNonArray for $scalar {}

        impl ToSl for $physical {
            type Output = $scalar;

            fn to_sl(self) -> Self::Output {
                $scalar::new(self)
            }
        }

        impl ToSl for $scalar {
            type Output = Self;

            fn to_sl(self) -> Self::Output {
                self
            }
        }

        impl From<$physical> for $scalar {
            fn from(x: $physical) -> Self {
                x.to_sl()
            }
        }

        impl $scalar {
            pub fn new(x: $physical) -> Self {
                Self::from_expr(Expr::ScalarLiteral {
                    ty: scalar_built_in_type!($scalar),
                    value: format!(scalar_format!($scalar), x),
                })
            }

            pub fn eq(self, right: impl ToSl<Output = Self>) -> Bool {
                <Self as Value>::eq(self, right)
            }

            pub fn ne(self, right: impl ToSl<Output = Self>) -> Bool {
                <Self as Value>::ne(self, right)
            }
        }
    };
}

impl_scalar!(F32, f32);
impl_scalar!(I32, i32);
impl_scalar!(U32, u32);
impl_scalar!(Bool, bool);

impl_numeric_ops!(F32);
impl_numeric_ops!(I32);
impl_numeric_ops!(U32);

impl_integral_ops!(I32);
impl_integral_ops!(U32);

impl_gen_type!(F32, Bool);

impl F32 {
    pub const ZERO: Self = F32(Trace::c(|| 0.0.to_sl().expr()));
    pub const ONE: Self = F32(Trace::c(|| 1.0.to_sl().expr()));
    pub const NEG_ONE: Self = F32(Trace::c(|| (-1.0).to_sl().expr()));

    pub fn as_i32(self) -> I32 {
        cast(self)
    }

    pub fn as_u32(self) -> U32 {
        cast(self)
    }
}

impl I32 {
    pub const ZERO: Self = I32(Trace::c(|| 0i32.to_sl().expr()));
    pub const ONE: Self = I32(Trace::c(|| 1i32.to_sl().expr()));
    pub const NEG_ONE: Self = I32(Trace::c(|| (-1i32).to_sl().expr()));

    pub fn as_f32(self) -> F32 {
        cast(self)
    }

    pub fn as_u32(self) -> U32 {
        cast(self)
    }

    pub fn as_bool(self) -> Bool {
        cast(self)
    }
}

impl U32 {
    pub const ZERO: Self = U32(Trace::c(|| 0u32.to_sl().expr()));
    pub const ONE: Self = U32(Trace::c(|| 1u32.to_sl().expr()));

    pub fn as_f32(self) -> F32 {
        cast(self)
    }

    pub fn as_i32(self) -> I32 {
        cast(self)
    }

    pub fn as_bool(self) -> Bool {
        cast(self)
    }
}

impl Bool {
    pub const TRUE: Self = Bool(Trace::c(|| true.to_sl().expr()));
    pub const FALSE: Self = Bool(Trace::c(|| false.to_sl().expr()));

    pub fn as_i32(self) -> I32 {
        cast(self)
    }

    pub fn as_u32(self) -> U32 {
        cast(self)
    }

    pub fn branch<V: Value>(self, yes: impl ToSl<Output = V>, no: impl ToSl<Output = V>) -> V {
        let ty = V::ty();
        let cond = self.to_sl().expr();
        let yes = yes.to_sl().expr();
        let no = no.to_sl().expr();

        let expr = Expr::Branch { ty, cond, yes, no };

        V::from_expr(expr)
    }

    pub fn then<V: Value>(self, yes: impl ToSl<Output = V>) -> Branches<V> {
        Branches {
            arms: vec![(self, yes.to_sl())],
        }
    }

    pub fn then_discard<V: Value, W>(self, input: FsInput<W>) -> Branches<V> {
        self.then(input.discard::<V>())
    }

    pub fn and(self, right: impl ToSl<Output = Self>) -> Self {
        binary(self, BinaryOp::And, right)
    }

    pub fn or(self, right: impl ToSl<Output = Self>) -> Self {
        binary(self, BinaryOp::Or, right)
    }
}

impl Not for Bool {
    type Output = Self;

    fn not(self) -> Self {
        unary(UnaryOp::Not, self)
    }
}

pub struct Branches<V> {
    arms: Vec<(Bool, V)>,
}

impl<V> Branches<V>
where
    V: Value,
{
    pub fn else_then(self, cond: impl ToSl<Output = Bool>, value: impl ToSl<Output = V>) -> Self {
        let mut arms = self.arms;
        arms.push((cond.to_sl(), value.to_sl()));

        Self { arms }
    }

    pub fn else_then_discard<W>(self, cond: impl ToSl<Output = Bool>, input: FsInput<W>) -> Self {
        self.else_then(cond, input.discard::<V>())
    }

    pub fn otherwise(self, default: impl ToSl<Output = V>) -> V {
        branches(self.arms, default)
    }

    pub fn otherwise_discard<W>(self, input: FsInput<W>) -> V {
        self.otherwise(input.discard::<V>())
    }
}
