use sealed::sealed;

use crate::{BuiltInValue, Representative, Value};

/// A representative of a resource.
pub trait Resource: Representative {
    fn func_arg() -> Self;
}

/// A representative of a uniform block.
pub trait UniformBlock: Resource + Value {}

/// A representative of a collection of resources.
pub trait Resources {
    fn func_arg() -> Self;
}

impl<D> Resources for D
where
    D: Resource,
{
    fn func_arg() -> Self {
        <Self as Resource>::func_arg()
    }
}

/// A field which can be stored in uniform blocks.
#[sealed]
pub trait UniformBlockField {}

#[sealed]
impl<T: BuiltInValue> UniformBlockField for T {}
