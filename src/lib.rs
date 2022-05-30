pub(crate) mod expr_reg;
pub mod lang;
pub mod prelude;
pub mod value;

pub use value::{and, branch, or, var, Bool, Fush, F32, U32};

#[cfg(feature = "macros")]
pub use fsl_macros::fsl;
