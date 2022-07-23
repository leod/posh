use crate::lang::{Expr, Ty};

#[doc(hidden)]
pub fn add_prefix(lhs: &str, rhs: &str) -> String {
    format!("{}_{}", lhs, rhs)
}

pub trait Fields {
    fn fields(prefix: &str) -> Vec<(String, Ty)>;
}

pub trait InputFields: Fields {
    #[doc(hidden)]
    fn stage_input(prefix: &str) -> Self;
}

pub trait OutputFields: Fields {
    fn stage_output(self) -> Vec<(String, Expr)>;
}
