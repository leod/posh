use nalgebra::Vector3;
use posh::{
    shader::{FragArg, FragOut, Shader, VertArg, VertOut},
    Expose, Rep, Sampler2,
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
#[expose(Interpolants)]
struct Interpolants {
    color: [f32; 3],
    normal: [f32; 3],
}

#[derive(Expose)]
#[expose(Fragment)]
struct Fragment {
    color: [f32; 3],
    normal: [f32; 3],
}

fn vert(res: Rep<Resources>, arg: VertArg<Vertex>) -> VertOut<Interpolants> {
    let interps = Rep::<Interpolants> {
        color: posh::vec3(255.0, 0.0, 0.0),
        normal: res.two.model_to_view * arg.attrs.normal,
    };
    let position = res.one.view_to_clip * res.one.model_to_view * arg.attrs.position;

    VertOut { interps, position }
}

fn vert2(res: Rep<Resources>, arg: VertArg<(Vertex, Instance)>) -> VertOut<Interpolants> {
    let (vertex, instance) = arg.attrs;

    let interps = Rep::<Interpolants> {
        color: instance.color,
        normal: res.one.model_to_view * vertex.normal,
    };
    let position = res.one.model_to_view * vertex.position;

    VertOut { interps, position }
}

fn frag(_: Rep<Resources>, arg: FragArg<Interpolants>) -> FragOut<Fragment> {
    let frag = posh::var(Rep::<Fragment> {
        color: arg.interps.color,
        normal: arg.interps.normal,
    });

    FragOut::frag(frag)
}

struct MyShader {
    shader: Shader<Resources, Vertex, Fragment>,
}

struct MyShader2 {
    shader: Shader<Resources, (Vertex, Instance), Fragment>,
}

fn main() {
    let my_shader = MyShader {
        shader: Shader::new(vert, frag),
    };

    let my_shader2 = MyShader2 {
        shader: Shader::new(vert2, frag),
    };

    let shaduer: Shader<Resources, _, _> = Shader::new(vert2, frag);
}
