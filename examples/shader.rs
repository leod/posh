use posh::{var, vec3, FSIn, FSOut, Posh, Shader, Struct, VSIn, VSOut};

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

impl posh::VertexOut for Varying {}

#[derive(Struct)]
struct Fragment {
    color: [f32; 3],
    normal: [f32; 3],
}

impl posh::FragmentOut for Fragment {}

fn vertex(params: Posh<ParamSet>, input: VSIn<Vertex>) -> VSOut<Varying> {
    VSOut {
        position: params.modelview * input.vertex.position,
        varying: Posh::<Varying> {
            color: vec3(255.0, 0.0, 0.0),
            normal: params.modelview * input.vertex.normal,
        },
    }
}

fn fragment(_: Posh<ParamSet>, input: FSIn<Varying>) -> FSOut<Fragment> {
    let fragment = var(Posh::<Fragment> {
        color: input.varying.color,
        normal: input.varying.normal,
    });

    FSOut::new(fragment)
}

struct MyShader {
    shader: Shader<ParamSet, Vertex, Fragment>,
}

fn main() {
    let my_shader = MyShader {
        shader: Shader::new(vertex, fragment),
    };
}
