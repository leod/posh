use crate::{Posh, F32};

use super::Type;

pub trait Domain {
    type Field<T: Type>;
    type F32;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RustD;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PoshD;

impl Domain for RustD {
    type Field<T: Type> = T;
    type F32 = f32;
}

impl Domain for PoshD {
    type Field<T: Type> = Posh<T>;
    type F32 = F32;
}
