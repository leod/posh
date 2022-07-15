use sealed::sealed;

use crate::{expose::NumericType, Scalar, Value, Vec2, Vec3, Vec4};

/// A value that can be stored in a vertex.
#[sealed]
pub trait VertexField {}

#[sealed]
impl<T: NumericType> VertexField for Scalar<T> {}

#[sealed]
impl<T: NumericType> VertexField for Vec2<T> {}

#[sealed]
impl<T: NumericType> VertexField for Vec3<T> {}

#[sealed]
impl<T: NumericType> VertexField for Vec4<T> {}

// TODO: VertexFieldValue for f32 matrix types

/// A representative of a vertex.
pub trait Vertex: Value {}

/// A representative of vertex stage input attributes.
pub trait Attributes: Value {}

impl<V: Vertex> Attributes for V {}

impl<V1: Vertex, V2: Vertex> Attributes for (V1, V2) {}

/// A representative of fragment stage input.
pub trait Interpolants: Value {}

/// A value that can be given as a fragment stage input field.
#[sealed]
pub trait InterpolantsField {}

#[sealed]
impl<T: NumericType> InterpolantsField for Scalar<T> {}

#[sealed]
impl<T: NumericType> InterpolantsField for Vec2<T> {}

#[sealed]
impl<T: NumericType> InterpolantsField for Vec3<T> {}

#[sealed]
impl<T: NumericType> InterpolantsField for Vec4<T> {}

// TODO: InterpolantsField for f32 matrix types
// TODO: InterpolantsField for arrays

#[sealed]
impl<V: Interpolants> InterpolantsField for V {}

/// A representative of fragment stage output.
pub trait Fragment: Value {}

/// A value that can be given as a fragment stage output field.
#[sealed]
pub trait FragmentField {}

#[sealed]
impl<T: NumericType> FragmentField for Scalar<T> {}

#[sealed]
impl<T: NumericType> FragmentField for Vec2<T> {}

#[sealed]
impl<T: NumericType> FragmentField for Vec3<T> {}

#[sealed]
impl<T: NumericType> FragmentField for Vec4<T> {}
