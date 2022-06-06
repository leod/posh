use std::marker::PhantomData;

use crate::{
    lang::{BuiltInTy, BuiltInVarExpr, CallExpr, Expr, Func, Ty, UserDefinedFunc},
    value::{BuiltIn, BuiltInValue},
    Value, Vec4, F32, I32,
};

pub trait Param {}

pub trait ParamSet {
    fn fields() -> Vec<(String, Ty)>;
    fn func_arg() -> Self;
}

pub trait ParamSets {
    fn fields() -> Vec<Vec<(String, Ty)>>;
    fn func_arg() -> Self;
}

impl<P: ParamSet> ParamSets for P {
    fn fields() -> Vec<Vec<(String, Ty)>> {
        vec![P::fields()]
    }

    fn func_arg() -> Self {
        P::func_arg()
    }
}

pub trait Vertex: Value {
    fn fields() -> Vec<(String, BuiltInTy)>;
    fn func_arg() -> Self;
}

/*impl Vertex for () {
    fn fields() -> Vec<(String, TypeBuiltIn)> {
        Vec::new()
    }
}*/

pub trait VertexSet: Value {
    fn fields() -> Vec<Vec<(String, BuiltInTy)>>;
    fn func_arg() -> Self;
}

impl<V: Vertex> VertexSet for V {
    fn fields() -> Vec<Vec<(String, BuiltInTy)>> {
        vec![V::fields()]
    }

    fn func_arg() -> Self {
        V::func_arg()
    }
}

pub trait Varying: Value {
    fn fields() -> Vec<(String, BuiltInTy)>;
    fn func_arg() -> Self;
}

//impl Varying for () {}

pub trait Fragment: Value {
    fn fields() -> Vec<(String, BuiltInTy)>;
}

pub struct VertexIn<V: VertexSet> {
    pub vertex: V,
    pub vertex_id: I32,
    pub instance_id: I32,
}

pub struct VertexOut<W: Varying> {
    pub position: Vec4<f32>,
    pub varying: W,
}

pub struct FragmentIn<W: Varying> {
    pub varying: W,
    pub frag_coord: Vec4<f32>,
}

pub struct FragmentOut<R: Fragment> {
    pub fragment: R,
    pub frag_depth: Option<F32>,
}

pub struct Shader<P, V, R> {
    vertex: UserDefinedFunc,
    fragment: UserDefinedFunc,
    _phantom: PhantomData<(P, V, R)>,
}

// TODO: Figure out if we can have Expr::Void for this.
/*impl VertexOut<()> {
    pub fn new(position: Vec4<f32>) -> Self {
        Self {
            position,
            varying: (),
        }
    }
}*/

fn builtin_var<V: BuiltInValue>(name: &'static str) -> V {
    V::from_expr(Expr::BuiltInVar(BuiltInVarExpr {
        name: name.to_string(),
        ty: V::BuiltInType::built_in_ty(),
    }))
}

impl<R: Fragment> FragmentOut<R> {
    pub fn new(fragment: R) -> Self {
        Self {
            fragment,
            frag_depth: None,
        }
    }
}

impl<V: VertexSet> VertexIn<V> {
    pub fn new(vertex: V) -> Self {
        Self {
            vertex,
            vertex_id: builtin_var("gl_VertexID"),
            instance_id: builtin_var("gl_InstanceID"),
        }
    }

    pub fn func_arg() -> Self {
        Self::new(V::func_arg())
    }
}

impl<W: Varying> FragmentIn<W> {
    pub fn new(varying: W) -> Self {
        Self {
            varying,
            frag_coord: builtin_var("gl_FragCoord"),
        }
    }

    pub fn func_arg() -> Self {
        Self::new(W::func_arg())
    }
}

impl<P, V, R> Shader<P, V, R>
where
    P: ParamSets,
    V: VertexSet,
    R: Fragment,
{
    pub fn new<W, VS, FS>(vertex_stage: VS, fragment_stage: FS) -> Self
    where
        W: Varying,
        VS: FnOnce(P, VertexIn<V>) -> VertexOut<W>,
        FS: FnOnce(P, FragmentIn<W>) -> FragmentOut<R>,
    {
        let vertex_out = vertex_stage(P::func_arg(), VertexIn::func_arg());
        let fragment_out = fragment_stage(P::func_arg(), FragmentIn::func_arg());

        unimplemented!()
        /*Self {
            vertex: Self::stage_func(vertex_out),
            fragment: Self::stage_func(fragment_out),
        }*/
    }

    fn stage_func<X: Value>(value: X) -> UserDefinedFunc {
        if let Expr::Call(CallExpr {
            func: Func::UserDefined(func),
            args: _,
        }) = value.expr()
        {
            func
        } else {
            panic!("Expected shader stage to be #[posh]");
        }
    }
}
