//! The shading library.
//!
//! The shading library allows defining shaders in Rust functions.

#[macro_use]
mod gen_type;
mod sampler;
mod scalar;
mod tuple;
mod vec;

pub(crate) mod primitives;

use std::{collections::BTreeMap, rc::Rc};

use self::primitives::value_arg;
use super::dag::{Expr, StructType, Type};

pub use {
    sampler::Sampler2d,
    scalar::{Bool, Scalar, F32, I32, U32},
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
/// Most types in the shader language implement [`Value`]. A notable exception
/// is [`Sampler2d`]. Only types that implement [`Value`] can be used in
/// `struct` definitions.
///
/// The interface of this trait is a private implementation detail.
pub trait Value: Object {
    #[doc(hidden)]
    fn from_expr(expr: Expr) -> Self;
}

#[doc(hidden)]
pub trait Struct: Value {
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

/// Data passed from a vertex stage to a fragment stage.
///
/// The interface of this trait is a private implementation detail.
pub trait Varying: Value {
    #[doc(hidden)]
    fn shader_outputs(&self, path: &str) -> Vec<(String, Rc<Expr>)>;

    #[doc(hidden)]
    fn shader_input(path: &str) -> Self;
}

// TODO: Impl Varying.
impl Varying for Vec2<f32> {
    fn shader_outputs(&self, path: &str) -> Vec<(String, Rc<Expr>)> {
        vec![(path.to_string(), self.expr())]
    }

    fn shader_input(path: &str) -> Self {
        value_arg(path)
    }
}

impl Varying for Vec4<f32> {
    fn shader_outputs(&self, path: &str) -> Vec<(String, Rc<Expr>)> {
        vec![(path.to_string(), self.expr())]
    }

    fn shader_input(path: &str) -> Self {
        value_arg(path)
    }
}

#[derive(Debug, Clone)]
pub(crate) struct Private;

/// Input given to a vertex shader.
#[derive(Debug, Clone)]
pub struct VertexInput<V> {
    pub vertex: V,
    pub vertex_id: U32,
    pub instance_id: U32,
    pub(crate) _private: Private,
}

/// Output computed by a vertex shader.
#[derive(Debug, Clone)]
pub struct VertexOutput<W> {
    pub position: Vec4<f32>,
    pub varying: W,
    pub point_size: Option<F32>,
}

impl<W> VertexOutput<W> {
    pub fn new(position: Vec4<f32>, varying: W) -> Self {
        Self {
            position,
            varying,
            point_size: None,
        }
    }
}

/// Input given to a fragment shader.
#[derive(Debug, Clone)]
pub struct FragmentInput<W> {
    pub varying: W,
    pub fragment_coord: Vec4<f32>,
    pub front_facing: Bool,
    pub point_coord: Vec2<f32>,
    pub(crate) _private: Private,
}

/// Output computed by a fragment shader.
#[derive(Debug, Clone)]
pub struct FragmentOutput<F> {
    pub fragment: F,
    pub fragment_depth: Option<F32>,
}

impl<F> FragmentOutput<F> {
    pub fn new(fragment: F) -> Self {
        FragmentOutput {
            fragment,
            fragment_depth: None,
        }
    }
}
