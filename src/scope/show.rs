use crate::lang::show::{show_expr, show_struct_def, show_ty};

use super::{Defs, Scope};

pub fn show_defs(defs: &Defs) -> String {
    let mut result = String::new();

    for ty in defs.struct_defs() {
        result += &format!("{}\n", show_struct_def(ty));
    }

    for func in defs.func_defs() {
        let params: Vec<_> = func
            .params
            .iter()
            .map(|param| format!("{}: {}", param.ident.to_string(), show_ty(&param.ty)))
            .collect();

        result += &format!(
            "fn {}({}) -> {} {{\n{}\n{}\n}}",
            func.name,
            params.join(", "),
            show_ty(&func.result_ty),
            show_scope(&func.scope),
            func.result,
        );
    }

    result
}

pub fn show_scope(scope: &Scope) -> String {
    let mut result = String::new();

    for (var_name, var_init) in scope.var_defs() {
        use super::Init::*;

        let var_init_string = match var_init {
            Branch(init) => format!(
                "if {} {{\n{}\n{}\n}} else {{\n{}\n{}\n}}",
                show_expr(&init.branch_expr.cond),
                show_scope(&init.true_scope),
                show_expr(&init.branch_expr.true_expr),
                show_scope(&init.false_scope),
                show_expr(&init.branch_expr.false_expr),
            ),
            Expr(init) => show_expr(&init),
        };

        result += &format!(
            "let {}: {} = {};\n",
            var_name,
            show_ty(&var_init.expr().ty()),
            var_init_string
        );
    }

    result
}
