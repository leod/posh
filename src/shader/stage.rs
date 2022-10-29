use crate::{Expose, FuncArg, Posh, Vec3, Vec4};

use super::{Attributes, Fragment, Interpolants};

/// Argument passed to vertex stages.
#[derive(Clone, Copy)]
pub struct VArg<V>
where
    V: Expose,
    V::Rep: Attributes,
{
    pub attrs: Posh<V>,
    pub vertex_id: Posh<i32>,
    pub instance_id: Posh<i32>,
}

impl<V> VArg<V>
where
    V: Expose,
    V::Rep: Attributes,
{
    pub(crate) fn new(attrs: Posh<V>) -> Self {
        Self {
            attrs,
            vertex_id: builtin_var("gl_VertexID"),
            instance_id: builtin_var("gl_InstanceID"),
        }
    }
}

/// Output produced by vertex stages.
pub struct VOut<W>
where
    W: Expose,
    W::Rep: Interpolants,
{
    pub interps: Posh<W>,
    pub pos: Vec3<f32>,
}

/// Argument passed to fragment stages.
pub struct FArg<W>
where
    W: Expose,
    W::Rep: Interpolants,
{
    pub interps: Posh<W>,
    pub frag_coord: Vec4<f32>,
}

impl<W> FArg<W>
where
    W: Expose,
    W::Rep: Interpolants,
{
    pub(crate) fn new(inputs: Posh<W>) -> Self {
        Self {
            interps: inputs,
            frag_coord: builtin_var("gl_FragCoord"),
        }
    }
}

/// Output produced by fragment stages.
pub struct FOut<F>
where
    F: Expose,
    F::Rep: Fragment,
{
    pub frag: Posh<F>,
    pub frag_depth: Option<Posh<f32>>,
}

impl<F> FOut<F>
where
    F: Expose,
    F::Rep: Fragment,
{
    pub fn frag(outputs: Posh<F>) -> Self {
        Self {
            frag: outputs,
            frag_depth: None,
        }
    }
}

fn builtin_var<V: FuncArg>(name: &'static str) -> V {
    V::from_var_name(name)
}
