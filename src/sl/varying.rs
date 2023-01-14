use std::rc::Rc;

use crate::dag::Expr;

use super::{primitives::value_arg, Object, Value, Vec2, Vec4};

/// Data passed from a vertex stage to a fragment stage.
///
/// The interface of this trait is a private implementation detail.
pub trait Varying: Value {
    #[doc(hidden)]
    fn shader_outputs(&self, path: &str) -> Vec<(String, Rc<Expr>)>;

    #[doc(hidden)]
    fn shader_input(path: &str) -> Self;
}

impl Varying for () {
    fn shader_outputs(&self, _: &str) -> Vec<(String, Rc<Expr>)> {
        Vec::new()
    }

    fn shader_input(_: &str) -> Self {}
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
