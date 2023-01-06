use std::fmt;

use super::{SimplifiedExpr, VarId};

fn write_call(
    f: &mut impl fmt::Write,
    name: &str,
    args: &[SimplifiedExpr],
) -> Result<(), fmt::Error> {
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

fn write_var_name(f: &mut impl fmt::Write, id: VarId) -> Result<(), fmt::Error> {
    write!(f, "var_{}", id.index())
}

impl fmt::Display for SimplifiedExpr {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        use SimplifiedExpr::*;

        match self {
            Arg { name, .. } => f.write_str(name),
            ScalarLiteral { value, .. } => f.write_str(value),
            Binary {
                left, op, right, ..
            } => write!(f, "({left} {op} {right})"),
            CallFunc { name, args, .. } => write_call(f, name, args),
            Field { base, name, .. } => write!(f, "{base}.{name}"),
            Branch { cond, yes, no, .. } => write!(f, "({cond} ? {yes} : {no})"),
            Var { id, .. } => write_var_name(f, *id),
        }
    }
}
