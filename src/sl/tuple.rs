use std::rc::Rc;

use crate::dag::{BaseTy, Expr, StructTy, Ty};

use super::{
    primitives::{field, simplify_struct_literal},
    Object, Value,
};

macro_rules! impl_value {
    ($($name: ident),*) => {
        impl<$($name: Value),*> Object for ($($name),*) {
            const TY: Ty = Ty::Base(BaseTy::Struct(&StructTy {
                name: "tuple",
                fields: &[
                    $((stringify!($name), $name::TY)),*
                ],
                is_built_in: false,
            }));

            #[allow(non_snake_case)]
            fn expr(&self) -> Rc<Expr> {
                let ($($name),*) = self;

                let struct_ty = match &Self::TY {
                    Ty::Base(BaseTy::Struct(ref struct_ty)) => struct_ty,
                    _ => unreachable!(),
                };

                simplify_struct_literal(struct_ty, &[$($name.expr()),*])
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
