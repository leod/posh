use posh::{var, vec3, FSIn, FSOut, IntoPosh, Posh, Shader, VSIn, VSOut};

#[derive(IntoPosh)]
struct ModelToClip {
    model_to_view: [f32; 3],
    view_to_clip: [f32; 3],
}

#[derive(IntoPosh)]
struct ParamSet {
    modelview: [f32; 3],
}

impl posh::Resource for _PoshParamSet {
    fn func_arg() -> Self {
        todo!()
    }
}

#[derive(IntoPosh)]
struct Vertex {
    position: [f32; 3],
    normal: [f32; 3],
    thickness: f32,
}

impl posh::Vertex for _PoshVertex {}

#[derive(IntoPosh)]
struct Instance {
    color: [f32; 3],
}

impl posh::Vertex for _PoshInstance {}

#[derive(IntoPosh)]
struct Varying {
    color: [f32; 3],
    normal: [f32; 3],
}

impl posh::VertexOut for _PoshVarying {}

#[derive(IntoPosh)]
struct Fragment {
    color: [f32; 3],
    normal: [f32; 3],
}

impl posh::FragmentOut for _PoshFragment {}

fn vertex(params: Posh<ParamSet>, input: VSIn<Vertex>) -> VSOut<Varying> {
    VSOut {
        position: params.modelview * input.vertex.position,
        varying: Posh::<Varying> {
            color: vec3(255.0, 0.0, 0.0),
            normal: params.modelview * input.vertex.normal,
        },
    }
}

fn vertex2(params: Posh<ParamSet>, input: VSIn<(Vertex, Instance)>) -> VSOut<Varying> {
    VSOut {
        position: params.modelview * input.vertex.0.position,
        varying: Posh::<Varying> {
            color: input.vertex.1.color,
            normal: params.modelview * input.vertex.0.normal,
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

struct MyShader2 {
    shader: Shader<ParamSet, (Vertex, Instance), Fragment>,
}

fn main() {
    let my_shader = MyShader {
        shader: Shader::new(vertex, fragment),
    };

    let my_shader2 = MyShader2 {
        shader: Shader::new(vertex2, fragment),
    };

    let shaduer: Shader<ParamSet, _, _> = Shader::new(vertex2, fragment);
}
