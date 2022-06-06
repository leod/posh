pub mod show;

use std::rc::Rc;

pub use uuid::Uuid;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ScalarTy {
    F32,
    I32,
    U32,
    Bool,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Ty {
    BuiltIn(BuiltInTy),
    Struct(StructTy),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BuiltInTy {
    Scalar(ScalarTy),
    Vec3(ScalarTy),
    Vec4(ScalarTy),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StructTy {
    pub ident: Ident,
    pub fields: Vec<(String, Ty)>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Ident {
    pub name: String,
    pub uuid: Uuid,
}

impl Ident {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            uuid: Uuid::new_v4(),
        }
    }
}

impl ToString for Ident {
    fn to_string(&self) -> String {
        format!("{}_{}", self.name, &self.uuid.simple().to_string()[0..8])
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Func {
    BuiltIn(BuiltInFunc),
    UserDefined(UserDefinedFunc),
    Struct(StructFunc),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BuiltInFunc {
    pub name: String,
    pub ty: Ty,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct UserDefinedFunc {
    pub ident: Ident,
    pub params: Vec<VarExpr>,
    pub result: Rc<Expr>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StructFunc {
    pub ty: StructTy,
}

impl Func {
    pub fn ty(&self) -> Ty {
        use Func::*;

        match self {
            BuiltIn(BuiltInFunc { ty, .. }) => ty.clone(),
            UserDefined(UserDefinedFunc { result, .. }) => result.ty(),
            Struct(StructFunc { ty }) => Ty::Struct(ty.clone()),
        }
    }

    pub fn name(&self) -> &str {
        use Func::*;

        match self {
            BuiltIn(BuiltInFunc { name, .. }) => name,
            UserDefined(UserDefinedFunc { ident, .. }) => &ident.name,
            Struct(StructFunc { ty, .. }) => &ty.ident.name,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Literal {
    pub value: String,
    pub ty: Ty,
}

impl From<bool> for Literal {
    fn from(x: bool) -> Self {
        Self {
            value: x.to_string(),
            ty: Ty::BuiltIn(BuiltInTy::Scalar(ScalarTy::Bool)),
        }
    }
}

impl From<i32> for Literal {
    fn from(x: i32) -> Self {
        Self {
            value: x.to_string(),
            ty: Ty::BuiltIn(BuiltInTy::Scalar(ScalarTy::I32)),
        }
    }
}

impl From<u32> for Literal {
    fn from(x: u32) -> Self {
        Self {
            value: x.to_string(),
            ty: Ty::BuiltIn(BuiltInTy::Scalar(ScalarTy::U32)),
        }
    }
}

impl From<f32> for Literal {
    fn from(x: f32) -> Self {
        Self {
            value: x.to_string(),
            ty: Ty::BuiltIn(BuiltInTy::Scalar(ScalarTy::F32)),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Eq,
    And,
    Or,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Expr {
    Binary(BinaryExpr),
    Ternary(TernaryExpr),
    Var(VarExpr),
    Call(CallExpr),
    Literal(LiteralExpr),
    Field(FieldExpr),
    BuiltInVar(BuiltInVarExpr),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BinaryExpr {
    pub left: Rc<Expr>,
    pub op: BinaryOp,
    pub right: Rc<Expr>,
    pub ty: Ty,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TernaryExpr {
    pub cond: Rc<Expr>,
    pub true_expr: Rc<Expr>,
    pub false_expr: Rc<Expr>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VarExpr {
    pub ident: Ident,
    pub ty: Ty,
    pub init: Option<Rc<Expr>>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CallExpr {
    pub func: Func,
    pub args: Vec<Expr>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LiteralExpr {
    pub literal: Literal,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FieldExpr {
    pub base: Rc<Expr>,
    pub member: String,
    pub ty: Ty,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BuiltInVarExpr {
    pub name: String,
    pub ty: BuiltInTy,
}

impl Expr {
    pub fn ty(&self) -> Ty {
        use Expr::*;

        match self {
            Binary(expr) => expr.ty.clone(),
            Ternary(expr) => {
                assert!(expr.true_expr.ty() == expr.false_expr.ty());
                expr.true_expr.ty()
            }
            Var(expr) => expr.ty.clone(),
            Call(expr) => expr.func.ty(),
            Literal(expr) => expr.literal.ty.clone(),
            Field(expr) => expr.ty.clone(),
            BuiltInVar(expr) => Ty::BuiltIn(expr.ty.clone()),
        }
    }
}
