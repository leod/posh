use std::marker::PhantomData;

use crate::{
    lang::{Expr, Ident},
    value::{Constructible, Lift},
    Posh, Value, Vec3, Vec4, F32, I32,
};

pub trait Resource {
    fn func_arg() -> Self;
}

pub trait Resources {
    fn func_arg() -> Self;
}

impl<D> Resources for D
where
    D: Resource,
{
    fn func_arg() -> Self {
        <Self as Resource>::func_arg()
    }
}

pub trait Vertex: Constructible {}

pub trait VertexIn: Constructible {}

impl<V: Vertex> VertexIn for V {}

impl<V1: Vertex, V2: Vertex> VertexIn for (V1, V2) {}

pub trait VertexOut: Constructible {}

pub trait FragmentOut: Constructible {}

#[derive(Clone, Copy)]
pub struct VSIn<V: Lift> {
    pub vertex: Posh<V>,
    pub vertex_id: I32,
    pub instance_id: I32,
}

pub struct VSOut<W: Lift> {
    pub position: Vec3<f32>,
    pub varying: Posh<W>,
}

pub struct FSIn<W: Lift> {
    pub varying: Posh<W>,
    pub frag_coord: Vec4<f32>,
}

pub struct FSOut<R: Lift> {
    pub fragment: Posh<R>,
    pub frag_depth: Option<F32>,
}

pub struct ErasedVertexFunc {
    pub position: Expr,
    pub varying: Expr,
}

pub struct ErasedFragmentFunc {
    pub fragment: Expr,
    pub frag_depth: Option<Expr>,
}

pub struct Shader<P, V, R> {
    vertex: ErasedVertexFunc,
    fragment: ErasedFragmentFunc,
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

fn builtin_var<V: Value>(name: &'static str) -> V {
    V::from_ident(Ident::new(name))
}

impl<V> VSIn<V>
where
    V: Lift,
    V::Posh: VertexIn,
{
    pub fn new(vertex: Posh<V>) -> Self {
        Self {
            vertex,
            vertex_id: builtin_var("gl_VertexID"),
            instance_id: builtin_var("gl_InstanceID"),
        }
    }

    pub fn func_arg() -> Self {
        Self::new(Posh::<V>::from_ident(Ident::new("input")))
    }
}

impl<W> FSIn<W>
where
    W: Lift,
    W::Posh: VertexOut,
{
    pub fn new(varying: Posh<W>) -> Self {
        Self {
            varying,
            frag_coord: builtin_var("gl_FragCoord"),
        }
    }

    pub fn func_arg() -> Self {
        Self::new(Posh::<W>::from_ident(Ident::new("input")))
    }
}

impl<R> FSOut<R>
where
    R: Lift,
    R::Posh: FragmentOut,
{
    pub fn new(fragment: Posh<R>) -> Self {
        Self {
            fragment,
            frag_depth: None,
        }
    }
}

impl<R, V, F> Shader<R, V, F>
where
    R: Lift,
    V: Lift,
    F: Lift,
    R::Posh: Resources,
    V::Posh: VertexIn,
    F::Posh: FragmentOut,
{
    pub fn new<W, VS, FS>(vertex_stage: VS, fragment_stage: FS) -> Self
    where
        W: Lift,
        W::Posh: VertexOut,
        VS: FnOnce(Posh<R>, VSIn<V>) -> VSOut<W>,
        FS: FnOnce(Posh<R>, FSIn<W>) -> FSOut<F>,
    {
        let vertex_out = vertex_stage(R::Posh::func_arg(), VSIn::func_arg());
        let fragment_out = fragment_stage(R::Posh::func_arg(), FSIn::func_arg());

        let vertex = ErasedVertexFunc {
            position: vertex_out.position.expr(),
            varying: vertex_out.varying.expr(),
        };

        let fragment = ErasedFragmentFunc {
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
