use posh::{posh, var, vec3, FSIn, FSOut, IntoPosh, Posh, Shader, VSIn, VSOut};

#[derive(IntoPosh)]
#[posh_derive(UniformBlock)]
struct ModelToClip {
    model_to_view: [f32; 3],
    view_to_clip: [f32; 3],
}

#[derive(IntoPosh)]
#[posh_derive(Resource)]
struct Resources {
    one: ModelToClip,
    two: ModelToClip,
}

#[derive(IntoPosh)]
#[posh_derive(Vertex)]
struct Vertex {
    position: [f32; 3],
    normal: [f32; 3],
    thickness: f32,
}

#[derive(IntoPosh)]
#[posh_derive(Vertex)]
struct Instance {
    color: [f32; 3],
}

#[derive(IntoPosh)]
#[posh_derive(VertexOut)]
struct VertexOut {
    color: [f32; 3],
    normal: [f32; 3],
}

#[derive(IntoPosh)]
#[posh_derive(FragmentOut)]
struct FragmentOut {
    color: [f32; 3],
    normal: [f32; 3],
}

fn vertex(resources: Posh<Resources>, input: VSIn<Vertex>) -> VSOut<VertexOut> {
    VSOut {
        position: resources.one.view_to_clip * resources.one.model_to_view * input.vertex.position,
        varying: Posh::<VertexOut> {
            color: vec3(255.0, 0.0, 0.0),
            normal: resources.two.model_to_view * input.vertex.normal,
        },
    }
}

fn vertex2(params: Posh<Resources>, input: VSIn<(Vertex, Instance)>) -> VSOut<VertexOut> {
    VSOut {
        position: params.one.model_to_view * input.vertex.0.position,
        varying: Posh::<VertexOut> {
            color: input.vertex.1.color,
            normal: params.one.model_to_view * input.vertex.0.normal,
        },
    }
}

fn fragment(_: Posh<Resources>, input: FSIn<VertexOut>) -> FSOut<FragmentOut> {
    let fragment = var(Posh::<FragmentOut> {
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
