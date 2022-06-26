use posh::{posh, var, vec3, FStageArg, FStageRes, IntoValue, Po, Shader, VStageArg, VStageRes};

#[derive(IntoValue)]
#[posh_derive(UniformBlock)]
struct ModelToClip {
    model_to_view: [f32; 3],
    view_to_clip: [f32; 3],
}

#[derive(IntoValue)]
#[posh_derive(Resources)]
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
#[posh_derive(VOutputs)]
struct VOutputs {
    color: [f32; 3],
    normal: [f32; 3],
}

#[derive(IntoValue)]
#[posh_derive(FOutputs)]
struct FOutputs {
    color: [f32; 3],
    normal: [f32; 3],
}

fn vertex(res: Po<Resources>, arg: VStageArg<Vertex>) -> VStageRes<VOutputs> {
    let outputs = Po::<VOutputs> {
        color: vec3(255.0, 0.0, 0.0),
        normal: res.two.model_to_view * arg.vertex.normal,
    };
    let position = res.one.view_to_clip * res.one.model_to_view * arg.vertex.position;

    VStageRes { outputs, position }
}

fn vertex2(res: Po<Resources>, arg: VStageArg<(Vertex, Instance)>) -> VStageRes<VOutputs> {
    let outputs = Po::<VOutputs> {
        color: arg.vertex.1.color,
        normal: res.one.model_to_view * arg.vertex.0.normal,
    };
    let position = res.one.model_to_view * arg.vertex.0.position;

    VStageRes { outputs, position }
}

fn fragment(_: Po<Resources>, arg: FStageArg<VOutputs>) -> FStageRes<FOutputs> {
    let fragment = var(Po::<FOutputs> {
        color: arg.inputs.color,
        normal: arg.inputs.normal,
    });

    FStageRes::outputs(fragment)
}

struct MyShader {
    shader: Shader<Resources, Vertex, FOutputs>,
}

struct MyShader2 {
    shader: Shader<Resources, (Vertex, Instance), FOutputs>,
}

fn main() {
    let my_shader = MyShader {
        shader: Shader::new(vertex, fragment),
    };

    let my_shader2 = MyShader2 {
        shader: Shader::new(vertex2, fragment),
    };

    let shaduer: Shader<Resources, _, _> = Shader::new(vertex2, fragment);
}
