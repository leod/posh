use std::{collections::BTreeSet, iter};

use crate::lang::{
    defs::collect_vars,
    show::{show_defs, show_expr, show_lets, show_ty},
    Expr, Ty, VarExpr,
};

use super::{ErasedFStage, ErasedShader, ErasedVStage};

fn show_interface(kind: &str, fields: impl IntoIterator<Item = (String, Ty)>) -> String {
    fields
        .into_iter()
        .map(|(name, ty)| format!("{} {}: {};", kind, name, show_ty(&ty)))
        .collect::<Vec<_>>()
        .join("\n")
}

fn show_main<'a>(
    vars: &BTreeSet<VarExpr>,
    outputs: impl Iterator<Item = (String, &'a Expr)>,
) -> String {
    let mut result = String::new();

    result += "fn main() {\n";

    result += &show_lets(vars);

    result += "\n\n";
    for (name, expr) in outputs {
        result += &format!("    {} := {};\n", name, show_expr(expr));
    }

    result += "}";

    result
}

fn show_v_stage(res: &str, stage: &ErasedVStage) -> String {
    let mut vars = BTreeSet::new();
    for expr in stage.output_exprs() {
        collect_vars(expr, &mut vars);
    }

    let outputs = stage
        .interps
        .iter()
        .map(|(name, expr)| (name.clone(), expr))
        .chain(iter::once(("gl_Position".into(), &stage.pos)));

    let mut result = String::new();

    result += &show_defs(&stage.defs());

    result += "\n\n";
    result += res;

    result += "\n\n";
    result += &show_interface("in", stage.attrs.iter().cloned());

    result += "\n\n";
    result += &show_interface(
        "out",
        stage
            .interps
            .iter()
            .map(|(name, expr)| (name.clone(), expr.ty())),
    );

    result += "\n\n";
    result += &show_main(&vars, outputs);

    result
}

fn show_f_stage(res: &str, stage: &ErasedFStage) -> String {
    let mut vars = BTreeSet::new();
    for expr in stage.output_exprs() {
        collect_vars(expr, &mut vars);
    }

    let outputs = stage
        .frag
        .iter()
        .map(|(name, expr)| (name.clone(), expr))
        .chain(
            stage
                .frag_depth
                .as_ref()
                .map(|frag_depth| ("gl_FragDepth".into(), frag_depth)),
        );

    let mut result = String::new();

    result += &show_defs(&stage.defs());

    result += "\n\n";
    result += res;

    result += "\n\n";
    result += &show_interface("in", stage.interps.iter().cloned());

    result += "\n\n";
    result += &show_interface(
        "out",
        stage
            .frag
            .iter()
            .map(|(name, expr)| (name.clone(), expr.ty())),
    );

    result += "\n\n";
    result += &show_main(&vars, outputs);

    result
}

pub fn show_shader(shader: &ErasedShader) -> String {
    let res = show_interface("uniform", shader.res.iter().cloned());

    format!(
        "{}\n============================================================================\n\n{}",
        show_v_stage(&res, &shader.v_stage),
        show_f_stage(&res, &shader.f_stage)
    )
}
