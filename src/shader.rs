mod resource;

mod stage;
mod vertex;

use std::marker::PhantomData;

pub use resource::{Resource, Resources, UniformBlock, UniformBlockField};
pub use stage::{FragArg, FragOut, VertArg, VertOut};
pub use vertex::{
    Attributes, Fragment, FragmentField, Interpolants, InterpolantsField, Vertex, VertexField,
};

use crate::{expose::Expose, lang::Expr, FuncArg, Rep};

/// Description of a shader.
pub struct Shader<P, V, R> {
    vert_stage: ErasedVertStage,
    frag_stage: ErasedFragStage,
    _phantom: PhantomData<(P, V, R)>,
}

struct ErasedVertStage {
    pub interps: Expr,
    pub position: Expr,
}

struct ErasedFragStage {
    pub frag: Expr,
    pub frag_depth: Option<Expr>,
}

impl ErasedVertStage {
    fn new<W>(out: VertOut<W>) -> Self
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

impl ErasedFragStage {
    fn new<F>(out: FragOut<F>) -> Self
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
    pub fn new<W, VertStage, FragStage>(vert_stage: VertStage, frag_stage: FragStage) -> Self
    where
        W: Expose,
        W::Rep: Interpolants,
        VertStage: FnOnce(Rep<R>, VertArg<V>) -> VertOut<W>,
        FragStage: FnOnce(Rep<R>, FragArg<W>) -> FragOut<F>,
    {
        // FIXME: stage arg handling
        let vert_out = vert_stage(R::Rep::stage_arg(), VertArg::stage_arg());
        let frag_out = frag_stage(R::Rep::stage_arg(), FragArg::stage_arg());

        let vert_stage = ErasedVertStage::new(vert_out);
        let frag_stage = ErasedFragStage::new(frag_out);

        Self {
            vert_stage,
            frag_stage,
            _phantom: PhantomData,
        }
    }
}
