use sealed::sealed;

use crate::Scalar;

use super::{ScalarType, Vec3, Vec4};

/// A representative of a built-in value.
#[sealed]
pub trait BuiltInValue {}

#[sealed]
impl<T: ScalarType> BuiltInValue for Scalar<T> {}

#[sealed]
impl<T: ScalarType> BuiltInValue for Vec3<T> {}

#[sealed]
impl<T: ScalarType> BuiltInValue for Vec4<T> {}

#[sealed]
impl<U: BuiltInValue, V: BuiltInValue> BuiltInValue for (U, V) {}

macro_rules! impl_trait_for_built_in_types {
    ($trait_name:ident) => {
        #[sealed]
        impl<T: crate::expose::ScalarType> $trait_name for crate::Scalar<T> {}

        #[sealed]
        impl<T: crate::expose::ScalarType> $trait_name for crate::Vec3<T> {}

        #[sealed]
        impl<T: crate::expose::ScalarType> $trait_name for crate::Vec4<T> {}

        #[sealed]
        impl<U: $trait_name, V: $trait_name> $trait_name for (U, V) {}
    };
}
