use std::fmt::{Display, Formatter, Result, Write};

use super::{Scope, ScopeForm, VarId, VarInit};

#[derive(Debug, Clone)]
pub struct WriteContext<'a> {
    pub scope_form: &'a ScopeForm<'a>,
    pub depth: usize,
}

impl<'a> WriteContext<'a> {
    pub fn new(scope_form: &'a ScopeForm<'a>) -> Self {
        Self {
            scope_form,
            depth: 0,
        }
    }

    fn nest(&self) -> Self {
        Self {
            scope_form: self.scope_form,
            depth: self.depth + 1,
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

pub fn write_var(
    f: &mut impl Write,
    ctx: WriteContext,
    var_id: VarId,
    var_init: &VarInit,
) -> Result {
    use VarInit::*;

    let indent = ctx.indent();

    match var_init {
        Expr(expr) => {
            let ty = expr.ty();

            write!(f, "{indent}{ty} {var_id} = {expr};\n")
        }
        Branch {
            cond,
            yes_id,
            no_id,
            ty,
        } => {
            write!(f, "{indent}{ty} {var_id};\n")?;
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

pub fn write_scope(f: &mut impl Write, ctx: WriteContext, scope: &Scope) -> Result {
    for (var_id, var_init) in &scope.vars {
        write_var(f, ctx.clone(), *var_id, var_init)?;
    }

    Ok(())
}
