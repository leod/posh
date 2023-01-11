use std::rc::Rc;

use super::{
    ty::{PrimitiveType, StructType},
    BaseType, Type,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Eq,
    And,
    Or,
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
        ty: PrimitiveType,
    },
    StructLiteral {
        args: Vec<Rc<Expr>>,
        ty: Rc<StructType>,
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
    Branch {
        cond: Rc<Expr>,
        yes: Rc<Expr>,
        no: Rc<Expr>,
        ty: Type,
    },
}

impl Expr {
    pub fn ty(&self) -> Type {
        use Expr::*;

        match self {
            Arg { ty, .. } => ty.clone(),
            ScalarLiteral { ty, .. } => Type::Base(BaseType::Scalar(*ty)),
            StructLiteral { ty, .. } => Type::Base(BaseType::Struct(ty.clone())),
            Binary { ty, .. } => ty.clone(),
            CallFuncDef { def, .. } => def.result.ty(),
            CallBuiltIn { ty, .. } => ty.clone(),
            Field { ty, .. } => ty.clone(),
            Branch { ty, .. } => ty.clone(),
        }
    }
}
