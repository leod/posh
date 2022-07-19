use sealed::sealed;

use crate::{expose::NumType, Scalar, Value, Vec2, Vec3, Vec4};

/// A representative that can be stored in a [`Vertex`].
#[sealed]
pub trait VertexField: Value {}

#[sealed]
impl<T: NumType> VertexField for Scalar<T> {}

#[sealed]
impl<T: NumType> VertexField for Vec2<T> {}

#[sealed]
impl<T: NumType> VertexField for Vec3<T> {}

#[sealed]
impl<T: NumType> VertexField for Vec4<T> {}

// TODO: VertexFieldValue for f32 matrix types

/// A representative of a vertex.
pub trait Vertex: Value {}

/// A representative of vertex stage input.
pub trait Attributes: Value {}

impl<V: Vertex> Attributes for V {}

impl<V1: Vertex, V2: Vertex> Attributes for (V1, V2) {}

/// A representative of vertex stage output and fragment stage input.
pub trait Interpolants: Value {}

/// A representative that can be stored in [`Interpolants`].
#[sealed]
pub trait InterpolantsField {}

#[sealed]
impl<T: NumType> InterpolantsField for Scalar<T> {}

#[sealed]
impl<T: NumType> InterpolantsField for Vec2<T> {}

#[sealed]
impl<T: NumType> InterpolantsField for Vec3<T> {}

#[sealed]
impl<T: NumType> InterpolantsField for Vec4<T> {}

// TODO: InterpolantsField for f32 matrix types
// TODO: InterpolantsField for arrays

#[sealed]
impl<V: Interpolants> InterpolantsField for V {}

/// A representative of fragment stage output.
pub trait Fragment: Value {}

/// A representative that can be stored in a [`Fragment`].
#[sealed]
pub trait FragmentField: Value {}

#[sealed]
impl<T: NumType> FragmentField for Scalar<T> {}

#[sealed]
impl<T: NumType> FragmentField for Vec2<T> {}

#[sealed]
impl<T: NumType> FragmentField for Vec3<T> {}

#[sealed]
impl<T: NumType> FragmentField for Vec4<T> {}
