pub mod defs;
pub(crate) mod expr_reg;
pub mod show;

use std::rc::Rc;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Ty {
    BuiltIn(BuiltInTy),
    Struct(StructTy),
    Named(NamedTy),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BuiltInTy {
    Scalar(ScalarTy),
    Vec2(ScalarTy),
    Vec3(ScalarTy),
    Vec4(ScalarTy),
    Sampler2,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ScalarTy {
    F32,
    I32,
    U32,
    Bool,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StructTy {
    pub ident: Ident,
    pub fields: Vec<(String, Ty)>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NamedTy {
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Ident {
    pub name: String,
}

impl Ident {
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into() }
    }
}

impl ToString for Ident {
    fn to_string(&self) -> String {
        self.name.clone()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Func {
    BuiltIn(BuiltInFunc),
    Def(FuncDef),
    Struct(StructFunc),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BuiltInFunc {
    pub name: String,
    pub ty: Ty,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FuncParam {
    pub ident: Ident,
    pub ty: Ty,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FuncDef {
    pub ident: Ident,
    pub params: Vec<FuncParam>,
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
            Def(FuncDef { result, .. }) => result.ty(),
            Struct(StructFunc { ty }) => Ty::Struct(ty.clone()),
        }
    }

    pub fn name(&self) -> &str {
        use Func::*;

        match self {
            BuiltIn(BuiltInFunc { name, .. }) => name,
            Def(FuncDef { ident, .. }) => &ident.name,
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
    Branch(BranchExpr),
    Var(VarExpr),
    Call(CallExpr),
    Literal(LiteralExpr),
    Field(FieldExpr),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BinaryExpr {
    pub left: Rc<Expr>,
    pub op: BinaryOp,
    pub right: Rc<Expr>,
    pub ty: Ty,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BranchExpr {
    pub cond: Rc<Expr>,
    pub true_expr: Rc<Expr>,
    pub false_expr: Rc<Expr>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VarExpr {
    pub ident: Ident,
    pub ty: Ty,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CallExpr {
    pub func: Func,
    pub args: Vec<Rc<Expr>>,
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

impl Expr {
    pub fn ty(&self) -> Ty {
        use Expr::*;

        match self {
            Binary(expr) => expr.ty.clone(),
            Branch(expr) => {
                // Careful: The following assertion has potential for introducing exponential
                // slowdown.
                //assert!(expr.true_expr.ty() == expr.false_expr.ty());

                expr.true_expr.ty()
            }
            Var(expr) => expr.ty.clone(),
            Call(expr) => expr.func.ty(),
            Literal(expr) => expr.literal.ty.clone(),
            Field(expr) => expr.ty.clone(),
        }
    }
}
