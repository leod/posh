use sealed::sealed;

use crate::{expose::NumType, lang::Ty, FuncArg, Scalar, Value, Vec2, Vec3, Vec4};

use super::fields::{add_prefix, Fields, InputFields, OutputFields};

/// A representative that can be stored in a [`Vertex`].
#[sealed]
pub trait VertexField: Value + InputFields {
    #[doc(hidden)]
    fn must_impl() {}
}

impl<T: NumType> Fields for Scalar<T> {
    fn fields(prefix: &str) -> Vec<(String, Ty)> {
        vec![(prefix.into(), <Self as FuncArg>::ty())]
    }
}

impl<T: NumType> Fields for Vec2<T> {
    fn fields(prefix: &str) -> Vec<(String, Ty)> {
        vec![(prefix.into(), <Self as FuncArg>::ty())]
    }
}

impl<T: NumType> Fields for Vec3<T> {
    fn fields(prefix: &str) -> Vec<(String, Ty)> {
        vec![(prefix.into(), <Self as FuncArg>::ty())]
    }
}

impl<T: NumType> Fields for Vec4<T> {
    fn fields(prefix: &str) -> Vec<(String, Ty)> {
        vec![(prefix.into(), <Self as FuncArg>::ty())]
    }
}

impl<T: NumType> InputFields for Scalar<T> {
    fn stage_input(prefix: &str) -> Self {
        Self::from_var_name(prefix)
    }
}

impl<T: NumType> InputFields for Vec2<T> {
    fn stage_input(prefix: &str) -> Self {
        Self::from_var_name(prefix)
    }
}

impl<T: NumType> InputFields for Vec3<T> {
    fn stage_input(prefix: &str) -> Self {
        Self::from_var_name(prefix)
    }
}

impl<T: NumType> InputFields for Vec4<T> {
    fn stage_input(prefix: &str) -> Self {
        Self::from_var_name(prefix)
    }
}

#[sealed]
impl<T: NumType> VertexField for Scalar<T> {}

#[sealed]
impl<T: NumType> VertexField for Vec2<T> {}

#[sealed]
impl<T: NumType> VertexField for Vec3<T> {}

#[sealed]
impl<T: NumType> VertexField for Vec4<T> {}

// TODO: VertexField for f32 matrix types

/// A representative of a vertex.
pub trait Vertex: Value + InputFields {}

/// A representative of vertex stage input.
pub trait Attributes: Value + InputFields {}

impl<V: Vertex> Attributes for V {}

impl<V0: Vertex, V1: Vertex> Fields for (V0, V1) {
    fn fields(prefix: &str) -> Vec<(String, Ty)> {
        V0::fields(&add_prefix(prefix, "x0"))
            .into_iter()
            .chain(V1::fields(&add_prefix(prefix, "x1")))
            .collect()
    }
}

impl<V0: Vertex, V1: Vertex> InputFields for (V0, V1) {
    fn stage_input(prefix: &str) -> Self {
        (
            V0::stage_input(&add_prefix(prefix, "x0")),
            V1::stage_input(&add_prefix(prefix, "x1")),
        )
    }
}

impl<V1: Vertex, V2: Vertex> Attributes for (V1, V2) {}

/// A representative of vertex stage output and fragment stage input.
pub trait Interpolants: Value + InputFields + OutputFields {}

/// A representative that can be stored in [`Interpolants`].
#[sealed]
pub trait InterpolantsField: Value + InputFields {
    #[doc(hidden)]
    fn must_impl() {}
}

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
pub trait Fragment: Value + OutputFields {}

/// A representative that can be stored in a [`Fragment`].
#[sealed]
pub trait FragmentField: Value {
    #[doc(hidden)]
    fn must_impl() {}
}

#[sealed]
impl<T: NumType> FragmentField for Scalar<T> {}

#[sealed]
impl<T: NumType> FragmentField for Vec2<T> {}

#[sealed]
impl<T: NumType> FragmentField for Vec3<T> {}

#[sealed]
impl<T: NumType> FragmentField for Vec4<T> {}
