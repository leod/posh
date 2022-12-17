use posh::{
    sl::{self, Value},
    Numeric, Uniform, UniformDomain,
};

#[derive(Value)]
struct Foo<T: Numeric> {
    x: sl::Scalar<T>,
}

#[derive(Uniform)]
struct MyUniform<D: UniformDomain, T: Numeric> {
    x: D::Vec2<T>,
}

fn main() {}
