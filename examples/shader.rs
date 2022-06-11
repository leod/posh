use posh::{vec3, FragIn, FragOut, Posh, Shader, Struct, VertIn, VertOut};

#[derive(Struct)]
struct ModelToClip {
    model_to_view: [f32; 3],
    view_to_clip: [f32; 3],
}

#[derive(Struct)]
struct ParamSet {
    modelview: [f32; 3],
}

impl posh::Descriptor for ParamSet {
    fn func_arg() -> Posh<Self> {
        todo!()
    }
}

#[derive(Struct)]
struct Vertex {
    position: [f32; 3],
    normal: [f32; 3],
    thickness: f32,
}

impl posh::Vertex for Vertex {}

#[derive(Struct)]
struct Varying {
    color: [f32; 3],
    normal: [f32; 3],
}

impl posh::VertexOutputs for Varying {}

#[derive(Struct)]
struct Fragment {
    color: [f32; 3],
    normal: [f32; 3],
}

impl posh::FragmentOutputs for Fragment {}

fn vertex(params: Posh<ParamSet>, input: VertIn<Vertex>) -> VertOut<Varying> {
    VertOut {
        position: params.modelview * input.vertex.position,
        varying: Posh::<Varying> {
            color: vec3(255.0, 0.0, 0.0),
            normal: params.modelview * input.vertex.normal,
        },
    }
}

fn fragment(params: Posh<ParamSet>, input: FragIn<Varying>) -> FragOut<Fragment> {
    use posh::prelude::*;

    let fragment = var(Posh::<Fragment> {
        color: input.varying.color,
        normal: input.varying.normal,
    });

    FragOut::new(fragment)
}

struct MyShader {
    shader: Shader<ParamSet, Vertex, Fragment>,
}

fn main() {
    let my_shader = MyShader {
        shader: Shader::new(vertex, fragment),
    };
}
