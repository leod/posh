use crate::{Vec4, I32, F32, Value, lang::{Type, TypeBuiltIn}};

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

pub trait Vertex {
    fn fields() -> Vec<(String, TypeBuiltIn)>;
}

impl Vertex for () {
    fn fields() -> Vec<(String, TypeBuiltIn)> {
        Vec::new()
    }
}

pub trait VertexSet {}

impl<V: Vertex> VertexSet for V {}

pub trait Varying: Value {
    fn fields() -> Vec<(String, TypeBuiltIn)>;
}

//impl Varying for () {}

pub trait Fragment: Value {
    fn fields() -> Vec<(String, TypeBuiltIn)>;
}

pub struct VertexIn<U: VertexSet> {
    pub vertex: U,
    pub vertex_id: I32,
    pub instance_id: I32,
}

pub struct VertexOut<V: Varying> {
    pub position: Vec4<f32>,
    pub varying: V,
}

pub struct FragmentIn<V: Varying> {
    pub varying: V,
    pub frag_coord: Vec4<f32>,
}

pub struct FragmentOut<F: Fragment> {
    pub fragment: F,
    pub frag_depth: Option<F32>,
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

impl<F: Fragment> FragmentOut<F> {
    pub fn new(fragment: F) -> Self {
        Self { fragment, frag_depth: None }
    }
}