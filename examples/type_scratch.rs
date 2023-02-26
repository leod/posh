use posh::{
    sl::{self, Value},
    Block, BlockFields, SlView, Uniform, UniformFields, Vertex, VertexFields,
};

#[derive(Clone, Copy, Value)]
struct Foo {
    x: sl::Vec3,
}

#[derive(Clone, Copy, Block)]
struct MyUniform1<F: BlockFields = SlView> {
    x: F::Vec2,
    y: F::Bool,
}

#[derive(Clone, Copy, Block)]
struct MyUniform2<F: BlockFields = SlView> {
    x: F::Vec2,
    y: MyUniform1<F>,
}

#[derive(Clone, Copy, Block)]
struct MyVertex<F: BlockFields = SlView> {
    x: F::F32,
    y: F::Vec2,
}

#[derive(Clone, Copy, Block)]
struct MyNestedVertex<F: BlockFields = SlView> {
    x: F::F32,
    zzz: MyUniform1<F>,
    y: F::Vec2,
}

#[derive(Vertex)]
struct MyVertexIface<F: VertexFields = SlView> {
    vertex: F::Block<MyVertex>,
    instance: F::Block<MyNestedVertex>,
}

#[derive(Uniform)]
struct MyUniformIface<F: UniformFields = SlView> {
    uniform: F::Block<MyUniform1>,
}

#[derive(Uniform)]
struct MyUniformIface2<F: UniformFields = SlView> {
    uniformxy: F::Block<MyUniform1>,
    bla: MyUniformIface<F>,
    zzz: F::Block<MyUniform1>,
}

#[derive(Uniform)]
struct GenericUniformIface<U, F: UniformFields = SlView>
where
    U: Uniform<SlView>,
{
    uniformxy: F::Block<MyUniform1>,
    x: F::Compose<U>,
}

fn main() {}
