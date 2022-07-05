use std::marker::PhantomData;

use crate::{
    expose::{Expose, Value},
    lang::{Expr, Ident},
    MapToExpr, Rep, Vec3, Vec4,
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

pub trait Vertex: Value {}

pub trait VInputs: Value {}

impl<V: Vertex> VInputs for V {}

impl<V1: Vertex, V2: Vertex> VInputs for (V1, V2) {}

pub trait VOutputs: Value {}

pub trait FOutputs: Value {}

#[derive(Clone, Copy)]
pub struct VStageIn<V: Expose> {
    pub vertex: Rep<V>,
    pub vertex_id: Rep<i32>,
    pub instance_id: Rep<i32>,
}

pub struct VStageOut<W: Expose> {
    pub outputs: Rep<W>,
    pub position: Vec3<f32>,
}

pub struct FStageIn<W: Expose> {
    pub inputs: Rep<W>,
    pub frag_coord: Vec4<f32>,
}

pub struct FStageOut<F: Expose> {
    pub outputs: Rep<F>,
    pub frag_depth: Option<Rep<f32>>,
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

    fn func_arg() -> Self {
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

    fn func_arg() -> Self {
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

impl ErasedVStage {
    fn new<W>(out: VStageOut<W>) -> Self
    where
        W: Expose,
        W::Rep: VOutputs,
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
        F: Expose,
        F::Rep: FOutputs,
    {
        Self {
            outputs: out.outputs.expr(),
            frag_depth: out.frag_depth.map(|v| v.expr()),
        }
    }
}

impl<R, V, F> Shader<R, V, F>
where
    R: Expose,
    V: Expose,
    F: Expose,
    R::Rep: Resources,
    V::Rep: VInputs,
    F::Rep: FOutputs,
{
    pub fn new<W, VStage, FStage>(v_stage: VStage, f_stage: FStage) -> Self
    where
        W: Expose,
        W::Rep: VOutputs,
        VStage: FnOnce(Rep<R>, VStageIn<V>) -> VStageOut<W>,
        FStage: FnOnce(Rep<R>, FStageIn<W>) -> FStageOut<F>,
    {
        let v_out = v_stage(R::Rep::func_arg(), VStageIn::func_arg());
        let f_out = f_stage(R::Rep::func_arg(), FStageIn::func_arg());

        let v_stage = ErasedVStage::new(v_out);
        let f_stage = ErasedFStage::new(f_out);

        Self {
            v_stage,
            f_stage,
            _phantom: PhantomData,
        }
    }
}
