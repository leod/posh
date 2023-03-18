//! The shading language.
//!
//! `posh`'s shading language allows defining statically typed shaders in Rust.

#[macro_use]
mod gen_type;
#[macro_use]
mod scalar;
mod array;
mod mat;
mod sampler;
mod sig;
mod tuple;
mod varying;
mod vec;

pub(crate) mod codegen;
pub(crate) mod primitives;

pub mod dag;
pub mod program_def;
pub mod transpile;

use std::{collections::BTreeMap, rc::Rc};

use dag::{Expr, StructType, Type};

pub(crate) use scalar::scalar_physical;

pub use {
    array::Array,
    mat::{mat2, mat3, mat4, Mat2, Mat3, Mat4},
    sampler::{ColorSample, ColorSampler2d, ComparisonSampler2d, Depth, Sample},
    scalar::{Bool, F32, I32, U32},
    sig::{ConstParams, FragmentInput, FragmentOutput, VaryingOutput, VertexInput, VertexOutput},
    varying::Varying,
    vec::{
        bvec2, bvec3, bvec4, ivec2, ivec3, ivec4, uvec2, uvec3, uvec4, vec2, vec3, vec4, BVec2,
        BVec3, BVec4, IVec2, IVec3, IVec4, UVec2, UVec3, UVec4, Vec2, Vec3, Vec4,
    },
};

pub use posh_derive::{Value, Varying};

/// Base trait for types representing objects in the shading language.
///
/// Almost all types that implement [`Object`] also implement [`Value`]. The
/// only exception to this are intransparent types like [`Sampler2d`].
///
/// Internally, implementations of [`Object`] carry around expressions
/// describing their computation. This enables generation of shader source code.
pub trait Object: 'static {
    /// Returns the type of the object.
    fn ty() -> Type;

    /// Returns an expression for computing the object.
    fn expr(&self) -> Rc<Expr>;

    #[doc(hidden)]
    fn from_arg(name: &str) -> Self;
}

/// A transparent value in the shading language.
///
/// Only types that implement [`Value`] can be used in `struct` definitions.
///
/// Most types in the shading language implement [`Value`]. A notable exception
/// is [`Sampler2d`]. See also [`Object`].
///
/// The interface of this trait is a private implementation detail.
pub trait Value: Object + Copy {
    #[doc(hidden)]
    fn from_expr(expr: Expr) -> Self;
}

/// A transparent non-array value in the shading value.
///
/// The interface of this trait is a private implementation detail.
pub trait ValueNonArray: Value {}

#[doc(hidden)]
pub trait Struct: ValueNonArray {
    /// Returns the type of the struct.
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
