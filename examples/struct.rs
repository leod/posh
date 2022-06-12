use posh::{lang::Ident, posh, IntoPosh as _, Posh, Struct, Value};

#[derive(Struct)]
pub struct Helper {
    x: i32,
    y: i32,
}

#[derive(Struct)]
pub struct Vertex {
    pos: i32,
    time: f32,
    helper: Helper,
    helper2: Helper,
}

#[posh]
fn vertex(vertex: Posh<Vertex>) -> Posh<Vertex> {
    Posh::<Vertex> {
        pos: 3 * vertex.pos,
        time: 2.0.into_posh(),
        ..vertex
    }
}

pub fn main() {
    let result = vertex(Posh::<Vertex>::from_ident(Ident::new("foo")));
    println!("{:#?}", result.expr());

    if let posh::lang::Expr::Call(expr) = result.expr() {
        if let posh::lang::Func::UserDefined(func) = expr.func {
            println!("{}", posh::lang::show::show_user_defined_funcs(&func));
        }
    }
}
