use nalgebra::Vector3;
use posh::{
    expose::compile::compile1,
    lang::{defs::Defs, show::show_defs, Ident},
    rep, Expose, FuncArg, IntoRep, Rep, ScalarType,
};

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

#[derive(Expose)]
pub struct Generic<T: ScalarType> {
    x: posh::Vec3<T>,
}

impl<T: ScalarType> rep!(Generic<T>) {
    fn foo(&self) -> Rep<f32> {
        0.0.into()
    }
}

#[posh::def]
fn vertex(vertex: Rep<test::Vertex>) -> Rep<test::Vertex> {
    Rep::<test::Vertex> {
        pos: 3 * vertex.pos,
        time: 2.0.into_rep(),
        ..vertex
    }
}

pub fn main() {
    let func_def = compile1(vertex).unwrap();

    println!("{}", show_defs(&Defs::from_func_def(&func_def)));
}
