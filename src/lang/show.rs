use super::{defs::Defs, BinaryOp, BuiltInTy, DefFunc, Expr, ScalarTy, StructTy, Ty};

pub fn show_expr(expr: &Expr) -> String {
    use Expr::*;

    match expr {
        Binary(expr) => format!(
            "({}) {} ({})",
            show_expr(&*expr.left),
            show_binary_op(expr.op),
            show_expr(&*expr.right)
        ),
        Branch(expr) => format!(
            "if {} {{ {} }} else {{ {} }}",
            show_expr(&*expr.cond),
            show_expr(&*expr.true_expr),
            show_expr(&*expr.false_expr),
        ),
        Var(expr) => expr.ident.to_string(),
        Call(expr) => {
            let args: Vec<_> = expr.args.iter().map(|arg| show_expr(&*arg)).collect();
            format!("{}({})", expr.func.name(), args.join(", "),)
        }
        Literal(expr) => expr.literal.value.clone(),
        Field(expr) => {
            let base = show_expr(&*expr.base);
            format!("({}).{}", base, expr.member)
        }
    }
}

pub fn show_func_def(def: &DefFunc) -> String {
    let params: Vec<_> = def
        .params
        .iter()
        .map(|param| format!("{}: {}", param.ident.to_string(), show_ty(&param.ty)))
        .collect();

    format!(
        "fn {}({}) -> {} {{\n{}\n}}",
        def.ident.name,
        params.join(", "),
        show_ty(&def.result.ty()),
        show_expr(&def.result),
    )
}

pub fn show_defs(defs: &Defs) -> String {
    // FIXME: Sort funcs and structs by dependencies.
    let struct_defs = defs
        .structs
        .iter()
        .map(show_struct_def)
        .collect::<Vec<_>>()
        .join("\n\n");

    let func_defs = defs
        .funcs
        .iter()
        .map(show_func_def)
        .collect::<Vec<_>>()
        .join("\n\n");

    format!("{}\n\n{}", struct_defs, func_defs)
}

pub fn show_scalar_ty(ty: ScalarTy) -> String {
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

pub fn show_built_in_ty(ty: &BuiltInTy) -> String {
    use BuiltInTy::*;

    match ty {
        Scalar(ty) => show_scalar_ty(*ty),
        Vec2(ty) => format!("{}vec2", scalar_type_prefix(*ty)),
        Vec3(ty) => format!("{}vec3", scalar_type_prefix(*ty)),
        Vec4(ty) => format!("{}vec4", scalar_type_prefix(*ty)),
        Sampler2 => "sampler2D".to_string(),
    }
}

pub fn show_struct_ty(ty: &StructTy) -> String {
    ty.ident.to_string()
}

pub fn show_ty(ty: &Ty) -> String {
    use Ty::*;

    match ty {
        BuiltIn(ty) => show_built_in_ty(ty),
        Struct(ty) => show_struct_ty(ty),
    }
}

pub fn show_struct_def(ty: &StructTy) -> String {
    let fields: Vec<_> = ty
        .fields
        .iter()
        .map(|(name, ty)| format!("    {}: {},", name, show_ty(ty)))
        .collect();

    format!(
        "struct {}\n{{\n{}\n}}",
        ty.ident.to_string(),
        fields.join("\n")
    )
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
