use posh::{
    sl::{self, Value},
    FieldDomain, Numeric, Sl, Uniform, Vertex,
};

#[derive(Value)]
struct Foo<T: Numeric> {
    x: sl::Scalar<T>,
}

#[derive(Clone, Copy, Uniform)]
struct MyUniform2<D: FieldDomain = Sl> {
    x: D::Vec2<f32>,
    y: D::Bool,
}

#[derive(Clone, Copy, Uniform)]
struct MyUniform<D: FieldDomain = Sl> {
    x: D::Vec2<f32>,
    y: MyUniform2<D>,
}

#[derive(Clone, Copy, Vertex)]
struct MyVertex<D: FieldDomain = Sl> {
    x: D::F32,
    y: D::Vec2<f32>,
}

#[derive(Clone, Copy, Vertex)]
struct MyGenericVertex<D: FieldDomain = Sl> {
    x: D::Scalar<f32>,
    y: D::Vec2<f32>,
}

fn main() {}
