use posh::{posh, IntoValue as _, Posh, StructType, Value};

#[derive(StructType, Default)]
pub struct Helper {
    x: i32,
    y: i32,
}

#[derive(StructType)]
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
        time: 2.0.into_value(),
        ..vertex
    }
}

pub fn main() {
    let vertex1: Vertex = Vertex {
        pos: 3,
        time: 5.0,
        helper: Default::default(),
        helper2: Default::default(),
    };

    //println!("{:#?}", vertex1.into_value().expr());

    let vertex2: Vertex = Vertex {
        pos: 3,
        time: 43.0,
        helper: Default::default(),
        helper2: Default::default(),
    };

    let result = vertex(vertex2);
    println!("{:#?}", result);

    if let posh::lang::Expr::Call(expr) = result.expr() {
        if let posh::lang::Func::UserDefined(func) = expr.func {
            println!("{}", posh::lang::show::show_user_defined_funcs(&func));
        }
    }
}
