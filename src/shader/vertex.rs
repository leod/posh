use sealed::sealed;

use crate::{expose::NumericType, Scalar, Value, Vec3, Vec4};

/// A value that can be stored in a vertex.
#[sealed]
pub trait VertexFieldValue {}

#[sealed]
impl<T: NumericType> VertexFieldValue for Scalar<T> {}

// TODO: VertexFieldValue for Vec2

#[sealed]
impl<T: NumericType> VertexFieldValue for Vec3<T> {}

#[sealed]
impl<T: NumericType> VertexFieldValue for Vec4<T> {}

// TODO: VertexFieldValue for f32 matrix types

/// A representative of a vertex.
pub trait Vertex: Value {}

/// A representative of vertex stage input attributes.
pub trait Attributes: Value {}

impl<V: Vertex> Attributes for V {}

impl<V1: Vertex, V2: Vertex> Attributes for (V1, V2) {}

/// A representative of fragment stage input fields.
pub trait FInputs: Value {}

/// A value that can be given as a fragment stage input field.
#[sealed]
pub trait FInputFieldValue {}

#[sealed]
impl<T: NumericType> FInputFieldValue for Scalar<T> {}

// TODO: FInputFieldValue for Vec2

#[sealed]
impl<T: NumericType> FInputFieldValue for Vec3<T> {}

#[sealed]
impl<T: NumericType> FInputFieldValue for Vec4<T> {}

// TODO: VertexFieldValue for f32 matrix types
// TODO: FInputFieldValue for arrays

#[sealed]
impl<V: FInputs> FInputFieldValue for V {}

/// A representative of fragment stage output fields.
pub trait FOutputs: Value {}

/// A value that can be given as a fragment stage output field.
#[sealed]
pub trait FOutputFieldValue {}

#[sealed]
impl<T: NumericType> FOutputFieldValue for Scalar<T> {}

// TODO: FOutputFieldValue for Vec2

#[sealed]
impl<T: NumericType> FOutputFieldValue for Vec3<T> {}

#[sealed]
impl<T: NumericType> FOutputFieldValue for Vec4<T> {}
