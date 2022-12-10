use std::rc::Rc;

use super::{
    ty::{PrimitiveTy, StructTy},
    BaseTy, Ty,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Eq,
    And,
    Or,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FuncDef {
    pub name: &'static str,
    pub params: Vec<(&'static str, Ty)>,
    pub result: Rc<Expr>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Expr {
    Arg {
        ty: Ty,
        name: String,
    },
    ScalarLiteral {
        ty: PrimitiveTy,
        value: String,
    },
    StructLiteral {
        ty: &'static StructTy,
        args: Vec<Rc<Expr>>,
    },
    Binary {
        ty: Ty,
        left: Rc<Expr>,
        op: BinaryOp,
        right: Rc<Expr>,
    },
    CallFuncDef {
        def: FuncDef,
        args: Vec<Rc<Expr>>,
    },
    CallBuiltIn {
        ty: Ty,
        name: &'static str,
        args: Vec<Rc<Expr>>,
    },
    Field {
        ty: Ty,
        base: Rc<Expr>,
        name: &'static str,
    },
    Branch {
        ty: Ty,
        cond: Rc<Expr>,
        yes: Rc<Expr>,
        no: Rc<Expr>,
    },
}

impl Expr {
    pub fn ty(&self) -> Ty {
        match self {
            Expr::Arg { ty, .. } => ty.clone(),
            Expr::ScalarLiteral { ty, .. } => Ty::Base(BaseTy::Scalar(*ty)),
            Expr::StructLiteral { ty, .. } => Ty::Base(BaseTy::Struct(ty)),
            Expr::Binary { ty, .. } => ty.clone(),
            Expr::CallFuncDef { def, .. } => def.result.ty(),
            Expr::CallBuiltIn { ty, .. } => ty.clone(),
            Expr::Field { ty, .. } => ty.clone(),
            Expr::Branch { ty, .. } => ty.clone(),
        }
    }
}
