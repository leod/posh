use std::{
    ops::{Add, Div, Mul, Sub},
    rc::Rc,
};

use crate::{
    dag::{BaseType, BinaryOp, Expr, Type},
    Numeric, Primitive,
};

use super::{
    primitives::{binary, common_field_base, field, value_arg},
    Object, Scalar, ToValue, Value,
};

/// A two-dimensional vector in the shading language.
#[derive(Debug, Copy, Clone)]
pub struct Vec2<T> {
    pub x: Scalar<T>,
    pub y: Scalar<T>,
}

impl<T: Primitive> Vec2<T> {
    pub fn to_vec3(self) -> Vec3<T> {
        Vec3 {
            x: self.x,
            y: self.y,
            z: Default::default(),
        }
    }

    pub fn to_vec4(self) -> Vec4<T> {
        Vec4 {
            x: self.x,
            y: self.y,
            z: Default::default(),
            w: Default::default(),
        }
    }
}

/// A four-dimensional vector in the shading language.
#[derive(Debug, Copy, Clone)]
pub struct Vec3<T> {
    pub x: Scalar<T>,
    pub y: Scalar<T>,
    pub z: Scalar<T>,
}

impl<T: Primitive> Vec3<T> {
    pub fn to_vec4(self) -> Vec4<T> {
        Vec4 {
            x: self.x,
            y: self.y,
            z: self.z,
            w: Default::default(),
        }
    }
}

/// A four-dimensional vector in the shading language.
#[derive(Debug, Copy, Clone)]
pub struct Vec4<T> {
    pub x: Scalar<T>,
    pub y: Scalar<T>,
    pub z: Scalar<T>,
    pub w: Scalar<T>,
}

// Implements `Object` and `Value` for `$ty<T>`.
macro_rules! impl_value {
    ($ty:ident, $mint_ty: ident, $($member:ident),+) => {
        impl<T: Primitive> Object for $ty<T> {
            fn ty() -> Type {
                Type::Base(BaseType::$ty(T::PRIMITIVE_TYPE))
            }

            fn expr(&self) -> Rc<Expr> {
                if let Some(base) = common_field_base(
                    &BaseType::$ty(T::PRIMITIVE_TYPE),
                    [
                        $(
                            std::stringify!($member)
                        ),+
                    ]
                    .into_iter(),
                    &[
                        $(
                            self.$member.expr()
                        ),+,
                    ],
                ) {
                    base
                } else {
                    let ty = Self::ty();
                    let name = format!("{}", ty);
                    let args = vec![
                        $(
                            self.$member.expr()
                        ),+,
                    ];

                    let expr = Expr::CallBuiltIn { ty, name, args };

                    Rc::new(expr)
                }
            }

            fn from_arg(path: &str) -> Self {
                value_arg(path)
            }
        }

        impl<T: Primitive> Value for $ty<T> {
            fn from_expr(expr: Expr) -> Self {
                let base = Rc::new(expr);

                Self {
                    $(
                        $member: field(base.clone(), stringify!($member))
                    ),+
                }
            }
        }

        impl<T: Primitive> ToValue for mint::$mint_ty<T> {
            type Output = $ty<T>;

            fn to_value(self) -> Self::Output {
                Self::Output {
                    $(
                        $member: self.$member.to_value()
                    ),+
                }
            }
        }

        impl<T: Primitive> ToValue for $ty<T> {
            type Output = Self;

            fn to_value(self) -> Self::Output {
                self
            }
        }
    };
}

// Implements `$ty<T> <op> $ty<T>` for all `T: Numeric`.
macro_rules! impl_binary_op_symmetric {
    ($ty:ident, $fn:ident, $op:ident) => {
        impl<T> $op<$ty<T>> for $ty<T>
        where
            T: Numeric,
        {
            type Output = Self;

            fn $fn(self, right: Self) -> Self::Output {
                binary(self, BinaryOp::$op, right)
            }
        }
    };
}

// Implements `$ty<T> <op> Scalar<T>` for all `T: Numeric`.
macro_rules! impl_binary_op_scalar_rhs {
    ($ty:ident, $fn:ident, $op:ident) => {
        impl<T, Rhs> $op<Rhs> for $ty<T>
        where
            T: Numeric,
            Rhs: ToValue<Output = Scalar<T>>,
        {
            type Output = Self;

            fn $fn(self, right: Rhs) -> Self::Output {
                binary(self, BinaryOp::$op, right)
            }
        }
    };
}

// Implements all the things for `$ty`.
macro_rules! impl_vec {
    ($ty:ident, $mint_ty:ident, $name:ident, $($member:ident),+) => {
        impl_value!($ty, $mint_ty, $($member),+);

        impl<T: Primitive> Default for $ty<T> {
            fn default() -> Self {
                $ty {
                    $(
                        $member: Default::default()
                    ),*
                }
            }
        }

        impl_binary_op_symmetric!($ty, add, Add);
        impl_binary_op_symmetric!($ty, div, Div);
        impl_binary_op_symmetric!($ty, mul, Mul);
        impl_binary_op_symmetric!($ty, sub, Sub);

        impl_binary_op_scalar_rhs!($ty, add, Add);
        impl_binary_op_scalar_rhs!($ty, div, Div);
        impl_binary_op_scalar_rhs!($ty, mul, Mul);
        impl_binary_op_scalar_rhs!($ty, sub, Sub);

        impl_gen_type!($ty);
    };
}

impl_vec!(Vec2, Vector2, vec2, x, y);
impl_vec!(Vec3, Vector3, vec3, x, y, z);
impl_vec!(Vec4, Vector4, vec4, x, y, z, w);

/// Constructs a [`Vec2`] conveniently.
pub fn vec2<T: Primitive>(
    x: impl ToValue<Output = Scalar<T>>,
    y: impl ToValue<Output = Scalar<T>>,
) -> Vec2<T> {
    Vec2 {
        x: x.to_value(),
        y: y.to_value(),
    }
}

/// Constructs a [`Vec3`] conveniently.
pub fn vec3<T: Primitive>(
    x: impl ToValue<Output = Scalar<T>>,
    y: impl ToValue<Output = Scalar<T>>,
    z: impl ToValue<Output = Scalar<T>>,
) -> Vec3<T> {
    Vec3 {
        x: x.to_value(),
        y: y.to_value(),
        z: z.to_value(),
    }
}

/// Constructs a [`Vec4`] conveniently.
pub fn vec4<T: Primitive>(
    x: impl ToValue<Output = Scalar<T>>,
    y: impl ToValue<Output = Scalar<T>>,
    z: impl ToValue<Output = Scalar<T>>,
    w: impl ToValue<Output = Scalar<T>>,
) -> Vec4<T> {
    Vec4 {
        x: x.to_value(),
        y: y.to_value(),
        z: z.to_value(),
        w: w.to_value(),
    }
}
