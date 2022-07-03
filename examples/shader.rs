use posh::{
    shader::{FStageIn, FStageOut, Shader, VStageIn, VStageOut},
    var, vec3, IntoVal, Value,
};

#[derive(Type, IntoVal)]
#[val(UniformBlock)]
struct ModelToClip {
    model_to_view: [f32; 3],
    view_to_clip: [f32; 3],
}

#[derive(Type)]
#[val(Resources)]
struct Resources {
    one: ModelToClip,
    two: ModelToClip,
}

#[derive(Type, IntoVal)]
#[val(Vertex)]
struct Vertex {
    position: [f32; 3],
    normal: [f32; 3],
    thickness: f32,
}

#[derive(Type, IntoVal)]
#[val(Vertex)]
struct Instance {
    color: [f32; 3],
}

#[derive(Type, IntoVal)]
#[val(VOutputs)]
struct VOutputs {
    color: [f32; 3],
    normal: [f32; 3],
}

#[derive(Type, IntoVal)]
#[val(FOutputs)]
struct FOutputs {
    color: [f32; 3],
    normal: [f32; 3],
}

fn vertex(res: Value<Resources>, arg: VStageIn<Vertex>) -> VStageOut<VOutputs> {
    let outputs = Value::<VOutputs> {
        color: vec3(255.0, 0.0, 0.0),
        normal: res.two.model_to_view * arg.vertex.normal,
    };
    let position = res.one.view_to_clip * res.one.model_to_view * arg.vertex.position;

    VStageOut { outputs, position }
}

fn vertex2(res: Value<Resources>, arg: VStageIn<(Vertex, Instance)>) -> VStageOut<VOutputs> {
    let (instance, vertex) = arg.vertex;

    let outputs = Value::<VOutputs> {
        color: instance.color,
        normal: res.one.model_to_view * vertex.normal,
    };
    let position = res.one.model_to_view * vertex.position;

    VStageOut { outputs, position }
}

fn fragment(_: Value<Resources>, arg: FStageIn<VOutputs>) -> FStageOut<FOutputs> {
    let outputs = var(Value::<FOutputs> {
        color: arg.inputs.color,
        normal: arg.inputs.normal,
    });

    FStageOut::outputs(fragment)
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
