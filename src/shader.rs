use std::marker::PhantomData;

use crate::{
    lang::{BuiltInTy, BuiltInVarExpr, CallExpr, Expr, Func, Ident, Ty, UserDefinedFunc, VarExpr},
    value::{BuiltIn, BuiltInValue},
    Struct, Type, Val, Value, Vec3, Vec4, F32, I32,
};

pub trait Descriptor {
    type Param;

    fn func_arg(name: String) -> Self;
}

pub trait DescriptorSet {
    type Param;

    fn func_arg(name: String) -> Self;
}

impl<D> DescriptorSet for D
where
    D: Descriptor + Struct,
{
    type Param = <D as Type>::Value;

    fn func_arg(name: String) -> Self {
        todo!()
    }
}

pub trait Vertex: Struct {}

pub trait VertexSet: Struct {}

impl<V: Vertex> VertexSet for V {}

pub trait Varying: Struct {}

pub trait Fragment: Struct {}

#[derive(Clone, Copy)]
pub struct VertIn<V: VertexSet> {
    pub vertex: Val<V>,
    pub vertex_id: I32,
    pub instance_id: I32,
}

pub struct VertOut<W: Varying> {
    pub position: Vec3<f32>,
    pub varying: Val<W>,
}

pub struct FragIn<W: Varying> {
    pub varying: Val<W>,
    pub frag_coord: Vec4<f32>,
}

pub struct FragOut<R: Fragment> {
    pub fragment: Val<R>,
    pub frag_depth: Option<F32>,
}

pub struct VertexFunc {
    pub position: Expr,
    pub varying: Expr,
}

pub struct FragmentFunc {
    pub fragment: Expr,
    pub frag_depth: Option<Expr>,
}

pub struct Shader<P, V, R> {
    vertex: VertexFunc,
    fragment: FragmentFunc,
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

fn func_arg<V: Value>(name: &'static str) -> V {
    let expr = Expr::Var(VarExpr {
        ident: Ident::new(name),
        ty: <V::Type as Type>::ty(),
        init: None,
    });

    V::from_expr(expr)
}

impl<R: Fragment> FragOut<R> {
    pub fn new(fragment: Val<R>) -> Self {
        Self {
            fragment,
            frag_depth: None,
        }
    }
}

impl<V: VertexSet> VertIn<V> {
    pub fn new(vertex: Val<V>) -> Self {
        Self {
            vertex,
            vertex_id: builtin_var("gl_VertexID"),
            instance_id: builtin_var("gl_InstanceID"),
        }
    }

    pub fn func_arg() -> Self {
        Self::new(func_arg("input"))
    }
}

impl<W: Varying> FragIn<W> {
    pub fn new(varying: Val<W>) -> Self {
        Self {
            varying,
            frag_coord: builtin_var("gl_FragCoord"),
        }
    }

    pub fn func_arg() -> Self {
        Self::new(func_arg("input"))
    }
}

impl<D, V, R> Shader<D, V, R>
where
    D: DescriptorSet,
    V: VertexSet,
    R: Fragment,
{
    pub fn new<W, VS, FS>(vertex_stage: VS, fragment_stage: FS) -> Self
    where
        W: Varying,
        VS: FnOnce(D, VertIn<V>) -> VertOut<W>,
        FS: FnOnce(D, FragIn<W>) -> FragOut<R>,
    {
        let params = || D::func_arg("params".to_string());
        let vertex_out = vertex_stage(params(), VertIn::func_arg());
        let fragment_out = fragment_stage(params(), FragIn::func_arg());

        let vertex = VertexFunc {
            position: vertex_out.position.expr(),
            varying: vertex_out.varying.expr(),
        };

        let fragment = FragmentFunc {
            fragment: fragment_out.fragment.expr(),
            frag_depth: fragment_out.frag_depth.map(|v| v.expr()),
        };

        Self {
            vertex,
            fragment,
            _phantom: PhantomData,
        }
    }
}

/*
pub fn shader<P, V, R, W, VS, FS>(vertex_stage: VS, fragment_stage: FS) -> Shader<P, V, R>
where
    P: ParamSets,
    V: VertexSet,
    R: Fragment,
    W: Varying,
    VS: FnOnce(Val<P>, VertIn<V>) -> VertOut<W>,
    FS: FnOnce(Val<P>, FragIn<W>) -> FragOut<R>,
{
    Shader::<P, V, R>::new(vertex_stage, fragment_stage)
}
*/
