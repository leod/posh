//! The shading language.
//!
//! `posh`'s shading language allows defining statically typed shaders in Rust.

#[macro_use]
mod gen_type;
mod array;
mod sampler;
mod scalar;
mod shader;
mod tuple;
mod varying;
mod vec;

pub(crate) mod primitives;

use std::{collections::BTreeMap, rc::Rc};

use crate::dag::BaseType;

use super::dag::{Expr, StructType, Type};

pub(crate) use shader::Private;
pub use {
    array::Array,
    sampler::Sampler2d,
    scalar::{Bool, Scalar, F32, I32, U32},
    shader::{
        ConstInput, FragmentInput, FragmentOutput, FromFragmentInput, FromVertexInput,
        IntoFragmentOutput, IntoVertexOutput, VaryingOutput, VertexInput, VertexOutput,
    },
    varying::Varying,
    vec::{vec2, vec3, vec4, Vec2, Vec3, Vec4},
};

pub use posh_derive::{ToValue, Value};

/// Base trait for types representing objects in the shading language.
///
/// Almost all types that implement [`Object`] also implement [`Value`]. The
/// only exception to this are intransparent types like [`Sampler2d`].
///
/// Internally, implementations of [`Object`] carry around expressions
/// describing their computation. This enables generation of shader source code.
///
/// The interface of this trait is a private implementation detail.
pub trait Object: 'static {
    #[doc(hidden)]
    fn ty() -> Type;

    #[doc(hidden)]
    fn expr(&self) -> Rc<Expr>;

    #[doc(hidden)]
    fn from_arg(name: &str) -> Self;
}

/// A transparent value in the shading language.
///
/// Only types that implement [`Value`] can be used in `struct` definitions.
///
/// Most types in the shader language implement [`Value`]. A notable exception
/// is [`Sampler2d`].
///
/// The interface of this trait is a private implementation detail.
pub trait Value: Object + Copy {
    #[doc(hidden)]
    fn from_expr(expr: Expr) -> Self;
}

/// A transparent non-array value in the shading value.
///
/// The interface of this trait is a private implementation detail.
pub trait ValueNonArray: Value {
    #[doc(hidden)]
    fn base_type() -> BaseType;
}

#[doc(hidden)]
pub trait Struct: ValueNonArray {
    #[doc(hidden)]
    fn struct_type() -> Rc<StructType>;
}

#[doc(hidden)]
pub fn unique_struct_type<T: Struct>(ty: fn() -> StructType) -> Rc<StructType> {
    use std::{any::TypeId, cell::RefCell};

    std::thread_local! {
        static MAP: RefCell<BTreeMap<TypeId, Rc<StructType>>> = RefCell::new(BTreeMap::new());
    }

    let key = TypeId::of::<T>();

    if let Some(ty) = MAP.with(|map| map.borrow().get(&key).cloned()) {
        return ty;
    }

    // We must not borrow `MAP` while calling `ty`, since `ty` may also call
    // `unique_struct_type`.
    let ty = Rc::new(ty());

    MAP.with(|map| {
        map.borrow_mut().insert(key, ty.clone());
    });

    ty
}

/// A conversion to a [`Value`] in the shading language.
///
/// This is useful for converting literals to the shading language.
pub trait ToValue: Copy {
    type Output: Value;

    fn to_value(self) -> Self::Output;
}
