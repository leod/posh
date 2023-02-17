use crate::{FragmentData, Logical, VertexData};

use super::{Bool, Varying, Vec2, Vec4, F32, U32};

/// Constants that can be passed to a shader at shader build time.
pub trait ConstInput: Copy {}

impl ConstInput for () {}

#[derive(Debug, Clone)]
pub(crate) struct Private;

/// The full per-vertex input given to a vertex shader.
#[derive(Debug, Clone)]
pub struct VertexInput<Vert> {
    pub vertex: Vert,
    pub vertex_id: U32,
    pub instance_id: U32,
    pub(crate) _private: Private,
}

/// Types that can be used as vertex input for a vertex shader.
pub trait FromVertexInput {
    type Vert: VertexData<Logical>;

    fn from(input: VertexInput<Self::Vert>) -> Self;
}

impl<Vert: VertexData<Logical>> FromVertexInput for VertexInput<Vert> {
    type Vert = Vert;

    fn from(input: Self) -> Self {
        input
    }
}

impl<Vert: VertexData<Logical>> FromVertexInput for Vert {
    type Vert = Self;

    fn from(input: VertexInput<Self>) -> Self {
        input.vertex
    }
}

/// The full output computed by a vertex shader.
#[derive(Debug, Clone)]
pub struct VertexOutput<Vary> {
    pub varying: Vary,
    pub position: Vec4<f32>,
    pub point_size: Option<F32>,
}

/// Types that can be used as vertex output for a vertex shader.
pub trait IntoVertexOutput {
    type Vary: Varying;

    fn into(self) -> VertexOutput<Self::Vary>;
}

impl<Vary: Varying> IntoVertexOutput for VertexOutput<Vary> {
    type Vary = Vary;

    fn into(self) -> Self {
        self
    }
}

impl IntoVertexOutput for Vec4<f32> {
    type Vary = ();

    fn into(self) -> VertexOutput<()> {
        VertexOutput {
            varying: (),
            position: self,
            point_size: None,
        }
    }
}

/// Position output and varying output computed by a vertex shader.
#[derive(Debug, Clone)]
pub struct VaryingOutput<Vary> {
    pub varying: Vary,
    pub position: Vec4<f32>,
}

impl<Vary: Varying> IntoVertexOutput for VaryingOutput<Vary> {
    type Vary = Vary;

    fn into(self) -> VertexOutput<Vary> {
        VertexOutput {
            position: self.position,
            varying: self.varying,
            point_size: None,
        }
    }
}

/// The full per-fragment input given to a fragment shader.
#[derive(Debug, Clone)]
pub struct FragmentInput<Vary> {
    pub varying: Vary,
    pub fragment_coord: Vec4<f32>,
    pub front_facing: Bool,
    pub point_coord: Vec2<f32>,
    pub(crate) _private: Private,
}

/// Types that can be used as fragment input for a fragment shader.
pub trait FromFragmentInput {
    type Vary: Varying;

    fn from(input: FragmentInput<Self::Vary>) -> Self;
}

impl<Vary: Varying> FromFragmentInput for FragmentInput<Vary> {
    type Vary = Vary;

    fn from(input: Self) -> Self {
        input
    }
}

impl<Vary: Varying> FromFragmentInput for Vary {
    type Vary = Self;

    fn from(input: FragmentInput<Self>) -> Self {
        input.varying
    }
}

/// The full output computed by a fragment shader.
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

/// Types that can be used as fragment output for a fragment shader.
pub trait IntoFragmentOutput {
    type Frag: FragmentData<Logical>;

    fn into(self) -> FragmentOutput<Self::Frag>;
}

impl<Frag: FragmentData<Logical>> IntoFragmentOutput for FragmentOutput<Frag> {
    type Frag = Frag;

    fn into(self) -> Self {
        self
    }
}

impl<Frag: FragmentData<Logical>> IntoFragmentOutput for Frag {
    type Frag = Self;

    fn into(self) -> FragmentOutput<Self> {
        FragmentOutput {
            fragment: self,
            fragment_depth: None,
        }
    }
}
