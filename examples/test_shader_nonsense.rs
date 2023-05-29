use posh::{
    sl::{self, VsOutput},
    Block, BlockDom, Sl,
};

#[derive(Clone, Copy, Block)]
#[repr(C)]
struct Foo<D: BlockDom> {
    bar: D::I32,
}

#[derive(Clone, Copy, Block)]
#[repr(C)]
struct Globals<D: BlockDom> {
    time: D::F32,
    offset: D::Vec2,
    invert: D::I32,
    foo: Foo<D>,
    camera: D::Mat4,
    projection: D::Mat4,
}

#[derive(Clone, Copy, Block)]
#[repr(C)]
struct ColorVertex<D: BlockDom> {
    position: D::Vec2,
    color: D::Vec2,
    flag: D::I32,
}

fn vertex_stage(globals: Globals<Sl>, vertex: ColorVertex<Sl>) -> VsOutput<sl::Vec4> {
    let shift = globals.offset * globals.time;
    let shift = sl::branch(
        globals.invert.eq(2),
        shift,
        sl::branch(false, shift * -1.0, shift * -2.0),
    );

    let shift2 = sl::branch(globals.invert.eq(3), shift, {
        let x = shift * 5.0;
        let y = !(x.as_ivec2() << 3) & 1 % sl::ivec2(1, 1);

        sl::branch(false, x * -1.0, y.as_vec2() * -2.0)
    });

    let position = sl::Mat2::IDENTITY * vertex.position + shift2 + sl::Mat2::diagonal(4.0).x_axis;

    let offsets = sl::array([
        position,
        position,
        sl::vec2(0.3, 0.4) * sl::Vec2::ONE,
        sl::vec2(0.5, 0.6),
    ]);

    VsOutput {
        interpolant: sl::Vec4::splat(0.0)
            + offsets
                .get(globals.invert.as_u32() + 3)
                .extend(1.0)
                .extend(2.0),
        clip_position: globals.projection * globals.camera * position.extend(1.0).extend(1.0),
    }
}

fn fragment_stage(_: (), interpolant: sl::Vec4) -> sl::Vec4 {
    interpolant * 3.0
}

fn main() {
    let program_def =
        posh::sl::transpile::transpile_to_program_def::<Globals<Sl>, _, _, _, _, _, _, _, _, _>(
            vertex_stage,
            fragment_stage,
        );

    println!("{}", program_def.vertex_shader_source);
    println!("{}", program_def.fragment_shader_source);
}
