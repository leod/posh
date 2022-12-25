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
        ty: Type,
        name: String,
    },
    ScalarLiteral {
        ty: PrimitiveType,
        value: String,
    },
    StructLiteral {
        ty: &'static StructType,
        args: Vec<Rc<Expr>>,
    },
    Binary {
        ty: Type,
        left: Rc<Expr>,
        op: BinaryOp,
        right: Rc<Expr>,
    },
    CallFuncDef {
        def: FuncDef,
        args: Vec<Rc<Expr>>,
    },
    CallBuiltIn {
        ty: Type,
        name: &'static str,
        args: Vec<Rc<Expr>>,
    },
    Field {
        ty: Type,
        base: Rc<Expr>,
        name: &'static str,
    },
    Branch {
        ty: Type,
        cond: Rc<Expr>,
        yes: Rc<Expr>,
        no: Rc<Expr>,
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
