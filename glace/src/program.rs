use posh::{Lift, Shader};

pub trait BindResources: Lift {}

pub trait BindVertexIn: Lift {}

pub trait BindFragmentOut: Lift {}

pub struct Program<R, V, F> {
    shader: Shader<R, V, F>,
}

impl<R, V, F> Program<R, V, F>
where
    R: BindResources,
    V: BindVertexIn,
    F: BindFragmentOut,
{
}
