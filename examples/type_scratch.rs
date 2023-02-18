use posh::{
    sl::{self, Value},
    Block, BlockView, Logical, UniformData, UniformDataView, VertexData, VertexDataView,
};

#[derive(Clone, Copy, Value)]
struct Foo {
    x: sl::Vec3,
}

#[derive(Clone, Copy, Block)]
struct MyUniform1<V: BlockView = Logical> {
    x: V::Vec2,
    y: V::Bool,
}

#[derive(Clone, Copy, Block)]
struct MyUniform2<V: BlockView = Logical> {
    x: V::Vec2,
    y: MyUniform1<V>,
}

#[derive(Clone, Copy, Block)]
struct MyVertex<V: BlockView = Logical> {
    x: V::F32,
    y: V::Vec2,
}

#[derive(Clone, Copy, Block)]
struct MyNestedVertex<D: BlockView = Logical> {
    x: D::F32,
    zzz: MyUniform1<D>,
    y: D::Vec2,
}

#[derive(VertexData)]
struct MyVertexIface<D: VertexDataView = Logical> {
    vertex: D::Block<MyVertex>,
    instance: D::Block<MyNestedVertex>,
}

#[derive(UniformData)]
struct MyUniformIface<D: UniformDataView = Logical> {
    uniform: D::Block<MyUniform1>,
}

#[derive(UniformData)]
struct MyUniformIface2<D: UniformDataView = Logical> {
    uniformxy: D::Block<MyUniform1>,
    bla: MyUniformIface<D>,
    zzz: D::Block<MyUniform1>,
}

#[derive(UniformData)]
struct GenericUniformIface<R, D: UniformDataView = Logical>
where
    R: UniformData<Logical>,
{
    uniformxy: D::Block<MyUniform1>,
    x: D::Compose<R>,
}

fn main() {}
