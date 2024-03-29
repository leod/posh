use posh::{
    gl::ColorSampler2d,
    sl::{self, Value},
    Block, BlockDom, Sl, UniformInterface, UniformInterfaceDom, VsInterface, VsInterfaceDom,
};

#[derive(Clone, Copy, Value)]
struct Foo {
    x: sl::Vec3,
}

#[derive(Clone, Copy, Block)]
#[repr(C)]
struct MyUniform1<D: BlockDom = Sl> {
    x: D::Vec2,
    y: D::IVec2,
}

#[derive(Clone, Copy, Block)]
#[repr(C)]
struct MyUniform2<D: BlockDom = Sl> {
    x: D::Vec2,
    y: MyUniform1<D>,
}

#[derive(Clone, Copy, Block)]
#[repr(C)]
struct MyVertex<D: BlockDom = Sl> {
    x: D::F32,
    y: D::Vec2,
}

#[derive(Clone, Copy, Block)]
#[repr(C)]
struct MyNestedVertex<D: BlockDom = Sl> {
    x: D::F32,
    zzz: MyUniform1<D>,
    y: D::Vec2,
}

#[derive(Copy, Clone, VsInterface)]
struct MyVertexIface<D: VsInterfaceDom = Sl> {
    vertex: D::Block<MyVertex>,
    instance: D::Block<MyNestedVertex>,
}

#[derive(UniformInterface)]
struct MyUniformIface<D: UniformInterfaceDom = Sl> {
    uniform: D::Block<MyUniform1>,
    samplers: [D::ColorSampler2d<sl::Vec4>; 3],
}

#[derive(UniformInterface)]
struct MyUniformIface2<D: UniformInterfaceDom = Sl> {
    uniformxy: D::Block<MyUniform1>,
    bla: MyUniformIface<D>,
    zzz: D::Block<MyUniform1>,
}

#[derive(UniformInterface)]
struct GenericUniformIface<U, D: UniformInterfaceDom = Sl>
where
    U: UniformInterface<Sl>,
{
    uniformxy: D::Block<MyUniform1>,
    x: D::UniformInterface<U>,
}

#[derive(sl::Const)]
pub struct Consts {}

#[derive(sl::Const)]
pub struct Const<T: sl::Const> {
    x: T,
    y: Vec<T>,
    z: f32,
}

fn main() {}
