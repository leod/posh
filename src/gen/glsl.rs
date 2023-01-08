use std::{
    fmt::{Display, Formatter, Result, Write},
    rc::Rc,
};

use crate::{
    dag::{BaseType, Expr, Type},
    gen::{struct_registry, VarForm},
    sl::Struct,
};

use super::{Scope, ScopeForm, StructRegistry, VarId, VarInit};

#[derive(Debug, Clone)]
struct WriteFuncContext<'a> {
    pub struct_registry: &'a StructRegistry,
    pub scope_form: &'a ScopeForm<'a>,
    pub depth: usize,
}

impl<'a> WriteFuncContext<'a> {
    fn nest(&self) -> Self {
        Self {
            depth: self.depth + 1,
            ..self.clone()
        }
    }

    fn indent(&self) -> Indent {
        Indent(self.depth)
    }
}

struct Indent(usize);

impl Display for Indent {
    fn fmt(&self, f: &mut Formatter) -> Result {
        for _ in 0..self.0 {
            f.write_str("    ")?;
        }

        Ok(())
    }
}

pub fn write_shader_stage(f: &mut impl Write, outputs: &[(&str, Rc<Expr>)]) -> Result {
    let roots: Vec<_> = outputs.iter().map(|(_, root)| root.clone()).collect();
    let struct_registry = StructRegistry::new(&roots);
    let var_form = VarForm::new(&struct_registry, &roots);
    let scope_form = ScopeForm::new(&var_form);

    let write_context = WriteFuncContext {
        struct_registry: &struct_registry,
        scope_form: &scope_form,
        depth: 1,
    };

    write_struct_defs(f, &struct_registry)?;

    write!(f, "void main() {{\n")?;
    write_scope(f, write_context, scope_form.root_scope())?;
    for ((name, _), simplified_expr) in outputs.iter().zip(var_form.simplified_roots()) {
        write!(f, "    {name} = {simplified_expr};\n")?;
    }
    write!(f, "}}\n")?;

    Ok(())
}

fn write_var(
    f: &mut impl Write,
    ctx: WriteFuncContext,
    var_id: VarId,
    var_init: &VarInit,
) -> Result {
    use VarInit::*;

    let indent = ctx.indent();

    match var_init {
        Expr(expr) => {
            let ty_name = type_name(ctx.struct_registry, &expr.ty());

            write!(f, "{indent}{ty_name} {var_id} = {expr};\n")
        }
        Branch {
            cond,
            yes_id,
            no_id,
            ty,
        } => {
            let ty_name = type_name(ctx.struct_registry, ty);

            write!(f, "{indent}{ty_name} {var_id};\n")?;
            write!(f, "{indent}if {cond} {{\n")?;

            {
                let ctx = ctx.nest();
                let indent = ctx.indent();

                let yes_scope = ctx.scope_form.scope(*yes_id);
                let result = yes_scope.result.unwrap();

                write_scope(f, ctx, yes_scope)?;

                write!(f, "{indent}{var_id} = {result};\n")?;
            }

            write!(f, "{indent}}} else {{\n")?;

            {
                let ctx = ctx.nest();
                let indent = ctx.indent();

                let no_scope = ctx.scope_form.scope(*no_id);
                let result = no_scope.result.unwrap();

                write_scope(f, ctx, no_scope)?;

                write!(f, "{indent}{var_id} = {result};\n")?;
            }

            write!(f, "{indent}}}\n")?;

            Ok(())
        }
    }
}

fn write_scope(f: &mut impl Write, ctx: WriteFuncContext, scope: &Scope) -> Result {
    for (var_id, var_init) in &scope.vars {
        write_var(f, ctx.clone(), *var_id, var_init)?;
    }

    Ok(())
}

fn write_struct_defs(f: &mut impl Write, struct_registry: &StructRegistry) -> Result {
    for (name, ty) in struct_registry.defs() {
        write!(f, "struct {name} {{\n")?;

        for (field_name, field_ty) in ty.fields {
            let field_ty_name = type_name(struct_registry, field_ty);

            write!(f, "    {field_ty_name} {field_name};\n")?;
        }

        write!(f, "}}\n")?;
    }

    Ok(())
}

pub fn type_name(struct_registry: &StructRegistry, ty: &Type) -> String {
    use Type::*;

    match ty {
        Base(BaseType::Struct(ty)) if !ty.is_built_in => struct_registry.name(ty),
        Array(BaseType::Struct(ty), size) if !ty.is_built_in => {
            format!("{}[{}]", struct_registry.name(ty), size)
        }
        ty => format!("{ty}"),
    }
}
