use std::marker::PhantomData;

use crate::{
    lang::{Expr, Ident},
    value::{Binding, Constructible},
    Po, Value, Vec3, Vec4,
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
pub struct VSIn<V: Binding> {
    pub vertex: Po<V>,
    pub vertex_id: Po<i32>,
    pub instance_id: Po<i32>,
}

pub struct VSOut<W: Binding> {
    pub position: Vec3<f32>,
    pub varying: Po<W>,
}

pub struct FSIn<W: Binding> {
    pub varying: Po<W>,
    pub frag_coord: Vec4<f32>,
}

pub struct FSOut<R: Binding> {
    pub fragment: Po<R>,
    pub frag_depth: Option<Po<f32>>,
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
    V: Binding,
    V::Type: VertexIn,
{
    pub fn new(vertex: Po<V>) -> Self {
        Self {
            vertex,
            vertex_id: builtin_var("gl_VertexID"),
            instance_id: builtin_var("gl_InstanceID"),
        }
    }

    pub fn func_arg() -> Self {
        Self::new(Po::<V>::from_ident(Ident::new("input")))
    }
}

impl<W> FSIn<W>
where
    W: Binding,
    W::Type: VertexOut,
{
    pub fn new(varying: Po<W>) -> Self {
        Self {
            varying,
            frag_coord: builtin_var("gl_FragCoord"),
        }
    }

    pub fn func_arg() -> Self {
        Self::new(Po::<W>::from_ident(Ident::new("input")))
    }
}

impl<R> FSOut<R>
where
    R: Binding,
    R::Type: FragmentOut,
{
    pub fn new(fragment: Po<R>) -> Self {
        Self {
            fragment,
            frag_depth: None,
        }
    }
}

impl<R, V, F> Shader<R, V, F>
where
    R: Binding,
    V: Binding,
    F: Binding,
    R::Type: Resources,
    V::Type: VertexIn,
    F::Type: FragmentOut,
{
    pub fn new<W, VS, FS>(vertex_stage: VS, fragment_stage: FS) -> Self
    where
        W: Binding,
        W::Type: VertexOut,
        VS: FnOnce(Po<R>, VSIn<V>) -> VSOut<W>,
        FS: FnOnce(Po<R>, FSIn<W>) -> FSOut<F>,
    {
        let vertex_out = vertex_stage(R::Type::func_arg(), VSIn::func_arg());
        let fragment_out = fragment_stage(R::Type::func_arg(), FSIn::func_arg());

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
