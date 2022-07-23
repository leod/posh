use sealed::sealed;

use crate::{
    lang::{Ident, Ty},
    FuncArg, Representative, Value,
};

use super::fields::{Fields, InputFields};

/// A representative of a resource that can be bound to shaders.
pub trait Resource: FuncArg + InputFields {}

/// A representative of a uniform block.
pub trait UniformBlock: Resource + Value {}

/// A representative of a collection of resources that can be bound to shaders.
pub trait Resources: InputFields {
    #[doc(hidden)]
    fn must_impl() {}
}

/*
impl<D: Resource> Fields for D {
    fn fields(prefix: &str) -> Vec<(String, Ty)> {
        vec![(prefix.into(), <D as FuncArg>::ty())]
    }
}

impl<D: Resource> InputFields for D {
    fn stage_input(prefix: &str) -> Self {
        <D as FuncArg>::from_ident(Ident::new(prefix))
    }
}
*/

impl<D: Resource> Resources for D {}

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
