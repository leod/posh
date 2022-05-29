pub(crate) mod expr_reg;
pub mod lang;
pub mod value;

pub use value::{and, branch, eval, or, Fush};

#[cfg(feature = "macros")]
pub use fush_macros::fush;