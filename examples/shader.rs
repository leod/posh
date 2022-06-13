use posh::{posh, var, vec3, FSIn, FSOut, IntoPosh, Posh, Shader, VSIn, VSOut};

#[derive(IntoPosh)]
#[posh(UniformBlock)]
struct ModelToClip {
    model_to_view: [f32; 3],
    view_to_clip: [f32; 3],
}

#[derive(Resources)]
struct Params {
    one: ModelToClip,
    two: ModelToClip,
}

#[derive(IntoPosh)]
#[posh(Vertex)]
struct Vertex {
    position: [f32; 3],
    normal: [f32; 3],
    thickness: f32,
}

#[derive(IntoPosh)]
#[posh(Vertex)]
struct Instance {
    color: [f32; 3],
}

#[derive(IntoPosh)]
#[posh(VertexOut)]
struct VertexOut {
    color: [f32; 3],
    normal: [f32; 3],
}

#[derive(IntoPosh)]
#[posh(FragmentOut)]
struct FragmentOut {
    color: [f32; 3],
    normal: [f32; 3],
}

fn vertex(params: Posh<ParamSet>, input: VSIn<Vertex>) -> VSOut<VertexOut> {
    VSOut {
        position: params.one.view_to_clip * params.one.model_to_view * input.vertex.position,
        varying: Posh::<VertexOut> {
            color: vec3(255.0, 0.0, 0.0),
            normal: params.two.model_to_view * input.vertex.normal,
        },
    }
}

fn vertex2(params: Posh<ParamSet>, input: VSIn<(Vertex, Instance)>) -> VSOut<Varying> {
    VSOut {
        position: params.one.model_to_view * input.vertex.0.position,
        varying: Posh::<Varying> {
            color: input.vertex.1.color,
            normal: params.one.model_to_view * input.vertex.0.normal,
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
