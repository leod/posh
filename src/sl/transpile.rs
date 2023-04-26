//! Transpile a typed program to GLSL source code.
//!
//! This is exposed only in order to make the internally generated source code
//! more transparent. It is typically not necessary to use this module.

use std::{iter::once, rc::Rc};

use crate::{
    interface::{FragmentVisitor, UniformUnion, UniformVisitor, VertexVisitor},
    Block, Fragment, Sl, Uniform, Vertex,
};

use super::{
    codegen,
    dag::{Expr, SamplerType, Type},
    primitives::value_arg,
    program_def::{ProgramDef, UniformBlockDef, UniformSamplerDef, VertexBlockDef},
    ColorSample, ColorSampler2d, ComparisonSampler2d, ConstParams, FragmentInput, FragmentOutput,
    FullVertexOutput, Object, Varying, Vec4, VertexOutput, VertexInput, I32,
};

/// Types that can be used as vertex input for a vertex shader.
pub trait FromVertexInput {
    type Vertex: Vertex<Sl>;

    fn from(input: VertexInput<Self::Vertex>) -> Self;
}

impl<V: Vertex<Sl>> FromVertexInput for VertexInput<V> {
    type Vertex = V;

    fn from(input: Self) -> Self {
        input
    }
}

impl<V: Vertex<Sl>> FromVertexInput for V {
    type Vertex = Self;

    fn from(input: VertexInput<Self>) -> Self {
        input.vertex
    }
}

/// Types that can be used as vertex output for a vertex shader.
pub trait IntoFullVertexOutput {
    type Varying: Varying;

    fn into(self) -> FullVertexOutput<Self::Varying>;
}

impl<W: Varying> IntoFullVertexOutput for FullVertexOutput<W> {
    type Varying = W;

    fn into(self) -> Self {
        self
    }
}

impl<V: Varying> IntoFullVertexOutput for VertexOutput<V> {
    type Varying = V;

    fn into(self) -> FullVertexOutput<V> {
        FullVertexOutput {
            position: self.position,
            varying: self.varying,
            point_size: None,
        }
    }
}

impl IntoFullVertexOutput for Vec4 {
    type Varying = ();

    fn into(self) -> FullVertexOutput<()> {
        FullVertexOutput {
            position: self,
            varying: (),
            point_size: None,
        }
    }
}

/// Types that can be used as fragment input for a fragment shader.
pub trait FromFragmentInput {
    type Varying: Varying;

    fn from(input: FragmentInput<Self::Varying>) -> Self;
}

impl<W: Varying> FromFragmentInput for FragmentInput<W> {
    type Varying = W;

    fn from(input: Self) -> Self {
        input
    }
}

impl<W: Varying> FromFragmentInput for W {
    type Varying = Self;

    fn from(input: FragmentInput<Self>) -> Self {
        input.varying
    }
}

/// Types that can be used as fragment output for a fragment shader.
pub trait IntoFragmentOutput {
    type Fragment: Fragment<Sl>;

    fn into(self) -> FragmentOutput<Self::Fragment>;
}

impl<F: Fragment<Sl>> IntoFragmentOutput for FragmentOutput<F> {
    type Fragment = F;

    fn into(self) -> Self {
        self
    }
}

impl<F: Fragment<Sl>> IntoFragmentOutput for F {
    type Fragment = Self;

    fn into(self) -> FragmentOutput<Self> {
        FragmentOutput {
            fragment: self,
            fragment_depth: None,
        }
    }
}

/// Transpiles a vertex shader and a fragment shader to GLSL source code.
///
/// This is used internally by `posh` in order to create
/// [`Program`](crate::gl::Program)s. It is exposed for the purpose of
/// inspecting generated shader source code.
pub fn transpile_to_program_def<U, U1, U2, V, F, W, InV, OutW, InW, OutF>(
    vertex_shader: fn(U1, InV) -> OutW,
    fragment_shader: fn(U2, InW) -> OutF,
) -> ProgramDef
where
    U1: Uniform<Sl>,
    U2: Uniform<Sl>,
    U: UniformUnion<U1, U2>,
    V: Vertex<Sl>,
    F: Fragment<Sl>,
    W: Varying,
    InV: FromVertexInput<Vertex = V>,
    OutW: IntoFullVertexOutput<Varying = W>,
    InW: FromFragmentInput<Varying = W>,
    OutF: IntoFragmentOutput<Fragment = F>,
{
    transpile_to_program_def_with_consts_impl(
        (),
        |(), uniforms: U, input| vertex_shader(uniforms.lhs(), input),
        |(), uniforms: U, input| fragment_shader(uniforms.rhs(), input),
    )
}

/// Transpiles a vertex shader and a fragment shader with constant input to GLSL
/// source code.
///
/// See also [`transpile_to_program_def`].
pub fn transpile_to_program_def_with_consts<C, U, U1, U2, V, F, W, InV, OutW, InW, OutF>(
    consts: C,
    vertex_shader: fn(C, U1, InV) -> OutW,
    fragment_shader: fn(C, U2, InW) -> OutF,
) -> ProgramDef
where
    C: ConstParams,
    U1: Uniform<Sl>,
    U2: Uniform<Sl>,
    U: UniformUnion<U1, U2>,
    V: Vertex<Sl>,
    F: Fragment<Sl>,
    W: Varying,
    InV: FromVertexInput<Vertex = V>,
    OutW: IntoFullVertexOutput<Varying = W>,
    InW: FromFragmentInput<Varying = W>,
    OutF: IntoFragmentOutput<Fragment = F>,
{
    transpile_to_program_def_with_consts_impl(
        consts,
        |consts, uniforms: U, input| vertex_shader(consts, uniforms.lhs(), input),
        |consts, uniforms: U, input| fragment_shader(consts, uniforms.rhs(), input),
    )
}

fn transpile_to_program_def_with_consts_impl<C, U, V, F, W, InV, OutW, InW, OutF>(
    consts: C,
    vertex_shader: impl FnOnce(C, U, InV) -> OutW,
    fragment_shader: impl FnOnce(C, U, InW) -> OutF,
) -> ProgramDef
where
    C: ConstParams,
    U: Uniform<Sl>,
    V: Vertex<Sl>,
    F: Fragment<Sl>,
    W: Varying,
    InV: FromVertexInput<Vertex = V>,
    OutW: IntoFullVertexOutput<Varying = W>,
    InW: FromFragmentInput<Varying = W>,
    OutF: IntoFragmentOutput<Fragment = F>,
{
    // TODO: Remove hardcoded path names.
    let uniforms = U::shader_input("uniforms");

    let (uniform_block_defs, uniform_sampler_defs) = {
        // TODO: Remove hardcoded path names.
        let mut visitor = CollectUniforms::default();
        uniforms.visit("uniforms", &mut visitor);

        (visitor.block_defs, visitor.sampler_defs)
    };

    let (vertex_block_defs, varying_outputs, vertex_shader_source) = {
        let input = || VertexInput {
            vertex: V::shader_input("vertex_input"),
            vertex_id: value_arg::<I32>("gl_VertexID").as_u32(),
            instance_id: value_arg::<I32>("gl_InstanceID").as_u32(),
            _private: (),
        };
        let output = vertex_shader(consts, uniforms, InV::from(input())).into();

        let varying_outputs = output.varying.shader_outputs("vertex_output");
        let vertex_block_defs = {
            // TODO: Remove hardcoded path names.
            let mut visitor = CollectVertexBlocks::default();
            input().vertex.visit("vertex_input", &mut visitor);

            visitor.block_defs
        };

        let attributes = vertex_block_defs
            .iter()
            .flat_map(|block_def| block_def.attributes.iter())
            .map(|attribute_def| {
                (
                    "in".to_string(),
                    attribute_def.name.clone(),
                    Type::BuiltIn(attribute_def.ty),
                )
            })
            .chain(
                // TODO: Interpolation type.
                varying_outputs.iter().map(|(name, interp, expr)| {
                    let kind = format!("{} out", interp.to_glsl());

                    (kind, name.clone(), expr.ty())
                }),
            );
        let exprs = once(("gl_Position", output.position.expr()))
            .chain(
                varying_outputs
                    .iter()
                    .map(|(name, _, expr)| (name.as_str(), expr.clone())),
            )
            .chain(
                output
                    .point_size
                    .map(|value| ("gl_PointSize", value.expr())),
            );

        let mut source = String::new();
        codegen::write_shader_stage(
            &mut source,
            &uniform_block_defs,
            &uniform_sampler_defs,
            attributes,
            &exprs.collect::<Vec<_>>(),
        )
        .unwrap();

        (vertex_block_defs, varying_outputs, source)
    };

    // TODO: Remove hardcoded path names.
    let uniforms = U::shader_input("uniforms");

    let fragment_shader_source = {
        let input = FragmentInput {
            varying: W::shader_input("vertex_output"),
            fragment_coord: value_arg("gl_FragCoord"),
            front_facing: value_arg("gl_FrontFacing"),
            point_coord: value_arg("gl_PointCoord"),
            _private: (),
        };
        let output = fragment_shader(consts, uniforms, InW::from(input)).into();

        // TODO: Remove hardcoded path names.
        let mut visitor = CollectOutputs::default();
        output.fragment.visit("fragment_output", &mut visitor);

        let attributes = varying_outputs
            .iter()
            .map(|(name, interp, expr)| {
                let kind = format!("{} in", interp.to_glsl());

                (kind, name.clone(), expr.ty())
            })
            .chain(visitor.outputs.iter().enumerate().map(|(i, (name, expr))| {
                (
                    format!("layout(location = {i}) out"),
                    name.clone(),
                    expr.ty(),
                )
            }));

        let exprs = visitor
            .outputs
            .iter()
            .map(|(name, expr)| (name.as_str(), expr.clone()))
            .chain(
                output
                    .fragment_depth
                    .map(|value| ("gl_FragDepth", value.expr())),
            );

        let mut source = String::new();
        codegen::write_shader_stage(
            &mut source,
            &uniform_block_defs,
            &uniform_sampler_defs,
            attributes,
            &exprs.collect::<Vec<_>>(),
        )
        .unwrap();

        source
    };

    ProgramDef {
        uniform_block_defs,
        uniform_sampler_defs,
        vertex_block_defs,
        vertex_shader_source,
        fragment_shader_source,
    }
}

#[derive(Default)]
struct CollectUniforms {
    sampler_defs: Vec<UniformSamplerDef>,
    block_defs: Vec<UniformBlockDef>,
}

impl<'a> UniformVisitor<'a, Sl> for CollectUniforms {
    fn accept_block<U: Block<Sl>>(&mut self, path: &str, _: &U) {
        // TODO: Allow user-specified uniform block locations.
        let block_def = UniformBlockDef {
            block_name: path.to_string() + "_posh_block",
            arg_name: path.to_string(),
            ty: <U::Sl as Object>::ty(),
            location: self.block_defs.len(),
        };

        self.block_defs.push(block_def)
    }

    fn accept_color_sampler_2d<S: ColorSample>(&mut self, path: &str, _: &ColorSampler2d<S>) {
        // TODO: Allow user-specified sampler texture units.
        let sampler_def = UniformSamplerDef {
            name: path.to_string(),
            ty: S::SAMPLER_TYPE,
            texture_unit: self.sampler_defs.len(),
        };

        self.sampler_defs.push(sampler_def);
    }

    fn accept_comparison_sampler_2d(&mut self, path: &str, _: &ComparisonSampler2d) {
        // TODO: Allow user-specified sampler texture units.
        let sampler_def = UniformSamplerDef {
            name: path.to_string(),
            ty: SamplerType::ComparisonSampler2d,
            texture_unit: self.sampler_defs.len(),
        };

        self.sampler_defs.push(sampler_def);
    }
}

#[derive(Default)]
struct CollectVertexBlocks {
    block_defs: Vec<VertexBlockDef>,
}

impl<'a> VertexVisitor<'a, Sl> for CollectVertexBlocks {
    fn accept<B: Block<Sl>>(&mut self, path: &str, _: &B) {
        let block_def = VertexBlockDef {
            attributes: B::vertex_attribute_defs(path),
        };

        self.block_defs.push(block_def);
    }
}

#[derive(Default)]
struct CollectOutputs {
    outputs: Vec<(String, Rc<Expr>)>,
}

impl<'a> FragmentVisitor<'a, Sl> for CollectOutputs {
    fn accept<S: ColorSample>(&mut self, path: &str, output: &S) {
        self.outputs.push((path.to_string(), output.expr()));
    }
}
