use std::marker::PhantomData;

use crate::{
    lang::{CallExpr, Expr, Func, Ident, Ty, UserDefinedFunc, VarExpr},
    Struct, Type, Posh, Value, Vec3, Vec4, F32, I32,
};

pub trait Descriptor {
    fn func_arg(name: String) -> Self;
}

pub trait DescriptorSet: Struct {
    fn func_arg(name: String) -> Posh<Self>;
}

impl<D> DescriptorSet for D
where
    D: Descriptor + Struct,
{
    fn func_arg(name: String) -> Posh<Self> {
        todo!()
    }
}

pub trait Vertex: Struct {}

pub trait VertexAttributes: Struct {}

impl<V: Vertex> VertexAttributes for V {}

pub trait VertexOutputs: Struct {}

pub trait FragmentOutputs: Struct {}

#[derive(Clone, Copy)]
pub struct VertIn<V: VertexAttributes> {
    pub vertex: Posh<V>,
    pub vertex_id: I32,
    pub instance_id: I32,
}

pub struct VertOut<W: VertexOutputs> {
    pub position: Vec3<f32>,
    pub varying: Posh<W>,
}

pub struct FragIn<W: VertexOutputs> {
    pub varying: Posh<W>,
    pub frag_coord: Vec4<f32>,
}

pub struct FragOut<R: FragmentOutputs> {
    pub fragment: Posh<R>,
    pub frag_depth: Option<F32>,
}

pub struct VertexFunc {
    pub position: Expr,
    pub outputs: Expr,
}

pub struct FragmentFunc {
    pub outputs: Expr,
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

fn builtin_var<V>(name: &'static str) -> V {
    todo!()
}

fn func_arg<V: Value>(name: &'static str) -> V {
    let expr = Expr::Var(VarExpr {
        ident: Ident::new(name),
        ty: <V::Type as Type>::ty(),
        init: None,
    });

    V::from_expr(expr)
}

impl<R: FragmentOutputs> FragOut<R> {
    pub fn new(fragment: Posh<R>) -> Self {
        Self {
            fragment,
            frag_depth: None,
        }
    }
}

impl<V: VertexAttributes> VertIn<V> {
    pub fn new(vertex: Posh<V>) -> Self {
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

impl<W: VertexOutputs> FragIn<W> {
    pub fn new(varying: Posh<W>) -> Self {
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
    V: VertexAttributes,
    R: FragmentOutputs,
{
    pub fn new<W, VS, FS>(vertex_stage: VS, fragment_stage: FS) -> Self
    where
        D: DescriptorSet,
        V: VertexAttributes,
        W: VertexOutputs,
        R: FragmentOutputs,
        VS: FnOnce(Posh<D>, VertIn<V>) -> VertOut<W>,
        FS: FnOnce(Posh<D>, FragIn<W>) -> FragOut<R>,
    {
        let params = || D::func_arg("params".to_string());
        let vertex_out = vertex_stage(params(), VertIn::func_arg());
        let fragment_out = fragment_stage(params(), FragIn::func_arg());

        let vertex = VertexFunc {
            position: vertex_out.position.expr(),
            outputs: vertex_out.varying.expr(),
        };

        let fragment = FragmentFunc {
            outputs: fragment_out.fragment.expr(),
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
