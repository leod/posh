use posh::{
    sl::{self, Value},
    Numeric, Sl, Uniform, UniformDomain, Vertex, VertexDomain,
};

#[derive(Value)]
struct Foo<T: Numeric> {
    x: sl::Scalar<T>,
}

#[derive(Clone, Copy, Uniform)]
struct MyUniform2<D: UniformDomain = Sl> {
    x: D::Vec2<f32>,
}

#[derive(Clone, Copy, Uniform)]
struct MyUniform<D: UniformDomain = Sl> {
    x: D::Vec2<f32>,
    y: MyUniform2<D>,
}

#[derive(Clone, Copy, Vertex)]
struct MyVertex<D: VertexDomain = Sl> {
    x: D::F32,
    y: D::Vec2<f32>,
}

#[derive(Clone, Copy, Vertex)]
struct MyGenericVertex<D: VertexDomain = Sl> {
    x: D::Scalar<f32>,
    y: D::Vec2<f32>,
}

fn main() {}
