pub(crate) mod expr_reg;
pub mod lang;
pub mod value;

pub use value::{and, branch, or, var, Bool, Fush, F32, U32};

#[cfg(feature = "macros")]
pub use fush_macros::fush;
