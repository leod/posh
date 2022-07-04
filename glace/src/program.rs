use posh::{
    shader::{FOutputs, Resources, Shader, VInputs},
    Lift,
};

pub trait BindResources: Lift {}

pub trait BindVInputs: Lift {}

pub trait BindFOutputs: Lift {}

pub struct Program<R, V, F> {
    shader: Shader<R, V, F>,
}

impl<R, V, F> Program<R, V, F>
where
    R: BindResources,
    V: BindVInputs,
    F: BindFOutputs,
    R::Value: Resources,
    V::Value: VInputs,
    F::Value: FOutputs,
{
}
