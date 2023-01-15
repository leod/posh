use posh::{
    sl::{self, ToValue, Value},
    Domain, Numeric, Primitive, ResourceDomain, ResourceInterface, Sl, Uniform, Vertex,
    VertexDomain, VertexInterface,
};

#[derive(Clone, Copy, Value)]
struct Foo<T: Numeric> {
    x: sl::Scalar<T>,
}

#[derive(Clone, Copy, ToValue)]
struct MyThang<T: Primitive, D: Domain = Sl> {
    x: sl::F32,
    y: D::Scalar<T>,
}

#[derive(Clone, Copy, ToValue)]
struct MyThunk<T: Primitive, D: Domain = Sl> {
    x: MyThang<T, D>,
}

#[derive(Clone, Copy, ToValue, Uniform, Vertex)]
struct MyUniform1<D: Domain = Sl> {
    x: D::Vec2<f32>,
    y: D::Bool,
}

#[derive(Clone, Copy, ToValue, Uniform)]
struct MyUniform2<D: Domain = Sl> {
    x: D::Vec2<f32>,
    y: MyUniform1<D>,
}

#[derive(Clone, Copy, ToValue, Vertex)]
struct MyVertex<D: Domain = Sl> {
    x: D::F32,
    y: D::Vec2<f32>,
}

#[derive(Clone, Copy, ToValue, Vertex)]
struct MyNestedVertex<D: Domain = Sl> {
    x: D::Scalar<f32>,
    zzz: MyUniform1<D>,
    y: D::Vec2<f32>,
}

#[derive(VertexInterface)]
struct MyVertexIface<D: VertexDomain = Sl> {
    vertex: D::Vertex<MyVertex>,
    instance: D::Vertex<MyNestedVertex>,
}

#[derive(ResourceInterface)]
struct MyResourceIface<D: ResourceDomain = Sl> {
    uniform: D::Uniform<MyUniform1>,
}

#[derive(ResourceInterface)]
struct MyResourceIface2<D: ResourceDomain = Sl> {
    uniformxy: D::Uniform<MyUniform1>,
    bla: MyResourceIface<D>,
    zzz: D::Uniform<MyUniform1>,
}

#[derive(ResourceInterface)]
struct GenericResourceIface<R, D: ResourceDomain = Sl>
where
    R: ResourceInterface<Sl>,
{
    uniformxy: D::Uniform<MyUniform1>,
    x: D::Compose<R>,
}

fn main() {}
