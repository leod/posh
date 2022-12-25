use std::rc::Rc;

use crate::dag::{BaseType, Expr, StructType, Type};

use super::{
    primitives::{field, simplify_struct_literal},
    Object, Struct, ToValue, Value,
};

macro_rules! impl_value {
    ($($name: ident),*) => {
        impl<$($name: Value),*> Struct for ($($name),*) {
            const STRUCT_TYPE: StructType = StructType {
                name: "tuple",
                fields: &[
                    $(
                        (stringify!($name), $name::TYPE)
                    ),*
                ],
                is_built_in: false,
            };
        }

        impl<$($name: Value),*> Object for ($($name),*) {
            const TYPE: Type = Type::Base(BaseType::Struct(&Self::STRUCT_TYPE));

            fn expr(&self) -> Rc<Expr> {
                #[allow(non_snake_case)]
                let ($($name),*) = self;

                simplify_struct_literal(
                    &Self::STRUCT_TYPE,
                    &[
                        $(
                            $name.expr()
                        ),*
                    ])
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
