use std::rc::Rc;

use crate::dag::{BaseType, Expr, StructType, Type};

use super::{
    primitives::{field, simplify_struct_literal},
    Object, ToValue, Value,
};

macro_rules! impl_value {
    ($($name: ident),*) => {
        impl<$($name: Value),*> Object for ($($name),*) {
            const TYPE: Type = Type::Base(BaseType::Struct(&StructType {
                name: "tuple",
                fields: &[
                    $(
                        (stringify!($name), $name::TYPE)
                    ),*
                ],
                is_built_in: false,
            }));

            fn expr(&self) -> Rc<Expr> {
                #[allow(non_snake_case)]
                let ($($name),*) = self;

                let struct_type = match &Self::TYPE {
                    Type::Base(BaseType::Struct(ref struct_type)) => struct_type,
                    _ => unreachable!(),
                };

                simplify_struct_literal(
                    struct_type,
                    &[
                        $(
                            $name.expr()
                        ),*
                    ])
            }
        }

        impl<$($name: Value),*> Value for ($($name),*) {
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
