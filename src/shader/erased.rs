use crate::{
    lang::{defs::Defs, Expr, Ty},
    Expose, FuncArg,
};

use super::{fields::Fields, Attributes, FOut, Fragment, Interpolants, VArg, VOut};

pub struct ErasedVStage {
    pub attributes: Vec<(String, Ty)>,
    pub interps: Expr,
    pub pos: Expr,
}

pub struct ErasedFStage {
    pub frag: Expr,
    pub frag_depth: Option<Expr>,
}

pub struct ErasedShader {
    pub v_stage: ErasedVStage,
    pub f_stage: ErasedFStage,
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
    pub(crate) fn new<V, W>(out: VOut<W>) -> Self
    where
        V: Expose,
        V::Rep: Attributes,
        W: Expose,
        W::Rep: Interpolants,
    {
        Self {
            attributes: <V::Rep as Fields>::fields("attrs"),
            interps: out.interps.expr(),
            pos: out.pos.expr(),
        }
    }

    pub fn stage_arg<V>() -> VArg<V>
    where
        V: Expose,
        V::Rep: Attributes,
    {
        VArg::new(<V::Rep as Fields>::stage_input("attrs"))
    }

    pub fn output_exprs(&self) -> impl IntoIterator<Item = &Expr> + '_ {
        vec![&self.interps, &self.pos]
    }
}

impl ErasedFStage {
    pub(crate) fn new<F>(out: FOut<F>) -> Self
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
