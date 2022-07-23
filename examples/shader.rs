use nalgebra::Vector3;
use posh::{
    shader::{show::show_shader, FArg, FOut, Shader, VArg, VOut},
    Expose, Rep,
};

#[derive(Expose)]
#[expose(UniformBlock, Vertex)]
struct Transforms {
    world_to_view: Vector3<f32>,
    view_to_clip: Vector3<f32>,
}

#[derive(Expose)]
#[expose(Resources)]
struct Resources {
    camera: Transforms,
    shadow: Transforms,
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
    model_to_world: Vector3<f32>,
    color: [f32; 3],
}

#[derive(Expose)]
#[expose(Interpolants)]
struct Interps {
    color: [f32; 3],
    normal: [f32; 3],
}

#[derive(Expose)]
#[expose(Fragment)]
struct Frag {
    color: [f32; 3],
    normal: [f32; 3],
}

#[posh::def]
fn transform(ts: Rep<Transforms>, pos: posh::Vec3<f32>) -> posh::Vec3<f32> {
    ts.view_to_clip * ts.world_to_view * pos
}

fn vertex_stage(res: Rep<Resources>, arg: VArg<Vertex>) -> VOut<Interps> {
    let interps = Rep::<Interps> {
        color: posh::vec3(255.0, 0.0, 0.0),
        normal: res.shadow.world_to_view * arg.attrs.normal,
    };
    let pos = transform(res.camera, arg.attrs.position);

    VOut { interps, pos }
}

fn vertex_stage_instanced(res: Rep<Resources>, arg: VArg<(Vertex, Instance)>) -> VOut<Interps> {
    let (vertex, instance) = posh::var(arg.attrs);

    let interps = Rep::<Interps> {
        color: instance.color,
        normal: res.shadow.world_to_view * vertex.normal,
    };
    let pos = transform(res.camera, instance.model_to_world * vertex.position);

    VOut { interps, pos }
}

fn fragment_stage(_: Rep<Resources>, arg: FArg<Interps>) -> FOut<Frag> {
    let frag = posh::var(Rep::<Frag> {
        color: arg.interps.color,
        normal: arg.interps.normal,
    });

    FOut::frag(frag)
}

struct MyShader {
    shader: Shader<Resources, Vertex, Frag>,
}

struct MyShader2 {
    shader: Shader<Resources, (Vertex, Instance), Frag>,
}

fn main() {
    let shader = Shader::<Resources, _, _>::new(vertex_stage, fragment_stage);
    let shader_instanced = Shader::<Resources, _, _>::new(vertex_stage_instanced, fragment_stage);

    println!("{}", show_shader(shader_instanced.erased()))
}
