use std::{marker::PhantomData, rc::Rc};

use crate::{
    dag::{BaseTy, Expr, Trace, Ty},
    Numeric,
};

use super::{primitives::built_in_1, Object, Vec2, Vec4};

#[derive(Debug, Copy, Clone)]
pub struct Sampler2d<T: Numeric> {
    trace: Trace,
    _phantom: PhantomData<T>,
}

impl<T: Numeric> Object for Sampler2d<T> {
    const TY: Ty = Ty::Base(BaseTy::Sampler2d(T::NUMERIC_TY));

    fn expr(&self) -> Rc<Expr> {
        self.trace.expr()
    }
}

impl<T: Numeric> Sampler2d<T> {
    pub fn lookup(self, tex_coords: Vec2<f32>) -> Vec4<T> {
        built_in_1("texture", tex_coords)
    }
}
