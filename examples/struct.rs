use nalgebra::Vector3;
use posh::{lang::Ident, rep, Expose, FuncArg, IntoRep, Rep};

#[derive(Expose)]
pub struct Helper {
    x: i32,
    y: i32,
    foo: Vector3<f32>,
}

mod test {
    use super::*;

    #[derive(Expose)]
    pub struct Vertex {
        pub pos: i32,
        pub time: f32,
        pub helper: Helper,
        pub helper2: Helper,
    }
}

impl rep!(test::Vertex) {}

#[posh::def]
fn vertex(vertex: Rep<test::Vertex>) -> Rep<test::Vertex> {
    Rep::<test::Vertex> {
        pos: 3 * vertex.pos,
        time: 2.0.into_rep(),
        ..vertex
    }
}

pub fn main() {
    let result = vertex(Rep::<test::Vertex>::from_ident(Ident::new("foo")));
    println!("{:#?}", result.expr());

    if let posh::lang::Expr::Call(expr) = result.expr() {
        if let posh::lang::Func::UserDefined(func) = expr.func {
            println!("{}", posh::lang::show::show_user_defined_funcs(&func));
        }
    }
}
