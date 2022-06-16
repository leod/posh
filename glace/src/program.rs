use posh::{Shader, Binding};

pub trait ResourceBinding: Binding {

}

pub trait VertexBinding: Binding {

}

pub struct Program<R, V, F> {
    shader: Shader<R, V, F>,
}

impl<R, V, F> Program<R, V, F>
where
    R: ResourceBinding,
{

}