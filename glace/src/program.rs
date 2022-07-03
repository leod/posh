use posh::{
    shader::{FOutputs, Resources, Shader, VInputs},
    Type,
};

pub trait BindResources: Type {}

pub trait BindVInputs: Type {}

pub trait BindFOutputs: Type {}

pub struct Program<R, V, F> {
    shader: Shader<R, V, F>,
}

impl<R, V, F> Program<R, V, F>
where
    R: Type,
    V: Type,
    F: Type,
    R::Val: Resources,
    V::Val: VInputs,
    F::Val: FOutputs,
    R: BindResources,
    V: BindVInputs,
    F: BindFOutputs,
{
}
