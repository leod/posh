use super::{Bool, Vec2, Vec4, F32, U32};

/// Constants that can be passed to a shader at shader build time.
pub trait ConstParams: Copy {}

impl ConstParams for () {}

/// Per-vertex input given to a vertex shader.
#[derive(Debug, Clone)]
pub struct VertexInput<Vert> {
    pub vertex: Vert,
    pub vertex_id: U32,
    pub instance_id: U32,
    pub(crate) _private: (),
}

/// Per-vertex output computed by a vertex shader.
#[derive(Debug, Clone)]
pub struct VertexOutput<Vary> {
    pub varying: Vary,
    pub position: Vec4,
    pub point_size: Option<F32>,
}

/// Per-vertex position and varying output computed by a vertex shader.
#[derive(Debug, Clone)]
pub struct VaryingOutput<Vary> {
    pub varying: Vary,
    pub position: Vec4,
}

/// Per-fragment input given to a fragment shader.
#[derive(Debug, Clone)]
pub struct FragmentInput<Vary> {
    pub varying: Vary,
    pub fragment_coord: Vec4,
    pub front_facing: Bool,
    pub point_coord: Vec2,
    pub(crate) _private: (),
}

/// Per-fragment output computed by a fragment shader.
#[derive(Debug, Clone)]
pub struct FragmentOutput<Frag> {
    pub fragment: Frag,
    pub fragment_depth: Option<F32>,
}

impl<Frag> FragmentOutput<Frag> {
    pub fn new(fragment: Frag) -> Self {
        FragmentOutput {
            fragment,
            fragment_depth: None,
        }
    }
}
