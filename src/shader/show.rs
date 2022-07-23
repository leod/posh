use std::collections::BTreeSet;

use crate::lang::{
    defs::collect_vars,
    show::{show_defs, show_expr, show_lets, show_ty},
};

use super::{ErasedFStage, ErasedShader, ErasedVStage};

fn show_v_stage(v_stage: &ErasedVStage) -> String {
    let mut vars = BTreeSet::new();
    for expr in v_stage.output_exprs() {
        collect_vars(expr, &mut vars);
    }

    let mut result = String::new();
    result += &v_stage
        .attributes
        .iter()
        .map(|(name, ty)| format!("attribute {}: {};", name, show_ty(ty)))
        .collect::<Vec<_>>()
        .join("\n");
    result += "\n";
    result += &show_lets(&vars);
    result += "\n";
    result += &format!("interps := {};\n", show_expr(&v_stage.interps));
    result += &format!("pos := {};\n", show_expr(&v_stage.pos));

    result
}

fn show_f_stage(f_stage: &ErasedFStage) -> String {
    let mut vars = BTreeSet::new();
    for expr in f_stage.output_exprs() {
        collect_vars(expr, &mut vars);
    }

    let mut result = String::new();
    result += &show_lets(&vars);
    result += "\n\n";
    result += &format!("frag := {};\n", show_expr(&f_stage.frag));

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
