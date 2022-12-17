use posh::{
    sl::{self, Object, Value},
    Gl, Numeric, Sl, UniformDomain,
};

#[derive(Value)]
struct Foo<T: Numeric> {
    x: sl::Scalar<T>,
}

struct MyUniform<D: UniformDomain, T: Numeric> {
    x: D::Vec2<T>,
}

impl<T: Numeric> Object for MyUniform<Sl, T> {
    const TY: posh::dag::Ty = todo!();

    fn expr(&self) -> std::rc::Rc<posh::dag::Expr> {
        todo!()
    }
}

impl<T: Numeric> Value for MyUniform<Sl, T> {
    fn from_expr(expr: posh::dag::Expr) -> Self {
        todo!()
    }
}

impl<T: Numeric> Object for MyUniform<Gl, T> {
    const TY: posh::dag::Ty = todo!();

    fn expr(&self) -> std::rc::Rc<posh::dag::Expr> {
        todo!()
    }
}

impl<T: Numeric> Value for MyUniform<Gl, T> {
    fn from_expr(expr: posh::dag::Expr) -> Self {
        todo!()
    }
}

fn main() {}
