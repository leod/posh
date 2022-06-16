use posh::{posh, var, vec3, FSIn, FSOut, IntoValue, Po, Shader, VSIn, VSOut};

#[derive(IntoValue)]
#[posh_derive(UniformBlock)]
struct ModelToClip {
    model_to_view: [f32; 3],
    view_to_clip: [f32; 3],
}

#[derive(IntoValue)]
#[posh_derive(Resource)]
struct Resources {
    one: ModelToClip,
    two: ModelToClip,
}

#[derive(IntoValue)]
#[posh_derive(Vertex)]
struct Vertex {
    position: [f32; 3],
    normal: [f32; 3],
    thickness: f32,
}

#[derive(IntoValue)]
#[posh_derive(Vertex)]
struct Instance {
    color: [f32; 3],
}

#[derive(IntoValue)]
#[posh_derive(VertexOut)]
struct VertexOut {
    color: [f32; 3],
    normal: [f32; 3],
}

#[derive(IntoValue)]
#[posh_derive(FragmentOut)]
struct FragmentOut {
    color: [f32; 3],
    normal: [f32; 3],
}

fn vertex(resources: Po<Resources>, input: VSIn<Vertex>) -> VSOut<VertexOut> {
    let position = resources.one.view_to_clip * resources.one.model_to_view * input.vertex.position;
    let varying = Po::<VertexOut> {
        color: vec3(255.0, 0.0, 0.0),
        normal: resources.two.model_to_view * input.vertex.normal,
    };

    VSOut { position, varying }
}

fn vertex2(params: Po<Resources>, input: VSIn<(Vertex, Instance)>) -> VSOut<VertexOut> {
    let (vertex, instance) = input.vertex;

    let position = params.one.model_to_view * vertex.position;
    let varying = Po::<VertexOut> {
        color: instance.color,
        normal: params.one.model_to_view * vertex.normal,
    };

    VSOut { position, varying }
}

fn fragment(_: Po<Resources>, input: FSIn<VertexOut>) -> FSOut<FragmentOut> {
    let fragment = var(Po::<FragmentOut> {
        color: input.varying.color,
        normal: input.varying.normal,
    });

    FSOut::new(fragment)
}

struct MyShader {
    shader: Shader<Resources, Vertex, FragmentOut>,
}

struct MyShader2 {
    shader: Shader<Resources, (Vertex, Instance), FragmentOut>,
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
