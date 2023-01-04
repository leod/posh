use std::rc::Rc;

use super::{
    ty::{PrimitiveType, StructType},
    BaseType, Type,
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
    pub params: Vec<(&'static str, Type)>,
    pub result: Rc<Expr>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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
        ty: &'static StructType,
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
        name: &'static str,
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
        match self {
            Expr::Arg { ty, .. } => ty.clone(),
            Expr::ScalarLiteral { ty, .. } => Type::Base(BaseType::Scalar(*ty)),
            Expr::StructLiteral { ty, .. } => Type::Base(BaseType::Struct(ty)),
            Expr::Binary { ty, .. } => ty.clone(),
            Expr::CallFuncDef { def, .. } => def.result.ty(),
            Expr::CallBuiltIn { ty, .. } => ty.clone(),
            Expr::Field { ty, .. } => ty.clone(),
            Expr::Branch { ty, .. } => ty.clone(),
        }
    }
}
