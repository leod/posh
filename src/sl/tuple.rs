use std::rc::Rc;

use crate::dag::{BaseType, Expr, StructType, Type};

use super::{
    primitives::{field, simplify_struct_literal},
    Object, Value,
};

macro_rules! impl_value {
    ($($name: ident),*) => {
        impl<$($name: Value),*> Object for ($($name),*) {
            const TYPE: Type = Type::Base(BaseType::Struct(&StructType {
                name: "tuple",
                fields: &[
                    $((stringify!($name), $name::TYPE)),*
                ],
                is_built_in: false,
            }));

            #[allow(non_snake_case)]
            fn expr(&self) -> Rc<Expr> {
                let ($($name),*) = self;

                let struct_type = match &Self::TYPE {
                    Type::Base(BaseType::Struct(ref struct_type)) => struct_type,
                    _ => unreachable!(),
                };

                simplify_struct_literal(struct_type, &[$($name.expr()),*])
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
    };
}

impl_value!();
impl_value!(A, B);
impl_value!(A, B, C);
impl_value!(A, B, C, D);
