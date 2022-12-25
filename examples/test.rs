use posh::{
    sl::{self, ToValue, Value},
    FieldDomain, Numeric, Primitive, Sl, Uniform, Vertex,
};

#[derive(Value)]
struct Foo<T: Numeric> {
    x: sl::Scalar<T>,
}

#[derive(Clone, Copy, ToValue)]
struct MyThang<T: Primitive, D: FieldDomain = Sl> {
    x: D::Vec2<f32>,
    y: D::Scalar<T>,
}

#[derive(Clone, Copy, ToValue)]
struct MyThunk<T: Primitive, D: FieldDomain = Sl> {
    x: MyThang<T, D>,
    y: MyThang<f32, D>,
}

#[derive(Clone, Copy, ToValue, Vertex, Uniform)]
struct MyUniform1<D: FieldDomain = Sl> {
    x: D::Vec2<f32>,
    y: D::Bool,
}

#[derive(Clone, Copy, ToValue, Uniform)]
struct MyUniform2<D: FieldDomain = Sl> {
    x: D::Vec2<f32>,
    y: MyUniform1<D>,
}

#[derive(Clone, Copy, ToValue, Vertex)]
struct MyVertex<D: FieldDomain = Sl> {
    x: D::F32,
    y: D::Vec2<f32>,
}

#[derive(Clone, Copy, ToValue, Vertex)]
struct MyGenericVertex<D: FieldDomain = Sl> {
    x: D::Scalar<f32>,
    y: D::Vec2<f32>,
}

fn main() {}
