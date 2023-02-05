use std::fmt::{Display, Formatter, Result, Write};

use crate::dag::{BaseType, BinaryOp, PrimitiveType, Type};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VarId(pub usize);

#[derive(Debug, Clone)]
pub enum SimplifiedExpr {
    Arg {
        name: String,
        ty: Type,
    },
    ScalarLiteral {
        value: String,
        ty: PrimitiveType,
    },
    Binary {
        left: Box<SimplifiedExpr>,
        op: BinaryOp,
        right: Box<SimplifiedExpr>,
        ty: Type,
    },
    CallFunc {
        name: String,
        args: Vec<SimplifiedExpr>,
        ty: Type,
    },
    Field {
        base: Box<SimplifiedExpr>,
        name: &'static str,
        ty: Type,
    },
    Subscript {
        base: Box<SimplifiedExpr>,
        index: Box<SimplifiedExpr>,
        ty: Type,
    },
    Var {
        id: VarId,
        ty: Type,
    },
    Branch {
        cond: Box<SimplifiedExpr>,
        yes: Box<SimplifiedExpr>,
        no: Box<SimplifiedExpr>,
        ty: Type,
    },
}

impl SimplifiedExpr {
    pub fn ty(&self) -> Type {
        use SimplifiedExpr::*;

        match self {
            Arg { ty, .. } => ty.clone(),
            ScalarLiteral { ty, .. } => Type::Base(BaseType::Scalar(*ty)),
            Binary { ty, .. } => ty.clone(),
            CallFunc { ty, .. } => ty.clone(),
            Field { ty, .. } => ty.clone(),
            Branch { ty, .. } => ty.clone(),
            Subscript { ty, .. } => ty.clone(),
            Var { ty, .. } => ty.clone(),
        }
    }
}

fn write_call(f: &mut impl Write, name: &str, args: &[SimplifiedExpr]) -> Result {
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

impl Display for VarId {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "var_{}", self.0)
    }
}

impl Display for SimplifiedExpr {
    fn fmt(&self, f: &mut Formatter) -> Result {
        use SimplifiedExpr::*;

        match self {
            Arg { name, .. } => f.write_str(name),
            ScalarLiteral { value, .. } => f.write_str(value),
            Binary {
                left, op, right, ..
            } => write!(f, "({left} {op} {right})"),
            CallFunc { name, args, .. } => write_call(f, name, args),
            Field { base, name, .. } => match base.ty() {
                Type::Base(ty) if ty.is_mat() => {
                    let index = match *name {
                        "x" => 0,
                        "y" => 1,
                        "z" => 2,
                        "w" => 3,
                        _ => unreachable!(),
                    };

                    write!(f, "{base}[{index}]")
                }
                _ => write!(f, "{base}.{name}"),
            },
            Branch { cond, yes, no, .. } => write!(f, "({cond} ? {yes} : {no})"),
            Subscript { base, index, .. } => write!(f, "{base}[{index}"),
            Var { id, .. } => write!(f, "{id}"),
        }
    }
}
