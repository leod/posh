use std::rc::Rc;

use crate::ToSl;

use super::{
    dag::{Expr, StructType, Type},
    primitives::{field, simplify_struct_literal, value_arg},
    unique_struct_type, Object, Struct, Value, ValueNonArray,
};

macro_rules! tuple_impl {
    ($($name: ident),*) => {
        impl<$($name: Value,)*> Object for ($($name,)*) {
            fn ty() -> Type {
                Type::Struct(Self::struct_type())
            }

            fn expr(&self) -> Rc<Expr> {
                #[allow(non_snake_case)]
                let ($($name,)*) = self;

                simplify_struct_literal(
                    Self::struct_type(),
                    &[$($name.expr()),*])
            }

            fn from_arg(path: &str) -> Self {
                value_arg(path)
            }
        }

        impl<$($name: Value,)*> Value for ($($name,)*) {
            #[allow(clippy::unused_unit)]
            fn from_expr(expr: Expr) -> Self {
                #[allow(unused)]
                let base = Rc::new(expr);

                ($(field(base.clone(), stringify!($name))),*)
            }
        }

        impl<$($name: Value,)*> ValueNonArray for ($($name,)*) {}

        impl<$($name: Value,)*> Struct for ($($name,)*) {
            fn struct_type() -> Rc<StructType> {
                unique_struct_type::<Self>(
                    || StructType {
                        name: "tuple".to_string(),
                        fields: vec![
                            $((stringify!($name).to_string(), <$name as Object>::ty()),)*
                        ],
                    }
                )
            }
        }

        impl<$($name: ToSl,)*> ToSl for ($($name,)*) {
            type Output = (
                $(<$name as ToSl>::Output,)*
            );

            #[allow(clippy::unused_unit)]
            fn to_sl(self) -> Self::Output {
                #[allow(non_snake_case)]
                let ($($name,)*) = self;

                ($($name.to_sl(),)*)
            }
        }
    };
}

smaller_tuples_too!(tuple_impl, T0, T1, T2, T3, T4, T5, T6, T7);
