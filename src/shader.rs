mod resource;

mod stage;
mod vertex;

use std::marker::PhantomData;

pub use resource::{Resource, Resources, UniformBlock, UniformBlockField};
pub use stage::{FArg, FOut, VArg, VOut};
pub use vertex::{
    Attributes, Fragment, FragmentField, Interpolants, InterpolantsField, Vertex, VertexField,
};

use crate::{expose::Expose, lang::Expr, FuncArg, Rep};

/// Description of a shader.
pub struct Shader<R, V, F> {
    v_stage: ErasedVStage,
    f_stage: ErasedFStage,
    _phantom: PhantomData<(R, V, F)>,
}

struct ErasedVStage {
    pub interps: Expr,
    pub position: Expr,
}

struct ErasedFStage {
    pub frag: Expr,
    pub frag_depth: Option<Expr>,
}

impl ErasedVStage {
    fn new<W>(out: VOut<W>) -> Self
    where
        W: Expose,
        W::Rep: Interpolants,
    {
        Self {
            interps: out.interps.expr(),
            position: out.position.expr(),
        }
    }
}

impl ErasedFStage {
    fn new<F>(out: FOut<F>) -> Self
    where
        F: Expose,
        F::Rep: Fragment,
    {
        Self {
            frag: out.frag.expr(),
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
    V::Rep: Attributes,
    F::Rep: Fragment,
{
    pub fn new<W, VStage, FStage>(v_stage: VStage, f_stage: FStage) -> Self
    where
        W: Expose,
        W::Rep: Interpolants,
        VStage: FnOnce(Rep<R>, VArg<V>) -> VOut<W>,
        FStage: FnOnce(Rep<R>, FArg<W>) -> FOut<F>,
    {
        // FIXME: stage arg handling
        let v_out = v_stage(R::Rep::stage_arg(), VArg::stage_arg());
        let f_out = f_stage(R::Rep::stage_arg(), FArg::stage_arg());

        let v_stage = ErasedVStage::new(v_out);
        let f_stage = ErasedFStage::new(f_out);

        Self {
            v_stage,
            f_stage,
            _phantom: PhantomData,
        }
    }
}
