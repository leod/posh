mod resource;
mod stage;
mod vertex;

use std::marker::PhantomData;

pub use resource::{Resource, Resources, UniformBlock};
pub use stage::{FStageIn, FStageOut, VStageIn, VStageOut};
pub use vertex::{FOutputs, VInputs, VOutputs, Vertex};

use crate::{expose::Expose, lang::Expr, MapToExpr, Rep};

/// Description of a shader.
pub struct Shader<P, V, R> {
    v_stage: ErasedVStage,
    f_stage: ErasedFStage,
    _phantom: PhantomData<(P, V, R)>,
}

struct ErasedVStage {
    pub outputs: Expr,
    pub position: Expr,
}

struct ErasedFStage {
    pub outputs: Expr,
    pub frag_depth: Option<Expr>,
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
