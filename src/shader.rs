mod resource;
pub mod show;
mod stage;
mod vertex;

use std::marker::PhantomData;

pub use resource::{Resource, Resources, UniformBlock, UniformBlockField};
pub use stage::{FArg, FOut, VArg, VOut};
pub use vertex::{
    Attributes, Fragment, FragmentField, Interpolants, InterpolantsField, Vertex, VertexField,
};

use crate::{
    expose::Expose,
    lang::{defs::Defs, Expr},
    FuncArg, Rep,
};

/// Description of a shader.
pub struct Shader<R, V, F> {
    erased: ErasedShader,
    _phantom: PhantomData<(R, V, F)>,
}

pub struct ErasedShader {
    v_stage: ErasedVStage,
    f_stage: ErasedFStage,
}

pub struct ErasedVStage {
    pub interps: Expr,
    pub pos: Expr,
}

pub struct ErasedFStage {
    pub frag: Expr,
    pub frag_depth: Option<Expr>,
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

        let erased = ErasedShader { v_stage, f_stage };

        Self {
            erased,
            _phantom: PhantomData,
        }
    }

    pub fn erased(&self) -> &ErasedShader {
        &self.erased
    }
}

impl ErasedShader {
    pub fn defs(&self) -> Defs {
        let mut defs = Defs::new();

        let exprs = self
            .v_stage
            .output_exprs()
            .into_iter()
            .chain(self.f_stage.output_exprs());
        for expr in exprs {
            defs.extend(&Defs::from_expr(expr));
        }

        defs
    }
}

impl ErasedVStage {
    fn new<W>(out: VOut<W>) -> Self
    where
        W: Expose,
        W::Rep: Interpolants,
    {
        Self {
            interps: out.interps.expr(),
            pos: out.pos.expr(),
        }
    }

    pub fn output_exprs(&self) -> impl IntoIterator<Item = &Expr> + '_ {
        vec![&self.interps, &self.pos]
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

    pub fn output_exprs(&self) -> impl IntoIterator<Item = &Expr> + '_ {
        let mut exprs = vec![&self.frag];
        if let Some(pos) = self.frag_depth.as_ref() {
            exprs.push(pos);
        }

        exprs
    }
}
