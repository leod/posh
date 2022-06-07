use posh::{shader, FragIn, FragOut, Shader, Struct, Val, VertIn, VertOut};

#[derive(Struct)]
struct ModelToClip {
    model_to_view: [f32; 3],
    view_to_clip: [f32; 3],
}

#[derive(posh::Struct)]
struct ParamSet {
    modelview: [f32; 3],
}

/*
impl posh::Descriptor for  {
    type Param =
    fn func_arg(_name: String) -> Self {
        todo!()
    }
}
*/

#[derive(posh::Struct)]
struct Vertex {
    position: [f32; 3],
    normal: [f32; 3],
    thickness: f32,
}

impl posh::Vertex for Vertex {}

#[derive(posh::Struct)]
struct Varying {
    color: [f32; 3],
    normal: [f32; 3],
}

impl posh::Varying for Varying {}

#[derive(posh::Struct)]
struct Fragment {
    color: [f32; 3],
    normal: [f32; 3],
}

impl posh::Fragment for Fragment {}

/*
fn vertex(params: ParamSet, input: VertIn<Vertex>) -> VertOut<Varying> {
    use posh::prelude::*;

    VertOut {
        position: params.modelview * input.vertex.position,
        varying: Val::<Varying> {
            color: vec3(255.0, 0.0, 0.0),
            normal: params.modelview * input.vertex.normal,
        },
    }
}
*/

fn fragment(params: ParamSet, input: FragIn<Varying>) -> FragOut<Fragment> {
    use posh::prelude::*;

    let fragment = var(Val::<Fragment> {
        color: input.varying.color,
        normal: input.varying.normal,
    });

    FragOut::new(fragment)
}

fn main() {
    //let shader = Shader::new(vertex, fragment);
}
