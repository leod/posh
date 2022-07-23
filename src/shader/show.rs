use std::collections::BTreeSet;

use crate::lang::{
    defs::collect_vars,
    show::{show_defs, show_expr, show_lets, show_ty},
    Ty,
};

use super::{ErasedFStage, ErasedShader, ErasedVStage};

fn interface(kind: &str, fields: impl IntoIterator<Item = (String, Ty)>) -> String {
    fields
        .into_iter()
        .map(|(name, ty)| format!("{} {}: {};", kind, name, show_ty(&ty)))
        .collect::<Vec<_>>()
        .join("\n")
}

fn show_v_stage(v_stage: &ErasedVStage) -> String {
    let mut vars = BTreeSet::new();
    for expr in v_stage.output_exprs() {
        collect_vars(expr, &mut vars);
    }

    let mut result = String::new();
    result += &interface("attribute", v_stage.attrs.iter().cloned());
    result += "\n\n";
    result += &interface(
        "out",
        v_stage
            .interps
            .iter()
            .map(|(name, expr)| (name.clone(), expr.ty())),
    );
    result += "\n\n";
    result += &show_lets(&vars);
    result += "\n";
    for (name, expr) in &v_stage.interps {
        result += &format!("{} := {};\n", name, show_expr(expr));
    }
    result += &format!("gl_Position := {};\n", show_expr(&v_stage.pos));

    result
}

fn show_f_stage(f_stage: &ErasedFStage) -> String {
    let mut vars = BTreeSet::new();
    for expr in f_stage.output_exprs() {
        collect_vars(expr, &mut vars);
    }

    let mut result = String::new();
    result += &interface("in", f_stage.interps.iter().cloned());
    result += "\n\n";
    result += &interface(
        "out",
        f_stage
            .frag
            .iter()
            .map(|(name, expr)| (name.clone(), expr.ty())),
    );
    result += "\n\n";
    result += &show_lets(&vars);
    result += "\n";
    for (name, expr) in &f_stage.frag {
        result += &format!("{} := {};\n", name, show_expr(expr));
    }

    if let Some(frag_depth) = f_stage.frag_depth.as_ref() {
        result += &format!("frag_depth := {};\n", show_expr(frag_depth));
    }

    result
}

pub fn show_shader(shader: &ErasedShader) -> String {
    format!(
        "{}\n{}\n============================================================================\n\n{}",
        show_defs(&shader.defs()),
        show_v_stage(&shader.v_stage),
        show_f_stage(&shader.f_stage)
    )
}
