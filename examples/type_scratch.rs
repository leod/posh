use posh::{
    sl::{self, Value},
    Block, BlockDomain, Numeric, Sl, UniformDomain, UniformInterface, VertexDomain,
    VertexInterface,
};

#[derive(Clone, Copy, Value)]
struct Foo<T: Numeric> {
    x: sl::Scalar<T>,
}

#[derive(Clone, Copy, Block)]
struct MyUniform1<D: BlockDomain = Sl> {
    x: D::Vec2<f32>,
    y: D::Bool,
}

#[derive(Clone, Copy, Block)]
struct MyUniform2<D: BlockDomain = Sl> {
    x: D::Vec2<f32>,
    y: MyUniform1<D>,
}

#[derive(Clone, Copy, Block)]
struct MyVertex<D: BlockDomain = Sl> {
    x: D::F32,
    y: D::Vec2<f32>,
}

#[derive(Clone, Copy, Block)]
struct MyNestedVertex<D: BlockDomain = Sl> {
    x: D::Scalar<f32>,
    zzz: MyUniform1<D>,
    y: D::Vec2<f32>,
}

#[derive(VertexInterface)]
struct MyVertexIface<D: VertexDomain = Sl> {
    vertex: D::Vertex<MyVertex>,
    instance: D::Vertex<MyNestedVertex>,
}

#[derive(UniformInterface)]
struct MyUniformIface<D: UniformDomain = Sl> {
    uniform: D::Block<MyUniform1>,
}

#[derive(UniformInterface)]
struct MyUniformIface2<D: UniformDomain = Sl> {
    uniformxy: D::Block<MyUniform1>,
    bla: MyUniformIface<D>,
    zzz: D::Block<MyUniform1>,
}

#[derive(UniformInterface)]
struct GenericUniformIface<R, D: UniformDomain = Sl>
where
    R: UniformInterface<Sl>,
{
    uniformxy: D::Block<MyUniform1>,
    x: D::Compose<R>,
}

fn main() {}
