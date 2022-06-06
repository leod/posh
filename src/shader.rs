use std::marker::PhantomData;

use crate::{
    lang::{Expr, ExprBuiltInVar, Type, TypeBuiltIn},
    value::{BuiltInValue, BuiltInValueType},
    Posh, Value, Vec4, F32, I32,
};

pub trait Param {}

pub trait ParamSet {
    fn fields() -> Vec<(String, Type)>;
}

pub trait ParamSets {
    fn fields() -> Vec<Vec<(String, Type)>>;
}

impl<P: ParamSet> ParamSets for P {
    fn fields() -> Vec<Vec<(String, Type)>> {
        vec![P::fields()]
    }
}

pub trait Vertex: Value {
    fn fields() -> Vec<(String, TypeBuiltIn)>;
}

/*impl Vertex for () {
    fn fields() -> Vec<(String, TypeBuiltIn)> {
        Vec::new()
    }
}*/

pub trait VertexSet: Value {}

impl<V: Vertex> VertexSet for V {}

pub trait Varying: Value {
    fn fields() -> Vec<(String, TypeBuiltIn)>;
}

//impl Varying for () {}

pub trait Fragment: Value {
    fn fields() -> Vec<(String, TypeBuiltIn)>;
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
    V::from_expr(Expr::BuiltInVar(ExprBuiltInVar {
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
}

impl<P, V, R> Shader<P, V, R>
where
    P: ParamSets,
    V: VertexSet,
    R: Fragment,
{
    pub fn new<W, VS, FS>(vertex_stage: VS, fragment_stage: FS)
    where
        W: Varying,
        VS: FnOnce(P, VertexIn<V>) -> VertexOut<W>,
        FS: FnOnce(P, FragmentIn<W>) -> FragmentOut<R>,
    {
    }

    fn stage_expr<X, Y, S>(stage: S) -> Expr
    where
        Y: Value,
        S: FnOnce(P, X) -> Y,
    {
        unimplemented!()
    }
}
