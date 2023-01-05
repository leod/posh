use std::{fmt, rc::Rc};

use super::{BaseType, BinaryOp, Expr, NumericType, PrimitiveType, Type};

impl fmt::Display for BinaryOp {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        use BinaryOp::*;

        let s = match self {
            Add => "+",
            Sub => "-",
            Mul => "*",
            Div => "/",
            Eq => "==",
            And => "&&",
            Or => "||",
        };

        f.write_str(s)
    }
}

fn write_call(f: &mut impl fmt::Write, name: &str, args: &[Rc<Expr>]) -> Result<(), fmt::Error> {
    f.write_str(name)?;
    f.write_char('(')?;

    for (i, arg) in args.iter().enumerate() {
        write!(f, "{}", arg)?;

        if i + 1 < args.len() {
            f.write_str(", ")?;
        }
    }

    f.write_char(')')
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        use Expr::*;

        match self {
            Arg { name, .. } => f.write_str(&name),
            ScalarLiteral { value, .. } => f.write_str(&value),
            StructLiteral { args, ty } => write_call(f, ty.name, args),
            Binary {
                left, op, right, ..
            } => write!(f, "({left} {op} {right})"),
            CallFuncDef { def, args } => write_call(f, def.name, args),
            CallBuiltIn { name, args, .. } => write_call(f, name, args),
            Field { base, name, .. } => write!(f, "{base}.{name}"),
            Branch { cond, yes, no, .. } => write!(f, "({cond} ? {yes} : {no}"),
        }
    }
}

impl fmt::Display for NumericType {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        use NumericType::*;

        let s = match self {
            F32 => "float",
            I32 => "int",
            U32 => "uint",
        };

        f.write_str(s)
    }
}

impl fmt::Display for PrimitiveType {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        use PrimitiveType::*;

        match self {
            Numeric(ty) => write!(f, "{ty}"),
            Bool => write!(f, "bool"),
        }
    }
}

fn numeric_type_prefix(ty: NumericType) -> &'static str {
    use NumericType::*;

    match ty {
        F32 => "",
        I32 => "i",
        U32 => "u",
    }
}

fn primitive_type_prefix(ty: PrimitiveType) -> &'static str {
    use PrimitiveType::*;

    match ty {
        Numeric(ty) => numeric_type_prefix(ty),
        Bool => "b",
    }
}

impl fmt::Display for BaseType {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        use BaseType::*;

        match self {
            Scalar(ty) => write!(f, "{ty}"),
            Vec2(ty) => write!(f, "{}vec2", primitive_type_prefix(*ty)),
            Vec3(ty) => write!(f, "{}vec2", primitive_type_prefix(*ty)),
            Vec4(ty) => write!(f, "{}vec2", primitive_type_prefix(*ty)),
            Struct(ty) => write!(f, "{}", ty.name),
            Sampler2d(ty) => write!(f, "{}sampler2D", numeric_type_prefix(*ty)),
        }
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        use Type::*;

        match self {
            Base(ty) => write!(f, "{ty}"),
            Array(ty, size) => write!(f, "{ty}[{size}]"),
        }
    }
}
