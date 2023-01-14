use posh::{
    sl::{self, ToValue, VaryingOutput},
    Domain, Sl, Uniform, Vertex,
};

#[derive(Clone, Copy, ToValue, Uniform)]
struct Foo<D: Domain = Sl> {
    bar: D::I32,
}

#[derive(Clone, Copy, ToValue, Uniform)]
struct Globals<D: Domain = Sl> {
    time: D::F32,
    offset: D::Vec2<f32>,
    invert: D::Bool,
    foo: Foo<D>,
}

#[derive(Clone, Copy, ToValue, Vertex)]
struct ColorVertex<D: Domain = Sl> {
    position: D::Vec2<f32>,
    color: D::Vec2<f32>,
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

    let position = vertex.position + shift2;

    VaryingOutput {
        varying: sl::Vec4::default(),
        position: position.to_vec4(),
    }
}

fn fragment_shader<Res>(_: Res, varying: sl::Vec4<f32>) -> sl::Vec4<f32> {
    varying * 3.0
}

fn main() {
    let program_def = posh::util::build_program_def(vertex_shader, fragment_shader);

    println!("{}", program_def.vertex_shader_source);
    println!("{}", program_def.fragment_shader_source);
}
