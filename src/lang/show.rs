use std::collections::BTreeSet;

use super::{
    BinOp, Expr, ExprBinary, ExprCall, ExprField, ExprTernary, ExprVar, Func, FuncUserDefined,
    ScalarType, Type, TypeBuiltIn, Var,
};

pub fn collect_funcs(expr: &Expr, funcs: &mut BTreeSet<FuncUserDefined>) {
    match expr {
        Expr::Binary(ExprBinary { left, right, .. }) => {
            collect_funcs(&*left, funcs);
            collect_funcs(&*right, funcs);
        }
        Expr::Ternary(ExprTernary {
            cond,
            true_expr,
            false_expr,
        }) => {
            collect_funcs(&*cond, funcs);
            collect_funcs(&*true_expr, funcs);
            collect_funcs(&*false_expr, funcs);
        }
        Expr::Var(ExprVar {
            var: Var {
                init: Some(init), ..
            },
        }) => {
            collect_funcs(init, funcs);
        }
        Expr::Var(ExprVar {
            var: Var { init: None, .. },
        }) => (),
        Expr::Call(ExprCall { func, args }) => {
            if let Func::UserDefined(func) = func {
                funcs.insert(func.clone());
                collect_funcs(&*func.result, funcs);
            }
            for arg in args {
                collect_funcs(arg, funcs);
            }
        }
        Expr::Lit(_) => (),
        Expr::Field(ExprField { base, .. }) => {
            collect_funcs(&*base, funcs);
        }
    }
}

pub fn collect_vars(expr: &Expr, vars: &mut BTreeSet<Var>) {
    match expr {
        Expr::Binary(ExprBinary { left, right, .. }) => {
            collect_vars(&*left, vars);
            collect_vars(&*right, vars);
        }
        Expr::Ternary(ExprTernary {
            cond,
            true_expr,
            false_expr,
        }) => {
            collect_vars(&*cond, vars);
            collect_vars(&*true_expr, vars);
            collect_vars(&*false_expr, vars);
        }
        Expr::Var(ExprVar {
            var: var @ Var {
                init: Some(init), ..
            },
        }) => {
            vars.insert(var.clone());
            collect_vars(init, vars);
        }
        Expr::Var(ExprVar {
            var: Var { init: None, .. },
        }) => (),
        Expr::Call(ExprCall { args, .. }) => {
            for arg in args {
                collect_vars(arg, vars);
            }
        }
        Expr::Lit(_) => (),
        Expr::Field(ExprField { base, .. }) => {
            collect_vars(&*base, vars);
        }
    }
}

fn show_scalar_type(ty: ScalarType) -> String {
    use ScalarType::*;

    match ty {
        Bool => "bool".to_string(),
        I32 => "i32".to_string(),
        U32 => "u32".to_string(),
        F32 => "f32".to_string(),
    }
}

fn scalar_type_prefix(ty: ScalarType) -> String {
    use ScalarType::*;

    match ty {
        Bool => "b".to_string(),
        I32 => "i".to_string(),
        U32 => "u".to_string(),
        F32 => "".to_string(),
    }
}

fn show_type_built_in(ty: &TypeBuiltIn) -> String {
    use TypeBuiltIn::*;

    match ty {
        Scalar(ty) => show_scalar_type(*ty),
        Vec3(ty) => format!("{}vec3", scalar_type_prefix(*ty)),
        Vec4(ty) => format!("{}vec4", scalar_type_prefix(*ty)),
    }
}

fn show_type(ty: &Type) -> String {
    use Type::*;

    match ty {
        BuiltIn(ty) => show_type_built_in(ty),
    }
}

pub fn show_user_defined_func(func: &FuncUserDefined) -> String {
    let params: Vec<_> = func
        .params
        .iter()
        .map(|param| format!("{}: {}", param.ident.to_string(), show_type(&param.ty)))
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
                show_type(&var.ty),
                show_expr(var.init.as_ref().unwrap())
            )
        })
        .collect();

    format!(
        "fn {}({}) -> {} {{\n{}\n    {}\n}}",
        func.ident.name,
        params.join(", "),
        show_type(&func.result.ty()),
        lets.join("\n"),
        show_expr(&func.result),
    )
}

pub fn show_user_defined_funcs(func: &FuncUserDefined) -> String {
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

pub fn show_bin_op(op: BinOp) -> String {
    match op {
        BinOp::Add => "+".into(),
        BinOp::Sub => "-".into(),
        BinOp::Mul => "*".into(),
        BinOp::Div => "/".into(),
        BinOp::Eq => "==".into(),
        BinOp::And => "&&".into(),
        BinOp::Or => "||".into(),
    }
}

pub fn show_expr(expr: &Expr) -> String {
    match expr {
        Expr::Binary(expr) => format!(
            "({}) {} ({})",
            show_expr(&*expr.left),
            show_bin_op(expr.op),
            show_expr(&*expr.right)
        ),
        Expr::Ternary(expr) => format!(
            "if {} {{ {} }} else {{ {} }}",
            show_expr(&*expr.cond),
            show_expr(&*expr.true_expr),
            show_expr(&*expr.false_expr),
        ),
        Expr::Var(expr) => expr.var.ident.to_string(),
        Expr::Call(expr) => {
            let args: Vec<_> = expr.args.iter().map(show_expr).collect();
            format!("{}({})", expr.func.name(), args.join(", "),)
        }
        Expr::Lit(expr) => expr.lit.value.clone(),
        Expr::Field(expr) => {
            let base = show_expr(&*expr.base);
            format!("({}).{}", base, expr.member)
        }
    }
}
