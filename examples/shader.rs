use posh::{shader, FragIn, FragOut, Posh, Shader, Struct, Transparent, VertIn, VertOut};

#[derive(Struct)]
struct ModelToClip {
    model_to_view: [f32; 3],
    view_to_clip: [f32; 3],
}

#[derive(Struct)]
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

#[derive(Struct)]
struct Vertex {
    position: [f32; 3],
    normal: [f32; 3],
    thickness: f32,
}

impl posh::Vertex for Vertex {}

#[derive(Struct)]
struct Varying {
    color: [f32; 3],
    normal: [f32; 3],
}

impl posh::VertexOutputs for Varying {}

#[derive(Struct, Transparent)]
struct Fragment {
    color: [f32; 3],
    normal: [f32; 3],
}

impl posh::FragmentOutputs for Fragment {}

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

    let fragment = var(Posh::<Fragment> {
        color: input.varying.color,
        normal: input.varying.normal,
    });

    FragOut::new(fragment)
}

fn main() {
    //let shader = Shader::new(vertex, fragment);
}
