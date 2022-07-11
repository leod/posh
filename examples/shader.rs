use nalgebra::Vector3;
use posh::{
    shader::{FStageIn, FStageOut, Shader, UniformBlock, UniformBlockField, VStageIn, VStageOut},
    Expose, Rep,
};

#[derive(Expose)]
#[expose(UniformBlock, Vertex)]
struct ModelToClip {
    model_to_view: Vector3<f32>,
    view_to_clip: Vector3<f32>,
}

#[derive(Expose)]
#[expose(Resources)]
struct Resources {
    one: ModelToClip,
    two: ModelToClip,
}

#[derive(Expose)]
#[expose(Vertex)]
struct Vertex {
    position: [f32; 3],
    normal: [f32; 3],
    thickness: f32,
}

#[derive(Expose)]
#[expose(Vertex)]
struct Instance {
    color: [f32; 3],
}

#[derive(Expose)]
#[expose(FInputs)]
struct FInputs {
    color: [f32; 3],
    normal: [f32; 3],
}

#[derive(Expose)]
#[expose(FOutputs)]
struct FOutputs {
    color: [f32; 3],
    normal: [f32; 3],
}

fn vertex(res: Rep<Resources>, arg: VStageIn<Vertex>) -> VStageOut<FInputs> {
    let outputs = Rep::<FInputs> {
        color: posh::vec3(255.0, 0.0, 0.0),
        normal: res.two.model_to_view * arg.vertex.normal,
    };
    let position = res.one.view_to_clip * res.one.model_to_view * arg.vertex.position;

    VStageOut { outputs, position }
}

fn vertex2(res: Rep<Resources>, arg: VStageIn<(Vertex, Instance)>) -> VStageOut<FInputs> {
    let (vertex, instance) = arg.vertex;

    let outputs = Rep::<FInputs> {
        color: instance.color,
        normal: res.one.model_to_view * vertex.normal,
    };
    let position = res.one.model_to_view * vertex.position;

    VStageOut { outputs, position }
}

fn fragment(_: Rep<Resources>, arg: FStageIn<FInputs>) -> FStageOut<FOutputs> {
    let outputs = posh::var(Rep::<FOutputs> {
        color: arg.inputs.color,
        normal: arg.inputs.normal,
    });

    FStageOut::outputs(outputs)
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
