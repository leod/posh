use crate::{lang::Ident, Expose, FuncArg, Rep, Vec3, Vec4};

use super::{Attributes, Fragment, Interpolants};

/// Argument passed to vertex stages.
#[derive(Clone, Copy)]
pub struct VertArg<V>
where
    V: Expose,
    V::Rep: Attributes,
{
    pub attrs: Rep<V>,
    pub vertex_id: Rep<i32>,
    pub instance_id: Rep<i32>,
}

/// Output produced by vertex stages.
pub struct VertOut<W>
where
    W: Expose,
    W::Rep: Interpolants,
{
    pub interps: Rep<W>,
    pub position: Vec3<f32>,
}

/// Argument passed to fragment stages.
pub struct FragArg<W>
where
    W: Expose,
    W::Rep: Interpolants,
{
    pub interps: Rep<W>,
    pub frag_coord: Vec4<f32>,
}

/// Output produced by fragment stages.
pub struct FragOut<F>
where
    F: Expose,
    F::Rep: Fragment,
{
    pub frag: Rep<F>,
    pub frag_depth: Option<Rep<f32>>,
}

fn builtin_var<V: FuncArg>(name: &'static str) -> V {
    V::from_ident(Ident::new(name))
}

impl<V> VertArg<V>
where
    V: Expose,
    V::Rep: Attributes,
{
    fn new(attrs: Rep<V>) -> Self {
        Self {
            attrs,
            vertex_id: builtin_var("gl_VertexID"),
            instance_id: builtin_var("gl_InstanceID"),
        }
    }

    pub(crate) fn stage_arg() -> Self {
        // FIXME: stage arg handling
        Self::new(Rep::<V>::from_ident(Ident::new("input")))
    }
}

impl<W> FragArg<W>
where
    W: Expose,
    W::Rep: Interpolants,
{
    fn new(inputs: Rep<W>) -> Self {
        Self {
            interps: inputs,
            frag_coord: builtin_var("gl_FragCoord"),
        }
    }

    pub(crate) fn stage_arg() -> Self {
        // FIXME: stage arg handling
        Self::new(Rep::<W>::from_ident(Ident::new("input")))
    }
}

impl<F> FragOut<F>
where
    F: Expose,
    F::Rep: Fragment,
{
    pub fn frag(outputs: Rep<F>) -> Self {
        Self {
            frag: outputs,
            frag_depth: None,
        }
    }
}
