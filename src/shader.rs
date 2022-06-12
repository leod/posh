use std::marker::PhantomData;

use crate::{
    lang::{Expr, Ident},
    value::Lift,
    Posh, Struct, Value, Vec3, Vec4, F32, I32,
};

pub trait Descriptor: Lift {
    fn func_arg() -> Posh<Self>;
}

pub trait DescriptorSet: Lift {
    fn func_arg() -> Posh<Self>;
}

impl<D> DescriptorSet for D
where
    D: Descriptor,
{
    fn func_arg() -> Posh<Self> {
        <Self as Descriptor>::func_arg()
    }
}

pub trait Vertex: Struct {}

pub trait VertexIn: Struct {}

impl<V: Vertex> VertexIn for V {}

pub trait VertexOut: Struct {}

pub trait FragmentOut: Struct {}

#[derive(Clone, Copy)]
pub struct VertStageIn<V: VertexIn> {
    pub vertex: Posh<V>,
    pub vertex_id: I32,
    pub instance_id: I32,
}

pub struct VertStageOut<W: VertexOut> {
    pub position: Vec3<f32>,
    pub varying: Posh<W>,
}

pub struct FragStageIn<W: VertexOut> {
    pub varying: Posh<W>,
    pub frag_coord: Vec4<f32>,
}

pub struct FragStageOut<R: FragmentOut> {
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

impl<R: FragmentOut> FragStageOut<R> {
    pub fn new(fragment: Posh<R>) -> Self {
        Self {
            fragment,
            frag_depth: None,
        }
    }
}

impl<V: VertexIn> VertStageIn<V> {
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

impl<W: VertexOut> FragStageIn<W> {
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

impl<D, V, R> Shader<D, V, R>
where
    D: DescriptorSet,
    V: VertexIn,
    R: FragmentOut,
{
    pub fn new<W, VS, FS>(vertex_stage: VS, fragment_stage: FS) -> Self
    where
        W: VertexOut,
        VS: FnOnce(Posh<D>, VertStageIn<V>) -> VertStageOut<W>,
        FS: FnOnce(Posh<D>, FragStageIn<W>) -> FragStageOut<R>,
    {
        let vertex_out = vertex_stage(D::func_arg(), VertStageIn::func_arg());
        let fragment_out = fragment_stage(D::func_arg(), FragStageIn::func_arg());

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
