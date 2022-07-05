use posh::{
    shader::{FOutputs, Resources, Shader, VInputs},
    Expose,
};

pub trait BindResources: Expose {}

pub trait BindVInputs: Expose {}

pub trait BindFOutputs: Expose {}

pub struct Program<R, V, F> {
    shader: Shader<R, V, F>,
}

impl<R, V, F> Program<R, V, F>
where
    R: BindResources,
    V: BindVInputs,
    F: BindFOutputs,
    R::Rep: Resources,
    V::Rep: VInputs,
    F::Rep: FOutputs,
{
}
