use std::{
    fmt::{Display, Formatter, Result, Write},
    rc::Rc,
};

use super::{BinaryOp, BuiltInType, Expr, SamplerType, Type, UnaryOp};

impl Display for BinaryOp {
    fn fmt(&self, f: &mut Formatter) -> Result {
        use BinaryOp::*;

        let s = match self {
            Add => "+",
            Sub => "-",
            Mul => "*",
            Div => "/",
            And => "&&",
            Or => "||",
            Eq => "==",
            Lt => "<",
            Le => "<=",
            Ne => "!=",
            Ge => ">=",
            Gt => ">",
            Shl => "<<",
            Shr => ">>",
            BitAnd => "&",
            BitOr => "|",
            BitXor => "^",
            Rem => "%",
        };

        f.write_str(s)
    }
}

impl Display for UnaryOp {
    fn fmt(&self, f: &mut Formatter) -> Result {
        use UnaryOp::*;

        let s = match self {
            Neg => "-",
            Not => "!",
            BitNot => "~",
        };

        f.write_str(s)
    }
}

fn write_call(f: &mut impl Write, name: &str, args: &[Rc<Expr>]) -> Result {
    f.write_str(name)?;
    f.write_char('(')?;

    for (i, arg) in args.iter().enumerate() {
        write!(f, "{arg}")?;

        if i + 1 < args.len() {
            f.write_str(", ")?;
        }
    }

    f.write_char(')')
}

impl Display for Expr {
    fn fmt(&self, f: &mut Formatter) -> Result {
        use Expr::*;

        match self {
            Arg { name, .. } => f.write_str(name),
            ScalarLiteral { value, .. } => f.write_str(value),
            StructLiteral { args, ty } => write_call(f, &ty.name, args),
            Unary { op, arg, .. } => write!(f, "{op} {arg}"),
            Binary {
                left, op, right, ..
            } => write!(f, "({left} {op} {right})"),
            CallFuncDef { def, args } => write_call(f, def.name, args),
            CallBuiltIn { name, args, .. } => write_call(f, name, args),
            Field { base, name, .. } => write!(f, "{base}.{name}"),
            Subscript { base, index, .. } => write!(f, "{base}[{index}]"),
            Branch { cond, yes, no, .. } => write!(f, "({cond} ? {yes} : {no})"),
        }
    }
}

impl Display for SamplerType {
    fn fmt(&self, f: &mut Formatter) -> Result {
        use SamplerType::*;

        let s = match self {
            ColorSampler2d => "sampler2D",
            IColorSampler2d => "isampler2D",
            UColorSampler2d => "usampler2D",
            ComparisonSampler2d => "sampler2DShadow",
        };

        f.write_str(s)
    }
}

impl Display for BuiltInType {
    fn fmt(&self, f: &mut Formatter) -> Result {
        use BuiltInType::*;

        let s = match self {
            F32 => "float",
            I32 => "int",
            U32 => "uint",
            Bool => "bool",
            Vec2 => "vec2",
            IVec2 => "ivec2",
            UVec2 => "uvec2",
            BVec2 => "bvec2",
            Vec3 => "vec3",
            IVec3 => "ivec3",
            UVec3 => "uvec3",
            BVec3 => "bvec3",
            Vec4 => "vec4",
            IVec4 => "ivec4",
            UVec4 => "uvec4",
            BVec4 => "bvec4",
            Mat2 => "mat2",
            Mat3 => "mat3",
            Mat4 => "mat4",
            Sampler(sampler) => return write!(f, "{sampler}"),
        };

        f.write_str(s)
    }
}

impl Display for Type {
    fn fmt(&self, f: &mut Formatter) -> Result {
        use Type::*;

        match self {
            BuiltIn(ty) => write!(f, "{ty}"),
            Struct(ty) => write!(f, "{}", ty.name),
            Array(ty, size) => write!(f, "{ty}[{size}]"),
        }
    }
}
