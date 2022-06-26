use posh::{FOutputs, Lift, Resources, Shader, VInputs};

pub trait BindResources: Lift {}

pub trait BindVInputs: Lift {}

pub trait BindFOutputs: Lift {}

pub struct Program<R, V, F> {
    shader: Shader<R, V, F>,
}

impl<R, V, F> Program<R, V, F>
where
    R: Lift,
    V: Lift,
    F: Lift,
    R::Type: Resources,
    V::Type: VInputs,
    F::Type: FOutputs,
    R: BindResources,
    V: BindVInputs,
    F: BindFOutputs,
{
}
