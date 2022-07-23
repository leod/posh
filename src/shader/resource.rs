use sealed::sealed;

use crate::{Representative, Value};

/// A representative of a resource that can be bound to shaders.
pub trait Resource: Representative {
    #[doc(hidden)]
    fn stage_arg() -> Self;
}

/// A representative of a uniform block.
pub trait UniformBlock: Resource + Value {}

/// A representative of a collection of resources that can be bound to shaders.
pub trait Resources {
    #[doc(hidden)]
    fn stage_arg() -> Self;

    #[doc(hidden)]
    fn must_impl() {}
}

impl<D> Resources for D
where
    D: Resource,
{
    fn stage_arg() -> Self {
        <Self as Resource>::stage_arg()
    }
}

/// A representative that can be stored in a [`UniformBlock`].
#[sealed]
pub trait UniformBlockField: Value {
    #[doc(hidden)]
    fn must_impl() {}
}

// FIXME: Check for which types we should implement `UniformBlockField`.
impl_trait_for_built_in_types!(UniformBlockField);

#[sealed]
impl<T: UniformBlock> UniformBlockField for T {}
