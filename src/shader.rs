use std::marker::PhantomData;

use crate::{
    lang::{Expr, Ident},
    value::{ConstructibleVal, Type},
    TypedVal, Value, Vec3, Vec4,
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

pub trait Vertex: ConstructibleVal {}

pub trait VInputs: ConstructibleVal {}

impl<V: Vertex> VInputs for V {}

impl<V1: Vertex, V2: Vertex> VInputs for (V1, V2) {}

pub trait VOutputs: ConstructibleVal {}

pub trait FOutputs: ConstructibleVal {}

#[derive(Clone, Copy)]
pub struct VStageIn<V: Type> {
    pub vertex: Value<V>,
    pub vertex_id: Value<i32>,
    pub instance_id: Value<i32>,
}

pub struct VStageOut<W: Type> {
    pub outputs: Value<W>,
    pub position: Vec3<f32>,
}

pub struct FStageIn<W: Type> {
    pub inputs: Value<W>,
    pub frag_coord: Vec4<f32>,
}

pub struct FStageOut<F: Type> {
    pub outputs: Value<F>,
    pub frag_depth: Option<Value<f32>>,
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

fn builtin_var<V: TypedVal>(name: &'static str) -> V {
    V::from_ident(Ident::new(name))
}

impl<V> VStageIn<V>
where
    V: Type,
    V::Val: VInputs,
{
    fn new(vertex: Value<V>) -> Self {
        Self {
            vertex,
            vertex_id: builtin_var("gl_VertexID"),
            instance_id: builtin_var("gl_InstanceID"),
        }
    }

    fn func_arg() -> Self {
        Self::new(Value::<V>::from_ident(Ident::new("input")))
    }
}

impl<W> FStageIn<W>
where
    W: Type,
    W::Val: VOutputs,
{
    fn new(inputs: Value<W>) -> Self {
        Self {
            inputs,
            frag_coord: builtin_var("gl_FragCoord"),
        }
    }

    fn func_arg() -> Self {
        Self::new(Value::<W>::from_ident(Ident::new("input")))
    }
}

impl<F> FStageOut<F>
where
    F: Type,
    F::Val: FOutputs,
{
    pub fn outputs(outputs: Value<F>) -> Self {
        Self {
            outputs,
            frag_depth: None,
        }
    }
}

impl ErasedVStage {
    fn new<W>(out: VStageOut<W>) -> Self
    where
        W: Type,
        W::Val: VOutputs,
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
        F: Type,
        F::Val: FOutputs,
    {
        Self {
            outputs: out.outputs.expr(),
            frag_depth: out.frag_depth.map(|v| v.expr()),
        }
    }
}

impl<R, V, F> Shader<R, V, F>
where
    R: Type,
    V: Type,
    F: Type,
    R::Val: Resources,
    V::Val: VInputs,
    F::Val: FOutputs,
{
    pub fn new<W, VStage, FStage>(v_stage: VStage, f_stage: FStage) -> Self
    where
        W: Type,
        W::Val: VOutputs,
        VStage: FnOnce(Value<R>, VStageIn<V>) -> VStageOut<W>,
        FStage: FnOnce(Value<R>, FStageIn<W>) -> FStageOut<F>,
    {
        let v_out = v_stage(R::Val::func_arg(), VStageIn::func_arg());
        let f_out = f_stage(R::Val::func_arg(), FStageIn::func_arg());

        let v_stage = ErasedVStage::new(v_out);
        let f_stage = ErasedFStage::new(f_out);

        Self {
            v_stage,
            f_stage,
            _phantom: PhantomData,
        }
    }
}
