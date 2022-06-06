use posh::{value::Type, Value};

#[derive(Clone, posh::StructType)]
pub struct Vertex {
    pos: [f32; 3],
    time: f32,
}

pub fn main() {
    let vertex: Vertex = Vertex {
        pos: [0.0, 0.0, 1.0],
        time: 5.0,
    };
}
