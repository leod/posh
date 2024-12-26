mod scope_form;
mod simplified_expr;
mod struct_registry;
mod var_form;

use std::{
    fmt::{self, Display, Formatter, Write},
    rc::Rc,
};

use crate::sl::dag::ArrayType;

use super::{
    dag::{Expr, Type},
    program_def::{UniformBlockDef, UniformSamplerDef},
};

use self::{
    scope_form::{Scope, ScopeForm, VarInit},
    simplified_expr::VarId,
    struct_registry::StructRegistry,
    var_form::VarForm,
};

#[derive(Debug, Clone)]
struct WriteFuncContext<'a> {
    struct_registry: &'a StructRegistry,
    scope_form: &'a ScopeForm<'a>,
    depth: usize,
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
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        for _ in 0..self.0 {
            f.write_str("    ")?;
        }

        Ok(())
    }
}

pub fn write_shader_stage(
    f: &mut impl Write,
    block_defs: &[UniformBlockDef],
    sampler_defs: &[UniformSamplerDef],
    attributes: impl Iterator<Item = (String, String, Type)>,
    outputs: &[(&str, Rc<Expr>)],
) -> fmt::Result {
    let roots: Vec<_> = outputs.iter().map(|(_, root)| root.clone()).collect();
    let struct_registry = StructRegistry::new(&roots, block_defs.iter().map(|def| &def.ty));
    let var_form = VarForm::new(&struct_registry, &roots);
    let scope_form = ScopeForm::new(&var_form);

    let write_context = WriteFuncContext {
        struct_registry: &struct_registry,
        scope_form: &scope_form,
        depth: 1,
    };

    // TODO: Specify appropriate GLSL version depending on the target.
    writeln!(f, "#version 300 es")?;
    writeln!(f)?;

    // TODO: Make precision configurable.
    writeln!(f, "precision highp float;")?;
    writeln!(f, "precision highp int;")?;
    writeln!(f, "precision highp sampler2DShadow;")?;
    writeln!(f, "precision highp sampler2D;")?;
    writeln!(f)?;

    write_struct_defs(f, &struct_registry)?;

    writeln!(f)?;

    for sampler_def in sampler_defs {
        writeln!(f, "uniform {} {};", sampler_def.ty, sampler_def.name)?;
    }

    for block_def in block_defs {
        let ty_name = type_name(&struct_registry, &block_def.ty);

        writeln!(f, "layout(std140) uniform {} {{", block_def.block_name)?;
        writeln!(f, "    {} {};", ty_name, block_def.arg_name)?;
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
) -> Result<bool, fmt::Error> {
    use VarInit::*;

    let indent = ctx.indent();

    match var_init {
        Expr(expr) => {
            let ty_name = type_name(ctx.struct_registry, &expr.ty());

            writeln!(f, "{indent}{ty_name} {var_id} = {expr};")?;

            Ok(true)
        }
        Branch {
            cond,
            yes_id,
            no_id,
            ty,
        } => {
            let ty_name = type_name(ctx.struct_registry, ty);
            let yes_scope = ctx.scope_form.scope(*yes_id);
            let no_scope = ctx.scope_form.scope(*no_id);

            if yes_scope.vars.is_empty() && no_scope.vars.is_empty() && ty.built_in_type().is_some()
            {
                // If neither branch has variables, we can simplify the codegen
                // to avoid generating an if/else statement and generate a
                // ternary expression instead. Note that some GLSL versions do
                // not allow ternary expressions for structs.
                let yes_result = yes_scope.result.unwrap();
                let no_result = no_scope.result.unwrap();

                writeln!(
                    f,
                    "{indent}{ty_name} {var_id} = ({cond}) ? ({yes_result}) : ({no_result});"
                )?;

                return Ok(true);
            }

            writeln!(f, "{indent}{ty_name} {var_id};")?;
            writeln!(f, "{indent}if ({cond}) {{")?;

            {
                let ctx = ctx.nest();
                let indent = ctx.indent();

                let result = yes_scope.result.unwrap();

                if write_scope(f, ctx, yes_scope)? {
                    writeln!(f, "{indent}{var_id} = {result};")?;
                }
            }

            writeln!(f, "{indent}}} else {{")?;

            {
                let ctx = ctx.nest();
                let indent = ctx.indent();

                let result = no_scope.result.unwrap();

                if write_scope(f, ctx, no_scope)? {
                    writeln!(f, "{indent}{var_id} = {result};")?;
                }
            }

            writeln!(f, "{indent}}}")?;

            Ok(true)
        }
        Discard => {
            writeln!(f, "{indent}discard;")?;

            Ok(false)
        }
    }
}

fn write_scope(
    f: &mut impl Write,
    ctx: WriteFuncContext,
    scope: &Scope,
) -> Result<bool, fmt::Error> {
    for (var_id, var_init) in &scope.vars {
        if !write_var(f, ctx.clone(), *var_id, var_init)? {
            return Ok(false);
        }
    }

    Ok(true)
}

fn write_struct_defs(f: &mut impl Write, struct_reg: &StructRegistry) -> fmt::Result {
    for (name, ty) in struct_reg.defs() {
        writeln!(f, "struct {name} {{")?;

        for (field_name, field_ty) in ty.fields.iter() {
            let field_ty_name = type_name(struct_reg, field_ty);

            writeln!(f, "    {field_ty_name} {field_name};")?;
        }

        writeln!(f, "}};")?;
    }

    Ok(())
}

fn type_name(struct_reg: &StructRegistry, ty: &Type) -> String {
    use Type::*;

    match ty {
        BuiltIn(ty) => format!("{ty}"),
        Struct(ty) => struct_reg.name(ty),
        Array(ArrayType { ty, len }) => format!("{}[{}]", type_name(struct_reg, ty), len),
    }
}
