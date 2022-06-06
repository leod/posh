use posh::{IntoValue as _, Value};

#[derive(Clone, posh::StructType)]
pub struct Vertex {
    pos: i32,
    time: f32,
}

pub fn main() {
    let vertex1: Vertex = Vertex { pos: 3, time: 5.0 };

    println!("{:#?}", vertex1.into_value().expr());

    let vertex2: Vertex = Vertex { pos: 3, time: 43.0 };

    println!("{:#?}", vertex2.into_value().expr());
}
