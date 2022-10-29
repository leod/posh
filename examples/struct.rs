use nalgebra::Vector3;
use posh::{
    expose::compile::compile1,
    rep,
    var_form::{
        show::{show_func_defs, show_struct_defs},
        StructDefs, VarFormFuncDefs,
    },
    Expose, IntoPosh, Posh, ScalarType,
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
    fn foo(&self) -> Posh<f32> {
        0.0.into()
    }
}

#[posh::def]
fn vertex(vertex: Posh<test::Vertex>) -> Posh<test::Vertex> {
    Posh::<test::Vertex> {
        pos: 3 * vertex.pos,
        time: 2.0.into_posh(),
        ..vertex
    }
}

pub fn main() {
    let func_def = compile1(vertex).unwrap();

    let mut structs = StructDefs::new();
    let funcs = VarFormFuncDefs::from_func_def(&func_def, &mut structs);
    println!("{}", show_struct_defs(&structs));
    println!("{}", show_func_defs(&funcs));
}
