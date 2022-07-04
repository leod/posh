use posh::{lang::Ident, IntoVal, Lift, Val, Value};

#[derive(Lift)]
#[lift(Constructible)]
pub struct Helper {
    x: i32,
    y: i32,
}

#[derive(Lift)]
#[lift(Constructible)]
pub struct Vertex {
    pos: i32,
    time: f32,
    helper: Helper,
    helper2: Helper,
}

#[posh::def]
fn vertex(vertex: Val<Vertex>) -> Val<Vertex> {
    Val::<Vertex> {
        pos: 3 * vertex.pos,
        time: 2.0.into_val(),
        ..vertex
    }
}

pub fn main() {
    let result = vertex(Val::<Vertex>::from_ident(Ident::new("foo")));
    println!("{:#?}", result.expr());

    if let posh::lang::Expr::Call(expr) = result.expr() {
        if let posh::lang::Func::UserDefined(func) = expr.func {
            println!("{}", posh::lang::show::show_user_defined_funcs(&func));
        }
    }
}
