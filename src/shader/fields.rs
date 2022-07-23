use crate::lang::Ty;

#[doc(hidden)]
pub fn add_prefix(lhs: &str, rhs: &str) -> String {
    format!("{}_{}", lhs, rhs)
}

pub trait Fields {
    fn fields(prefix: &str) -> Vec<(String, Ty)>;

    #[doc(hidden)]
    fn stage_input(prefix: &str) -> Self;
}
