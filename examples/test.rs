use posh::{
    sl::{self, Value},
    Numeric, Uniform, UniformDomain,
};

#[derive(Value)]
struct Foo<T: Numeric> {
    x: sl::Scalar<T>,
}

#[derive(Uniform)]
struct MyUniform<D: UniformDomain> {
    x: D::Vec2<f32>,
}

fn main() {}
