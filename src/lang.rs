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
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
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
pub enum Func {
    BuiltIn(String),
    UserDefined {
        name: Ident,
        params: Vec<Var>,
        result: Box<Expr>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Lit {
    pub value: String,
    pub ty: Type,
}

impl From<u32> for Lit {
    fn from(x: u32) -> Self {
        Self {
            value: x.to_string(),
            ty: Type::U32,
        }
    }
}

impl From<f32> for Lit {
    fn from(x: f32) -> Self {
        Self {
            value: x.to_string(),
            ty: Type::F32,
        }
    }
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
    Call(ExprCall),
    Lit(ExprLit),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExprBinary {
    pub left: Box<Expr>,
    pub op: BinOp,
    pub right: Box<Expr>,
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
    pub init: Option<Box<Expr>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExprCall {
    pub func: Func,
    pub args: Vec<Expr>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExprLit {
    pub lit: Lit,
}
