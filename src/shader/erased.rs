use std::iter;

use crate::{
    lang::{defs::Defs, Expr, Ty},
    Expose, FuncArg,
};

use super::{
    fields::{Fields, InputFields, OutputFields},
    Attributes, FArg, FOut, Fragment, Interpolants, VArg, VOut,
};

pub struct ErasedVStage {
    pub attrs: Vec<(String, Ty)>,
    pub interps: Vec<(String, Expr)>,
    pub pos: Expr,
}

pub struct ErasedFStage {
    pub interps: Vec<(String, Ty)>,
    pub frag: Vec<(String, Expr)>,
    pub frag_depth: Option<Expr>,
}

pub struct ErasedShader {
    pub res: Vec<(String, Ty)>,
    pub v_stage: ErasedVStage,
    pub f_stage: ErasedFStage,
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
            attrs: <V::Rep as Fields>::fields("attrs"),
            interps: out.interps.stage_output("interps"),
            pos: (*out.pos.expr()).clone(),
        }
    }

    pub fn stage_arg<V>() -> VArg<V>
    where
        V: Expose,
        V::Rep: Attributes,
    {
        VArg::new(<V::Rep as InputFields>::stage_input("attrs"))
    }

    pub fn output_exprs(&self) -> impl IntoIterator<Item = &Expr> + '_ {
        self.interps
            .iter()
            .map(|(_, expr)| expr)
            .chain(iter::once(&self.pos))
    }

    pub fn defs(&self) -> Defs {
        let mut defs = Defs::new();

        for expr in self.output_exprs() {
            defs.extend(&Defs::from_expr(expr));
        }

        defs
    }
}

impl ErasedFStage {
    pub(crate) fn new<W, F>(out: FOut<F>) -> Self
    where
        W: Expose,
        W::Rep: Interpolants,
        F: Expose,
        F::Rep: Fragment,
    {
        Self {
            interps: <W::Rep as Fields>::fields("interps"),
            frag: out.frag.stage_output("frag"),
            frag_depth: out.frag_depth.map(|v| (*v.expr()).clone()),
        }
    }

    pub fn stage_arg<W>() -> FArg<W>
    where
        W: Expose,
        W::Rep: Interpolants,
    {
        FArg::new(<W::Rep as InputFields>::stage_input("interps"))
    }

    pub fn output_exprs(&self) -> impl IntoIterator<Item = &Expr> + '_ {
        self.frag
            .iter()
            .map(|(_, expr)| expr)
            .chain(self.frag_depth.as_ref())
    }

    pub fn defs(&self) -> Defs {
        let mut defs = Defs::new();

        for expr in self.output_exprs() {
            defs.extend(&Defs::from_expr(expr));
        }

        defs
    }
}
