use posh::{
    value::{Domain, PoshD, RustD, Type},
    Value,
};

#[derive(Clone)]
struct Vertex<D: Domain> {
    pos: D::Field<[f32; 3]>,
    time: D::F32,
}

impl Value for Vertex<PoshD> {
    type Type = Vertex<RustD>;

    fn from_trace(trace: posh::value::Trace) -> Self {
        todo!()
    }

    fn trace(&self) -> posh::value::Trace {
        todo!()
    }
}

impl Type for Vertex<RustD> {
    type Value = Vertex<PoshD>;

    fn ty() -> posh::lang::Ty {
        todo!()
    }
}

pub fn main() {
    let vertex: Vertex<RustD> = Vertex {
        pos: [0.0, 0.0, 1.0],
        time: 5.0,
    };
}
