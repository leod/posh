mod program_def;
mod sampler;
mod scalar;
mod tuple;
mod vec;

#[doc(hidden)]
pub mod primitives;

use std::{collections::BTreeMap, rc::Rc};

use crate::dag::StructType;

use super::dag::{Expr, Type};

pub use posh_derive::{ToValue, Value};

pub use {
    // TODO: Remove ProgramDef re-export.
    program_def::{FragmentInput, FragmentOutput, ProgramDef, VertexInput, VertexOutput},
    sampler::Sampler2d,
    scalar::{Bool, Scalar, F32, I32, U32},
    vec::{Vec2, Vec4},
};

/// An object that can be represented in the shading language.
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

/// An object that can be stored as a value in the shading language.
///
/// The interface of this trait is a private implementation detail.
pub trait Value: Object {
    #[doc(hidden)]
    fn from_expr(expr: Expr) -> Self;
}

/// A [`Value`] that has a struct type in the shading language.
///
/// The interface of this trait is a private implementation detail.
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

    // NOTE: We must not borrow `MAP` while calling `ty`, since `ty` may also
    // call `unique_struct_type`.
    let ty = Rc::new(ty());

    MAP.with(|map| {
        map.borrow_mut().insert(key, ty.clone());
    });

    ty
}

/// A conversion to a [`Value`] in the shading language.
pub trait ToValue: Copy {
    type Output: Value;

    fn to_value(self) -> Self::Output;
}

/// Data passed from a vertex stage to a fragment stage.
pub trait Varying: Value {
    fn attributes(path: &str) -> Vec<(String, Type)>;

    fn shader_outputs(&self) -> Vec<Rc<Expr>>;
}

// TODO: Impl Varying.
impl Varying for Vec4<f32> {
    fn attributes(path: &str) -> Vec<(String, Type)> {
        vec![(path.to_string(), <Self as Object>::ty())]
    }

    fn shader_outputs(&self) -> Vec<Rc<Expr>> {
        vec![self.expr()]
    }
}
