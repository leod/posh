use sealed::sealed;

use crate::{Representative, Value};

/// A representative of a resource.
pub trait Resource: Representative {
    #[doc(hidden)]
    fn stage_arg() -> Self;
}

/// A representative of a uniform block.
pub trait UniformBlock: Resource + Value {}

/// A representative of a collection of resources.
pub trait Resources {
    #[doc(hidden)]
    fn stage_arg() -> Self;
}

impl<D> Resources for D
where
    D: Resource,
{
    fn stage_arg() -> Self {
        <Self as Resource>::stage_arg()
    }
}

/// A field which can be stored in uniform blocks.
#[sealed]
pub trait UniformBlockField {}

impl_trait_for_built_in_types!(UniformBlockField);

#[sealed]
impl<T: UniformBlock> UniformBlockField for T {}
