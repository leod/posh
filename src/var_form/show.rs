use crate::{
    lang::show::{show_expr, show_struct_def, show_ty},
    var_form::var_name,
};

use super::{Scope, StructDefs, VarFormFuncDefs};

pub fn show_struct_defs(structs: &StructDefs) -> String {
    let mut result = String::new();

    for ty in structs.defs() {
        result += &format!("{}\n", show_struct_def(ty));
    }

    result
}

pub fn show_func_defs(funcs: &VarFormFuncDefs) -> String {
    let mut result = String::new();

    for (name, func) in funcs.defs() {
        let params: Vec<_> = func
            .params
            .iter()
            .map(|(name, ty)| format!("{}: {}", name, show_ty(ty)))
            .collect();

        result += &format!(
            "fn {}({}) -> {} {{\n{}\n{}\n}}",
            name,
            params.join(", "),
            show_ty(&func.result.1),
            show_scope(&*func.scope.borrow()),
            show_expr(&func.result.0),
        );
    }

    result
}

pub fn show_scope(scope: &Scope) -> String {
    let mut result = String::new();

    for (var_id, var_init) in scope.vars() {
        use super::VarInit::*;

        let var_init_string = match var_init {
            Branch(init) => format!(
                "if {} {{\n{}\n{}\n}} else {{\n{}\n{}\n}}",
                show_expr(&init.branch_expr.cond),
                show_scope(&*init.true_scope.borrow()),
                show_expr(&init.branch_expr.true_expr),
                show_scope(&*init.false_scope.borrow()),
                show_expr(&init.branch_expr.false_expr),
            ),
            Expr(init) => show_expr(init),
        };

        result += &format!(
            "let {}: {} = {};\n",
            var_name(*var_id),
            show_ty(&var_init.expr().ty()),
            var_init_string
        );
    }

    result
}
