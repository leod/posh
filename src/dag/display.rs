use std::{fmt, rc::Rc};

use super::{BinaryOp, Expr};

impl fmt::Display for BinaryOp {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        use BinaryOp::*;

        let s = match self {
            Add => "+",
            Sub => "-",
            Mul => "*",
            Div => "/",
            Eq => "==",
            And => "&&",
            Or => "||",
        };

        f.write_str(s)
    }
}

fn write_call(f: &mut impl fmt::Write, name: &str, args: &[Rc<Expr>]) -> Result<(), fmt::Error> {
    f.write_str(name)?;
    f.write_char('(')?;

    for (i, arg) in args.iter().enumerate() {
        write!(f, "{}", arg)?;

        if i + 1 < args.len() {
            f.write_str(", ")?;
        }
    }

    f.write_char(')')
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        use Expr::*;

        match self {
            Arg { name, .. } => f.write_str(&name),
            ScalarLiteral { value, .. } => f.write_str(&value),
            StructLiteral { args, ty } => write_call(f, ty.name, args),
            Binary {
                left, op, right, ..
            } => write!(f, "({left} {op} {right})"),
            CallFuncDef { def, args } => write_call(f, def.name, args),
            CallBuiltIn { name, args, .. } => write_call(f, name, args),
            Field { base, name, .. } => write!(f, "{base}.{name}"),
            Branch { cond, yes, no, .. } => write!(f, "({cond} ? {yes} : {no}"),
        }
    }
}
