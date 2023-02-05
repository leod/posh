use posh::{
    sl::{self, ToValue, VaryingOutput},
    Block, BlockDomain, Sl,
};

#[derive(Clone, Copy, Block)]
struct Foo<D: BlockDomain = Sl> {
    bar: D::I32,
}

#[derive(Clone, Copy, Block)]
struct Globals<D: BlockDomain = Sl> {
    time: D::F32,
    offset: D::Vec2<f32>,
    invert: D::Bool,
    foo: Foo<D>,
    camera: D::Mat4,
    projection: D::Mat4,
}

#[derive(Clone, Copy, Block)]
struct ColorVertex<D: BlockDomain = Sl> {
    position: D::Vec2<f32>,
    color: D::Vec2<f32>,
    flags: D::Vec4<bool>,
}

fn vertex_shader(globals: Globals, vertex: ColorVertex) -> VaryingOutput<sl::Vec4<f32>> {
    let shift = globals.offset * globals.time;
    let shift = globals
        .invert
        .branch(shift, false.to_value().branch(shift * -1.0, shift * -2.0));

    let shift2 = globals.invert.branch(shift, {
        let x = shift * 5.0;

        false.to_value().branch(x * -1.0, x * -2.0)
    });

    let position = sl::Mat2::identity() * vertex.position + shift2 + sl::Mat2::diagonal(4.0).x;

    VaryingOutput {
        varying: sl::Vec4::default(),
        position: globals.projection * globals.camera * position.extend(1.0).extend(1.0),
    }
}

fn fragment_shader<Res>(_: Res, varying: sl::Vec4<f32>) -> sl::Vec4<f32> {
    varying * 3.0
}

fn main() {
    let program_def = posh::util::compile_to_program_def(vertex_shader, fragment_shader);

    println!("{}", program_def.vertex_shader_source);
    println!("{}", program_def.fragment_shader_source);
}
