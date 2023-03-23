use std::{
    ops::{Add, Div, Mul, Sub},
    rc::Rc,
};

use super::{
    dag::{BinaryOp, Expr, Trace, Type},
    primitives::{binary, value_arg},
    Object, ToValue, Value, ValueNonArray,
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
    (U32) => {
        super::dag::BuiltInType::I32
    };
    (I32) => {
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
    (U32) => {
        "unsigned integer"
    };
    (I32) => {
        "signed integer"
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
    (U32) => {
        "{}"
    };
    (I32) => {
        "{}"
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

        impl $scalar {
            pub fn lt(self, rhs: impl ToValue<Output = Self>) -> Bool {
                binary(self, BinaryOp::Lt, rhs)
            }

            pub fn le(self, rhs: impl ToValue<Output = Self>) -> Bool {
                binary(self, BinaryOp::Le, rhs)
            }

            pub fn ge(self, rhs: impl ToValue<Output = Self>) -> Bool {
                binary(self, BinaryOp::Ge, rhs)
            }

            pub fn gt(self, rhs: impl ToValue<Output = Self>) -> Bool {
                binary(self, BinaryOp::Gt, rhs)
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
                $physical::default().to_value()
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

        impl ToValue for $physical {
            type Output = $scalar;

            fn to_value(self) -> Self::Output {
                $scalar::new(self)
            }
        }

        impl ToValue for $scalar {
            type Output = Self;

            fn to_value(self) -> Self::Output {
                self
            }
        }

        impl From<$physical> for $scalar {
            fn from(x: $physical) -> Self {
                x.to_value()
            }
        }

        impl $scalar {
            pub fn new(x: $physical) -> Self {
                Self::from_expr(Expr::ScalarLiteral {
                    ty: scalar_built_in_type!($scalar),
                    value: format!(scalar_format!($scalar), x),
                })
            }

            pub fn eq(&self, right: impl ToValue<Output = Self>) -> Bool {
                binary(*self, BinaryOp::Eq, right)
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

impl_gen_type!(F32);

impl Bool {
    pub fn and(self, right: impl ToValue<Output = Self>) -> Self {
        binary(self, BinaryOp::And, right)
    }

    pub fn or(self, right: impl ToValue<Output = Self>) -> Self {
        binary(self, BinaryOp::Or, right)
    }

    pub fn branch<V: Value>(
        self,
        yes: impl ToValue<Output = V>,
        no: impl ToValue<Output = V>,
    ) -> V {
        let ty = V::ty();
        let cond = self.expr();
        let yes = yes.to_value().expr();
        let no = no.to_value().expr();

        let expr = Expr::Branch { ty, cond, yes, no };

        V::from_expr(expr)
    }
}
