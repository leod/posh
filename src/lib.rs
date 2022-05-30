pub mod lang;
pub mod prelude;
pub mod value;

pub use value::{and, or, ternary, var, Bool, Fsl, IntoValue, Value, F32, U32};

#[cfg(feature = "macros")]
pub use fsl_macros::fsl;
