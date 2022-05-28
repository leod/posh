pub use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Type {
    U32,
    F32,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Ident {
    pub name: String,
    pub uuid: Uuid,
}

impl Ident {
    pub fn new(name: String) -> Self {
        Self {
            name,
            uuid: Uuid::new_v4(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Var {
    pub ident: Ident,
    pub ty: Type,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Function {
    BuiltIn(String),
    UserDefined {
        name: Ident,
        params: Vec<Var>,
        result: Expr,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BinOp {
    Add,
    Mul,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expr {
    Binary(ExprBinary),
    If(ExprIf),
    Var(ExprVar),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExprBinary {
    pub left: Box<ExprBinary>,
    pub op: BinOp,
    pub right: Box<ExprBinary>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExprIf {
    pub cond: Box<Expr>,
    pub then_branch: Box<Expr>,
    pub else_branch: Box<Expr>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExprVar {
    pub var: Var,
    pub init: Box<Expr>,
}

