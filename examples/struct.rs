use posh::{lang::Ident, Expose, IntoRep, MapToExpr, Rep};

#[derive(Expose)]
pub struct Helper {
    x: i32,
    y: i32,
}

#[derive(Expose)]
pub struct Vertex {
    pos: i32,
    time: f32,
    helper: Helper,
    helper2: Helper,
}

#[posh::def]
fn vertex(vertex: Rep<Vertex>) -> Rep<Vertex> {
    Rep::<Vertex> {
        pos: 3 * vertex.pos,
        time: 2.0.into_rep(),
        ..vertex
    }
}

pub fn main() {
    let result = vertex(Rep::<Vertex>::from_ident(Ident::new("foo")));
    println!("{:#?}", result.expr());

    if let posh::lang::Expr::Call(expr) = result.expr() {
        if let posh::lang::Func::UserDefined(func) = expr.func {
            println!("{}", posh::lang::show::show_user_defined_funcs(&func));
        }
    }
}
