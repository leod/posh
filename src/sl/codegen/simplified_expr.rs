use std::{
    fmt::{Display, Formatter, Result, Write},
    rc::Rc,
};

use crate::sl::dag::{BinaryOp, BuiltInType, Expr, Type, UnaryOp};

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ExprKey(*const Expr);

impl<'a> From<&'a Rc<Expr>> for ExprKey {
    fn from(value: &'a Rc<Expr>) -> Self {
        ExprKey(&**value as *const _)
    }
}

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
        ty: BuiltInType,
    },
    Unary {
        op: UnaryOp,
        arg: Box<SimplifiedExpr>,
        ty: Type,
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
    Discard {
        ty: Type,
    },
}

impl SimplifiedExpr {
    pub fn ty(&self) -> Type {
        use SimplifiedExpr::*;

        match self {
            Arg { ty, .. } => ty.clone(),
            ScalarLiteral { ty, .. } => Type::BuiltIn(*ty),
            Unary { ty, .. } => ty.clone(),
            Binary { ty, .. } => ty.clone(),
            CallFunc { ty, .. } => ty.clone(),
            Field { ty, .. } => ty.clone(),
            Branch { ty, .. } => ty.clone(),
            Subscript { ty, .. } => ty.clone(),
            Var { ty, .. } => ty.clone(),
            Discard { ty, .. } => ty.clone(),
        }
    }
}

fn write_call(f: &mut impl Write, name: &str, args: &[SimplifiedExpr]) -> Result {
    f.write_str(name)?;
    f.write_char('(')?;

    for (i, arg) in args.iter().enumerate() {
        write!(f, "{arg}")?;

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
            Unary { op, arg, .. } => write!(f, "{op} {arg}"),
            Binary {
                left, op, right, ..
            } => write!(f, "({left} {op} {right})"),
            CallFunc { name, args, .. } => write_call(f, name, args),
            Field { base, name, .. } => match base.ty() {
                Type::BuiltIn(ty) if ty.is_mat() => {
                    let index = match *name {
                        "x_axis" => 0,
                        "y_axis" => 1,
                        "z_axis" => 2,
                        "w_axis" => 3,
                        _ => unreachable!(),
                    };

                    write!(f, "{base}[{index}]")
                }
                _ => write!(f, "{base}.{name}"),
            },
            Branch { cond, yes, no, .. } => write!(f, "({cond} ? {yes} : {no})"),
            Subscript { base, index, .. } => write!(f, "{base}[{index}]"),
            Var { id, .. } => write!(f, "{id}"),
            Discard { .. } => write!(f, "discard"),
        }
    }
}
