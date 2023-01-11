use std::{
    fmt::{Display, Formatter, Result, Write},
    rc::Rc,
};

use crate::dag::{BaseType, Expr, Type};

use super::{
    scope_form::{Scope, ScopeForm, VarInit},
    simplified_expr::VarId,
    struct_registry::StructRegistry,
    var_form::VarForm,
};

pub struct UniformBlockDef {
    /// The name of the uniform block.
    pub block_name: String,

    /// The name of the single field within the uniform block.
    pub arg_name: String,

    /// The type of the uniform block.
    pub ty: Type,
}

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

pub fn write_shader_stage(
    f: &mut impl Write,
    uniform_block_defs: &[UniformBlockDef],
    attributes: impl Iterator<Item = (String, String, Type)>,
    outputs: &[(&str, Rc<Expr>)],
) -> Result {
    let roots: Vec<_> = outputs.iter().map(|(_, root)| root.clone()).collect();
    let struct_registry = StructRegistry::new(&roots, uniform_block_defs.iter().map(|def| &def.ty));
    let var_form = VarForm::new(&struct_registry, &roots);
    let scope_form = ScopeForm::new(&var_form);

    let write_context = WriteFuncContext {
        struct_registry: &struct_registry,
        scope_form: &scope_form,
        depth: 1,
    };

    write_struct_defs(f, &struct_registry)?;

    writeln!(f)?;

    for def in uniform_block_defs {
        let ty_name = type_name(&struct_registry, &def.ty);

        writeln!(f, "uniform {} {{", def.block_name)?;
        writeln!(f, "    {} {};", ty_name, def.arg_name)?;
        writeln!(f, "}};")?;
    }

    writeln!(f)?;

    for (kind, name, ty) in attributes {
        let ty_name = type_name(&struct_registry, &ty);

        writeln!(f, "{kind} {ty_name} {name};")?;
    }

    writeln!(f)?;

    writeln!(f, "void main() {{")?;
    write_scope(f, write_context, scope_form.root_scope())?;
    for ((name, _), simplified_expr) in outputs.iter().zip(var_form.simplified_roots()) {
        writeln!(f, "    {name} = {simplified_expr};")?;
    }
    writeln!(f, "}}")?;

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

            writeln!(f, "{indent}{ty_name} {var_id} = {expr};")
        }
        Branch {
            cond,
            yes_id,
            no_id,
            ty,
        } => {
            let ty_name = type_name(ctx.struct_registry, ty);

            writeln!(f, "{indent}{ty_name} {var_id};")?;
            writeln!(f, "{indent}if {cond} {{")?;

            {
                let ctx = ctx.nest();
                let indent = ctx.indent();

                let yes_scope = ctx.scope_form.scope(*yes_id);
                let result = yes_scope.result.unwrap();

                write_scope(f, ctx, yes_scope)?;

                writeln!(f, "{indent}{var_id} = {result};")?;
            }

            writeln!(f, "{indent}}} else {{")?;

            {
                let ctx = ctx.nest();
                let indent = ctx.indent();

                let no_scope = ctx.scope_form.scope(*no_id);
                let result = no_scope.result.unwrap();

                write_scope(f, ctx, no_scope)?;

                writeln!(f, "{indent}{var_id} = {result};")?;
            }

            writeln!(f, "{indent}}}")?;

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
        writeln!(f, "struct {name} {{")?;

        for (field_name, field_ty) in ty.fields.iter() {
            let field_ty_name = type_name(struct_registry, field_ty);

            writeln!(f, "    {field_ty_name} {field_name};")?;
        }

        writeln!(f, "}};")?;
    }

    Ok(())
}

fn type_name(struct_registry: &StructRegistry, ty: &Type) -> String {
    use Type::*;

    match ty {
        Base(BaseType::Struct(ty)) => struct_registry.name(ty),
        Array(BaseType::Struct(ty), size) => format!("{}[{}]", struct_registry.name(ty), size),
        ty => format!("{ty}"),
    }
}
