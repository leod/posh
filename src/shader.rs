use std::marker::PhantomData;

use crate::{
    lang::{Expr, Ident},
    value::{Constructible, Lift},
    Po, Value, Vec3, Vec4,
};

pub trait Resource {
    fn func_arg() -> Self;
}

pub trait Resources {
    fn func_arg() -> Self;
}

impl<D> Resources for D
where
    D: Resource,
{
    fn func_arg() -> Self {
        <Self as Resource>::func_arg()
    }
}

pub trait Vertex: Constructible {}

pub trait VInputs: Constructible {}

impl<V: Vertex> VInputs for V {}

impl<V1: Vertex, V2: Vertex> VInputs for (V1, V2) {}

pub trait VOutputs: Constructible {}

pub trait FOutputs: Constructible {}

#[derive(Clone, Copy)]
pub struct VStageIn<V: Lift> {
    pub vertex: Po<V>,
    pub vertex_id: Po<i32>,
    pub instance_id: Po<i32>,
}

pub struct VStageOut<W: Lift> {
    pub outputs: Po<W>,
    pub position: Vec3<f32>,
}

pub struct FStageIn<W: Lift> {
    pub inputs: Po<W>,
    pub frag_coord: Vec4<f32>,
}

pub struct FStageOut<F: Lift> {
    pub outputs: Po<F>,
    pub frag_depth: Option<Po<f32>>,
}

struct ErasedVStage {
    pub outputs: Expr,
    pub position: Expr,
}

struct ErasedFStage {
    pub outputs: Expr,
    pub frag_depth: Option<Expr>,
}

pub struct Shader<P, V, R> {
    v_stage: ErasedVStage,
    f_stage: ErasedFStage,
    _phantom: PhantomData<(P, V, R)>,
}

// TODO: Figure out if we can have Expr::Void for this.
/*impl VertexOut<()> {
    pub fn new(position: Vec4<f32>) -> Self {
        Self {
            position,
            varying: (),
        }
    }
}*/

fn builtin_var<V: Value>(name: &'static str) -> V {
    V::from_ident(Ident::new(name))
}

impl<V> VStageIn<V>
where
    V: Lift,
    V::Type: VInputs,
{
    fn new(vertex: Po<V>) -> Self {
        Self {
            vertex,
            vertex_id: builtin_var("gl_VertexID"),
            instance_id: builtin_var("gl_InstanceID"),
        }
    }

    fn func_arg() -> Self {
        Self::new(Po::<V>::from_ident(Ident::new("input")))
    }
}

impl<W> FStageIn<W>
where
    W: Lift,
    W::Type: VOutputs,
{
    fn new(inputs: Po<W>) -> Self {
        Self {
            inputs,
            frag_coord: builtin_var("gl_FragCoord"),
        }
    }

    fn func_arg() -> Self {
        Self::new(Po::<W>::from_ident(Ident::new("input")))
    }
}

impl<F> FStageOut<F>
where
    F: Lift,
    F::Type: FOutputs,
{
    pub fn outputs(outputs: Po<F>) -> Self {
        Self {
            outputs,
            frag_depth: None,
        }
    }
}

impl ErasedVStage {
    fn new<W>(out: VStageOut<W>) -> Self
    where
        W: Lift,
        W::Type: VOutputs,
    {
        Self {
            outputs: out.outputs.expr(),
            position: out.position.expr(),
        }
    }
}

impl ErasedFStage {
    fn new<F>(out: FStageOut<F>) -> Self
    where
        F: Lift,
        F::Type: FOutputs,
    {
        Self {
            outputs: out.outputs.expr(),
            frag_depth: out.frag_depth.map(|v| v.expr()),
        }
    }
}

impl<R, V, F> Shader<R, V, F>
where
    R: Lift,
    V: Lift,
    F: Lift,
    R::Type: Resources,
    V::Type: VInputs,
    F::Type: FOutputs,
{
    pub fn new<W, VStage, FStage>(v_stage: VStage, f_stage: FStage) -> Self
    where
        W: Lift,
        W::Type: VOutputs,
        VStage: FnOnce(Po<R>, VStageIn<V>) -> VStageOut<W>,
        FStage: FnOnce(Po<R>, FStageIn<W>) -> FStageOut<F>,
    {
        let v_out = v_stage(R::Type::func_arg(), VStageIn::func_arg());
        let f_out = f_stage(R::Type::func_arg(), FStageIn::func_arg());

        let v_stage = ErasedVStage::new(v_out);
        let f_stage = ErasedFStage::new(f_out);

        Self {
            v_stage,
            f_stage,
            _phantom: PhantomData,
        }
    }
}
