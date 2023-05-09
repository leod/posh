use std::rc::Rc;

use super::{ArrayType, BuiltInType, StructType, Type};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    And,
    Or,
    Eq,
    Lt,
    Le,
    Ne,
    Ge,
    Gt,
    Shl,
    Shr,
    BitAnd,
    BitOr,
    BitXor,
    Rem,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum UnaryOp {
    Neg,
    Not,
    BitNot,
}

#[derive(Debug, Clone)]
pub struct FuncDef {
    pub name: &'static str,
    pub params: Vec<(&'static str, Type)>,
    pub result: Rc<Expr>,
}

#[derive(Debug, Clone)]
pub enum Expr {
    Arg {
        name: String,
        ty: Type,
    },
    ScalarLiteral {
        value: String,
        ty: BuiltInType,
    },
    StructLiteral {
        args: Vec<Rc<Expr>>,
        ty: Rc<StructType>,
    },
    ArrayLiteral {
        args: Vec<Rc<Expr>>,
        ty: ArrayType,
    },
    Unary {
        op: UnaryOp,
        arg: Rc<Expr>,
        ty: Type,
    },
    Binary {
        left: Rc<Expr>,
        op: BinaryOp,
        right: Rc<Expr>,
        ty: Type,
    },
    CallFuncDef {
        def: FuncDef,
        args: Vec<Rc<Expr>>,
    },
    CallBuiltIn {
        name: String,
        args: Vec<Rc<Expr>>,
        ty: Type,
    },
    Field {
        base: Rc<Expr>,
        name: &'static str,
        ty: Type,
    },
    Subscript {
        base: Rc<Expr>,
        index: Rc<Expr>,
        ty: Type,
    },
    Branch {
        cond: Rc<Expr>,
        yes: Rc<Expr>,
        no: Rc<Expr>,
        ty: Type,
    },
    Discard {
        ty: Type,
    },
}

impl Expr {
    pub fn ty(&self) -> Type {
        use Expr::*;

        match self {
            Arg { ty, .. } => ty.clone(),
            ScalarLiteral { ty, .. } => Type::BuiltIn(*ty),
            StructLiteral { ty, .. } => Type::Struct(ty.clone()),
            ArrayLiteral { ty, .. } => Type::Array(ty.clone()),
            Unary { ty, .. } => ty.clone(),
            Binary { ty, .. } => ty.clone(),
            CallFuncDef { def, .. } => def.result.ty(),
            CallBuiltIn { ty, .. } => ty.clone(),
            Field { ty, .. } => ty.clone(),
            Subscript { ty, .. } => ty.clone(),
            Branch { ty, .. } => ty.clone(),
            Discard { ty, .. } => ty.clone(),
        }
    }

    pub fn successors(&self, mut f: impl FnMut(&Rc<Expr>)) {
        use Expr::*;

        match self {
            Arg { .. } | ScalarLiteral { .. } => (),
            StructLiteral { args, .. } => {
                for arg in args {
                    f(arg);
                }
            }
            ArrayLiteral { args, .. } => {
                for arg in args {
                    f(arg);
                }
            }
            Unary { arg: expr, .. } => {
                f(expr);
            }
            Binary { left, right, .. } => {
                f(left);
                f(right);
            }
            CallFuncDef { def, args, .. } => {
                f(&def.result);

                for arg in args {
                    f(arg);
                }
            }
            CallBuiltIn { args, .. } => {
                for arg in args {
                    f(arg);
                }
            }
            Field { base, .. } => {
                f(base);
            }
            Subscript { base, index, .. } => {
                f(base);
                f(index);
            }
            Branch { cond, yes, no, .. } => {
                f(cond);
                f(yes);
                f(no);
            }
            Discard { .. } => {}
        }
    }
}
