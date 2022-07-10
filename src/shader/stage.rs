use crate::{lang::Ident, Expose, MapToExpr, Rep, Vec3, Vec4};

use super::{FOutputs, VInputs, VOutputs};

/// Vertex stage input.
#[derive(Clone, Copy)]
pub struct VStageIn<V>
where
    V: Expose,
    V::Rep: VInputs,
{
    pub vertex: Rep<V>,
    pub vertex_id: Rep<i32>,
    pub instance_id: Rep<i32>,
}

/// Vertex stage output.
pub struct VStageOut<W>
where
    W: Expose,
    W::Rep: VOutputs,
{
    pub outputs: Rep<W>,
    pub position: Vec3<f32>,
}

/// Fragment stage input.
pub struct FStageIn<W>
where
    W: Expose,
    W::Rep: VOutputs,
{
    pub inputs: Rep<W>,
    pub frag_coord: Vec4<f32>,
}

/// Fragment stage output.
pub struct FStageOut<F>
where
    F: Expose,
    F::Rep: FOutputs,
{
    pub outputs: Rep<F>,
    pub frag_depth: Option<Rep<f32>>,
}

fn builtin_var<V: MapToExpr>(name: &'static str) -> V {
    V::from_ident(Ident::new(name))
}

impl<V> VStageIn<V>
where
    V: Expose,
    V::Rep: VInputs,
{
    fn new(vertex: Rep<V>) -> Self {
        Self {
            vertex,
            vertex_id: builtin_var("gl_VertexID"),
            instance_id: builtin_var("gl_InstanceID"),
        }
    }

    pub(crate) fn func_arg() -> Self {
        Self::new(Rep::<V>::from_ident(Ident::new("input")))
    }
}

impl<W> FStageIn<W>
where
    W: Expose,
    W::Rep: VOutputs,
{
    fn new(inputs: Rep<W>) -> Self {
        Self {
            inputs,
            frag_coord: builtin_var("gl_FragCoord"),
        }
    }

    pub(crate) fn func_arg() -> Self {
        Self::new(Rep::<W>::from_ident(Ident::new("input")))
    }
}

impl<F> FStageOut<F>
where
    F: Expose,
    F::Rep: FOutputs,
{
    pub fn outputs(outputs: Rep<F>) -> Self {
        Self {
            outputs,
            frag_depth: None,
        }
    }
}
