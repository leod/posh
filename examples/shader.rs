use posh::{
    sl::{self, ToValue, VaryingOutput},
    Block, BlockDom, Sl,
};

#[derive(Clone, Copy, Block)]
struct Foo<D: BlockDom = Sl> {
    bar: D::I32,
}

#[derive(Clone, Copy, Block)]
struct Globals<D: BlockDom = Sl> {
    time: D::F32,
    offset: D::Vec2,
    invert: D::I32,
    foo: Foo<D>,
    camera: D::Mat4,
    projection: D::Mat4,
}

#[derive(Clone, Copy, Block)]
struct ColorVertex<D: BlockDom = Sl> {
    position: D::Vec2,
    color: D::Vec2,
    flag: D::I32,
}

fn vertex_shader(globals: Globals, vertex: ColorVertex) -> VaryingOutput<sl::Vec4> {
    let shift = globals.offset * globals.time;
    let shift = globals
        .invert
        .eq(2)
        .branch(shift, false.to_value().branch(shift * -1.0, shift * -2.0));

    let shift2 = globals.invert.eq(3).branch(shift, {
        let x = shift * 5.0;

        false.to_value().branch(x * -1.0, x * -2.0)
    });

    let position = sl::Mat2::identity() * vertex.position + shift2 + sl::Mat2::diagonal(4.0).x_axis;

    VaryingOutput {
        output: sl::Vec4::splat(0.0),
        position: globals.projection * globals.camera * position.extend(1.0).extend(1.0),
    }
}

fn fragment_shader(_: (), varying: sl::Vec4) -> sl::Vec4 {
    varying * 3.0
}

fn main() {
    let program_def =
        posh::sl::transpile::transpile_to_program_def::<Globals, _, _, _, _, _, _, _, _, _>(
            vertex_shader,
            fragment_shader,
        );

    println!("{}", program_def.vertex_shader_source);
    println!("{}", program_def.fragment_shader_source);
}
