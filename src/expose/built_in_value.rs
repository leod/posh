use sealed::sealed;

use crate::Scalar;

use super::{ScalarType, Vec3, Vec4};

/// A representative for a built-in value.
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
