use std::rc::Rc;

use crate::dag::{BaseType, Expr, StructType, Type};

use super::{
    primitives::{field, simplify_struct_literal, value_arg},
    unique_struct_type, Object, Struct, ToValue, Value, ValueNonArray,
};

macro_rules! impl_value {
    ($($name: ident),*) => {
        impl<$($name: Value),*> Object for ($($name),*) {
            fn ty() -> Type {
                Type::Base(BaseType::Struct(Self::struct_type()))
            }

            fn expr(&self) -> Rc<Expr> {
                #[allow(non_snake_case)]
                let ($($name),*) = self;

                simplify_struct_literal(
                    Self::struct_type(),
                    &[
                        $(
                            $name.expr()
                        ),*
                    ])
            }

            fn from_arg(path: &str) -> Self {
                value_arg(path)
            }
        }

        impl<$($name: Value),*> Value for ($($name),*) {
            #[allow(clippy::unused_unit)]
            fn from_expr(expr: Expr) -> Self {
                #[allow(unused)]
                let base = Rc::new(expr);

                (
                    $(
                        field(base.clone(), stringify!($name))
                    ),*
                )
            }
        }

        impl<$($name: Value),*> ValueNonArray for ($($name),*) {
            fn base_type() -> BaseType {
                BaseType::Struct(Self::struct_type())
            }
        }

        impl<$($name: Value),*> Struct for ($($name),*) {
            fn struct_type() -> Rc<StructType> {
                unique_struct_type::<Self>(
                    || StructType {
                        name: "tuple".to_string(),
                        fields: vec![
                            $(
                                (stringify!($name).to_string(), <$name as Object>::ty())
                            ),*
                        ],
                    }
                )
            }
        }

        impl<$($name: ToValue),*> ToValue for ($($name),*) {
            type Output = (
                $(
                    <$name as ToValue>::Output
                ),*
            );

            #[allow(clippy::unused_unit)]
            fn to_value(self) -> Self::Output {
                #[allow(non_snake_case)]
                let ($($name),*) = self;

                (
                    $(
                        $name.to_value()
                    ),*
                )
            }
        }
    };
}

impl_value!();
impl_value!(A, B);
impl_value!(A, B, C);
impl_value!(A, B, C, D);
