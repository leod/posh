pub mod defs;
pub(crate) mod expr_reg;
pub mod show;

use std::rc::Rc;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Ty {
    BuiltIn(BuiltInTy),
    Struct(StructTy),
    Name(NameTy),
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
    pub name: String,
    pub fields: Vec<(String, Ty)>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NameTy {
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Func {
    Name(NameFunc),
    Def(FuncDef),
    Struct(StructFunc),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NameFunc {
    pub name: String,
    pub result_ty: Ty,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FuncDef {
    pub name: String,
    pub params: Vec<(String, Ty)>,
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
            Name(NameFunc { result_ty: ty, .. }) => ty.clone(),
            Def(FuncDef { result, .. }) => result.ty(),
            Struct(StructFunc { ty }) => Ty::Struct(ty.clone()),
        }
    }

    pub fn name(&self) -> &str {
        use Func::*;

        match self {
            Name(NameFunc { name, .. }) => name,
            Def(FuncDef { name, .. }) => name,
            Struct(StructFunc { ty, .. }) => &ty.name,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LiteralExpr {
    pub value: String,
    pub ty: Ty,
}

impl From<bool> for LiteralExpr {
    fn from(x: bool) -> Self {
        Self {
            value: x.to_string(),
            ty: Ty::BuiltIn(BuiltInTy::Scalar(ScalarTy::Bool)),
        }
    }
}

impl From<i32> for LiteralExpr {
    fn from(x: i32) -> Self {
        Self {
            value: x.to_string(),
            ty: Ty::BuiltIn(BuiltInTy::Scalar(ScalarTy::I32)),
        }
    }
}

impl From<u32> for LiteralExpr {
    fn from(x: u32) -> Self {
        Self {
            value: x.to_string(),
            ty: Ty::BuiltIn(BuiltInTy::Scalar(ScalarTy::U32)),
        }
    }
}

impl From<f32> for LiteralExpr {
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
    Var(VarExpr),
    Literal(LiteralExpr),
    Binary(BinaryExpr),
    Branch(BranchExpr),
    Call(CallExpr),
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
    pub name: String,
    pub ty: Ty,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CallExpr {
    pub func: Func,
    pub args: Vec<Rc<Expr>>,
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
            Literal(expr) => expr.ty.clone(),
            Field(expr) => expr.ty.clone(),
        }
    }
}
