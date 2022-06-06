use std::collections::BTreeSet;

use super::{
    BinaryOp, BuiltInTy, CallExpr, Expr, Func, ScalarTy, StructTy, Ty, UserDefinedFunc, VarExpr,
};

pub fn collect_funcs(expr: &Expr, funcs: &mut BTreeSet<UserDefinedFunc>) {
    use Expr::*;

    match expr {
        Binary(expr) => {
            collect_funcs(&*expr.left, funcs);
            collect_funcs(&*expr.right, funcs);
        }
        Ternary(expr) => {
            collect_funcs(&*expr.cond, funcs);
            collect_funcs(&*expr.true_expr, funcs);
            collect_funcs(&*expr.false_expr, funcs);
        }
        Var(VarExpr {
            init: Some(init), ..
        }) => {
            collect_funcs(init, funcs);
        }
        Var(VarExpr { init: None, .. }) => (),
        Call(expr) => {
            if let Func::UserDefined(func) = &expr.func {
                funcs.insert(func.clone());
                collect_funcs(&*func.result, funcs);
            }
            for arg in &expr.args {
                collect_funcs(arg, funcs);
            }
        }
        Literal(_) => (),
        Field(expr) => {
            collect_funcs(&*expr.base, funcs);
        }
        BuiltInVar(_) => (),
    }
}

pub fn collect_vars(expr: &Expr, vars: &mut BTreeSet<VarExpr>) {
    use Expr::*;

    match expr {
        Binary(expr) => {
            collect_vars(&*expr.left, vars);
            collect_vars(&*expr.right, vars);
        }
        Ternary(expr) => {
            collect_vars(&*expr.cond, vars);
            collect_vars(&*expr.true_expr, vars);
            collect_vars(&*expr.false_expr, vars);
        }
        Var(
            var @ VarExpr {
                init: Some(init), ..
            },
        ) => {
            vars.insert(var.clone());
            collect_vars(init, vars);
        }
        Var(VarExpr { init: None, .. }) => (),
        Call(CallExpr { args, .. }) => {
            for arg in args {
                collect_vars(arg, vars);
            }
        }
        Literal(_) => (),
        Field(expr) => {
            collect_vars(&*expr.base, vars);
        }
        BuiltInVar(_) => (),
    }
}

fn show_scalar_ty(ty: ScalarTy) -> String {
    use ScalarTy::*;

    match ty {
        Bool => "bool".to_string(),
        I32 => "i32".to_string(),
        U32 => "u32".to_string(),
        F32 => "f32".to_string(),
    }
}

fn scalar_type_prefix(ty: ScalarTy) -> String {
    use ScalarTy::*;

    match ty {
        Bool => "b".to_string(),
        I32 => "i".to_string(),
        U32 => "u".to_string(),
        F32 => "".to_string(),
    }
}

fn show_built_in_ty(ty: &BuiltInTy) -> String {
    use BuiltInTy::*;

    match ty {
        Scalar(ty) => show_scalar_ty(*ty),
        Vec3(ty) => format!("{}vec3", scalar_type_prefix(*ty)),
        Vec4(ty) => format!("{}vec4", scalar_type_prefix(*ty)),
    }
}

fn show_struct_ty(ty: &StructTy) -> String {
    ty.ident.to_string()
}

fn show_ty(ty: &Ty) -> String {
    use Ty::*;

    match ty {
        BuiltIn(ty) => show_built_in_ty(ty),
        Struct(ty) => show_struct_ty(ty),
    }
}

pub fn show_user_defined_func(func: &UserDefinedFunc) -> String {
    let params: Vec<_> = func
        .params
        .iter()
        .map(|param| format!("{}: {}", param.ident.to_string(), show_ty(&param.ty)))
        .collect();

    let mut vars = BTreeSet::new();
    collect_vars(&func.result, &mut vars);
    // TODO: Sort vars by dependencies.

    let lets: Vec<_> = vars
        .iter()
        .map(|var| {
            format!(
                "    let {}: {} = {};",
                var.ident.to_string(),
                show_ty(&var.ty),
                show_expr(var.init.as_ref().unwrap())
            )
        })
        .collect();

    format!(
        "fn {}({}) -> {} {{\n{}\n    {}\n}}",
        func.ident.name,
        params.join(", "),
        show_ty(&func.result.ty()),
        lets.join("\n"),
        show_expr(&func.result),
    )
}

pub fn show_user_defined_funcs(func: &UserDefinedFunc) -> String {
    let mut funcs = BTreeSet::new();
    collect_funcs(&*func.result, &mut funcs);

    funcs.insert(func.clone());
    // TODO: Sort funcs by dependencies.

    funcs
        .iter()
        .map(show_user_defined_func)
        .collect::<Vec<_>>()
        .join("\n\n")
}

pub fn show_binary_op(op: BinaryOp) -> String {
    use BinaryOp::*;

    match op {
        Add => "+".into(),
        Sub => "-".into(),
        Mul => "*".into(),
        Div => "/".into(),
        Eq => "==".into(),
        And => "&&".into(),
        Or => "||".into(),
    }
}

pub fn show_expr(expr: &Expr) -> String {
    use Expr::*;

    match expr {
        Binary(expr) => format!(
            "({}) {} ({})",
            show_expr(&*expr.left),
            show_binary_op(expr.op),
            show_expr(&*expr.right)
        ),
        Ternary(expr) => format!(
            "if {} {{ {} }} else {{ {} }}",
            show_expr(&*expr.cond),
            show_expr(&*expr.true_expr),
            show_expr(&*expr.false_expr),
        ),
        Var(expr) => expr.ident.to_string(),
        Call(expr) => {
            let args: Vec<_> = expr.args.iter().map(show_expr).collect();
            format!("{}({})", expr.func.name(), args.join(", "),)
        }
        Literal(expr) => expr.literal.value.clone(),
        Field(expr) => {
            let base = show_expr(&*expr.base);
            format!("({}).{}", base, expr.member)
        }
        BuiltInVar(expr) => expr.name.to_string(),
    }
}
