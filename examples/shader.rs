use posh::{
    sl::{self, FragmentInput, FragmentOutput, ProgramDef, ToValue, VertexInput, VertexOutput},
    Domain, ResourceInterface, Sl, Uniform, Vertex,
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

fn vertex_shader(globals: Globals, input: VertexInput<ColorVertex>) -> VertexOutput<sl::Vec4<f32>> {
    let shift = globals.offset * globals.time;
    let shift = globals
        .invert
        .branch(shift, false.to_value().branch(shift * -1.0, shift * -2.0));

    let shift2 = globals.invert.branch(shift, {
        let x = shift * 5.0;

        false.to_value().branch(x * -1.0, x * -2.0)
    });

    let position = input.vertex.position + shift2;

    VertexOutput::new(position.to_vec4(), sl::Vec4::default())
}

fn fragment_shader(
    resources: impl ResourceInterface<Sl>,
    input: FragmentInput<sl::Vec4<f32>>,
) -> FragmentOutput<sl::Vec4<f32>> {
    todo!()
}

fn main() {
    let program_def = ProgramDef::new(vertex_shader, fragment_shader);
}
