//! Transpile a typed program to GLSL source code.
//!
//! This is exposed only in order to make the internally generated source code
//! more transparent. It is typically not necessary to use this module.

use std::{iter::once, rc::Rc};

use crevice::std140::AsStd140;

use crate::{
    interface::{FragmentVisitor, UniformUnion, UniformVisitor, VertexVisitor},
    Block, Fragment, SlView, Uniform, Vertex,
};

use super::{
    codegen,
    dag::{Expr, SamplerType, Type},
    primitives::value_arg,
    program_def::{
        ProgramDef, UniformBlockDef, UniformSamplerDef, VertexAttributeDef, VertexDef,
        VertexInputRate,
    },
    ConstParams, FragmentInput, FragmentOutput, Object, Private, Sample, Sampler2d, Varying,
    VaryingOutput, Vec4, VertexInput, VertexOutput,
};

/// Types that can be used as vertex input for a vertex shader.
pub trait FromVertexInput {
    type Vertex: Vertex<SlView>;

    fn from(input: VertexInput<Self::Vertex>) -> Self;
}

impl<V: Vertex<SlView>> FromVertexInput for VertexInput<V> {
    type Vertex = V;

    fn from(input: Self) -> Self {
        input
    }
}

impl<V: Vertex<SlView>> FromVertexInput for V {
    type Vertex = Self;

    fn from(input: VertexInput<Self>) -> Self {
        input.vertex
    }
}

/// Types that can be used as vertex output for a vertex shader.
pub trait IntoVertexOutput {
    type Varying: Varying;

    fn into(self) -> VertexOutput<Self::Varying>;
}

impl<W: Varying> IntoVertexOutput for VertexOutput<W> {
    type Varying = W;

    fn into(self) -> Self {
        self
    }
}

impl<V: Varying> IntoVertexOutput for VaryingOutput<V> {
    type Varying = V;

    fn into(self) -> VertexOutput<V> {
        VertexOutput {
            position: self.position,
            varying: self.varying,
            point_size: None,
        }
    }
}

impl IntoVertexOutput for Vec4 {
    type Varying = ();

    fn into(self) -> VertexOutput<()> {
        VertexOutput {
            varying: (),
            position: self,
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
    type Fragment: Fragment<SlView>;

    fn into(self) -> FragmentOutput<Self::Fragment>;
}

impl<F: Fragment<SlView>> IntoFragmentOutput for FragmentOutput<F> {
    type Fragment = F;

    fn into(self) -> Self {
        self
    }
}

impl<F: Fragment<SlView>> IntoFragmentOutput for F {
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
    U1: Uniform<SlView>,
    U2: Uniform<SlView>,
    U: UniformUnion<U1, U2>,
    V: Vertex<SlView>,
    F: Fragment<SlView>,
    W: Varying,
    InV: FromVertexInput<Vertex = V>,
    OutW: IntoVertexOutput<Varying = W>,
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
    U1: Uniform<SlView>,
    U2: Uniform<SlView>,
    U: UniformUnion<U1, U2>,
    V: Vertex<SlView>,
    F: Fragment<SlView>,
    W: Varying,
    InV: FromVertexInput<Vertex = V>,
    OutW: IntoVertexOutput<Varying = W>,
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
    U: Uniform<SlView>,
    V: Vertex<SlView>,
    F: Fragment<SlView>,
    W: Varying,
    InV: FromVertexInput<Vertex = V>,
    OutW: IntoVertexOutput<Varying = W>,
    InW: FromFragmentInput<Varying = W>,
    OutF: IntoFragmentOutput<Fragment = F>,
{
    let uniforms = U::shader_input("uniforms");

    let (block_defs, sampler_defs) = {
        let mut visitor = CollectUniforms::default();
        uniforms.visit("uniforms", &mut visitor);

        (visitor.block_defs, visitor.sampler_defs)
    };

    let (vertex_defs, varying_outputs, vertex_shader_source) = {
        let input = || VertexInput {
            vertex: V::shader_input("vertex_input"),
            vertex_id: value_arg("gl_VertexID"),
            instance_id: value_arg("gl_InstanceID"),
            _private: Private,
        };
        let output = vertex_shader(consts, uniforms, InV::from(input())).into();

        let varying_outputs = output.varying.shader_outputs("vertex_output");
        let (vertex_attributes, vertex_defs) = {
            let mut visitor = CollectAttributes::default();
            input().vertex.visit("vertex_input", &mut visitor);

            (visitor.attribute_defs, visitor.vertex_defs)
        };

        let attributes = vertex_attributes
            .into_iter()
            .map(|attribute_def| {
                (
                    "in".to_string(),
                    attribute_def.name,
                    Type::BuiltIn(attribute_def.ty),
                )
            })
            .chain(
                // TODO: Interpolation type.
                varying_outputs
                    .iter()
                    .map(|(name, expr)| ("out".to_string(), name.clone(), expr.ty())),
            );
        let exprs = once(("gl_Position", output.position.expr()))
            .chain(
                varying_outputs
                    .iter()
                    .map(|(name, expr)| (name.as_str(), expr.clone())),
            )
            .chain(
                output
                    .point_size
                    .map(|value| ("gl_PointSize", value.expr())),
            );

        let mut source = String::new();
        codegen::write_shader_stage(
            &mut source,
            &block_defs,
            &sampler_defs,
            attributes,
            &exprs.collect::<Vec<_>>(),
        )
        .unwrap();

        (vertex_defs, varying_outputs, source)
    };

    let uniforms = U::shader_input("uniforms");

    let fragment_shader_source = {
        let input = FragmentInput {
            varying: W::shader_input("vertex_output"),
            fragment_coord: value_arg("gl_FragCoord"),
            front_facing: value_arg("gl_FrontFacing"),
            point_coord: value_arg("gl_PointCoord"),
            _private: Private,
        };
        let output = fragment_shader(consts, uniforms, InW::from(input)).into();

        let mut visitor = CollectOutputs::default();
        output.fragment.visit("fragment_output", &mut visitor);

        let attributes = varying_outputs
            .iter()
            .map(|(name, expr)| {
                // TODO: Interpolation type.
                ("in".to_string(), name.clone(), expr.ty())
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
            &block_defs,
            &sampler_defs,
            attributes,
            &exprs.collect::<Vec<_>>(),
        )
        .unwrap();

        source
    };

    ProgramDef {
        uniform_block_defs: block_defs,
        uniform_sampler_defs: sampler_defs,
        vertex_defs,
        vertex_shader_source,
        fragment_shader_source,
    }
}

#[derive(Default)]
struct CollectUniforms {
    sampler_defs: Vec<UniformSamplerDef>,
    block_defs: Vec<UniformBlockDef>,
}

impl<'a> UniformVisitor<'a, SlView> for CollectUniforms {
    fn accept_sampler2d<S: Sample>(&mut self, path: &str, _: &Sampler2d<S>) {
        // TODO: Allow user-specified sampler texture units.
        self.sampler_defs.push(UniformSamplerDef {
            name: path.to_string(),
            ty: SamplerType::Sampler2d,
            texture_unit: self.sampler_defs.len(),
        })
    }

    fn accept_block<U: Block<SlView>>(&mut self, path: &str, _: &U) {
        // TODO: Allow user-specified uniform block locations.
        self.block_defs.push(UniformBlockDef {
            block_name: path.to_string() + "_posh_block",
            arg_name: path.to_string(),
            ty: <U::SlView as Object>::ty(),
            location: self.block_defs.len(),
        })
    }
}

#[derive(Default)]
struct CollectAttributes {
    attribute_defs: Vec<VertexAttributeDef>,
    vertex_defs: Vec<VertexDef>,
}

impl<'a> VertexVisitor<'a, SlView> for CollectAttributes {
    fn accept<V: Block<SlView>>(&mut self, path: &str, input_rate: VertexInputRate, _: &V) {
        self.attribute_defs.extend(V::vertex_attribute_defs(path));
        self.vertex_defs.push(VertexDef {
            input_rate,
            stride: std::mem::size_of::<<V::GlView as AsStd140>::Output>(),
            attributes: V::vertex_attribute_defs(path),
        })
    }
}

#[derive(Default)]
struct CollectOutputs {
    outputs: Vec<(String, Rc<Expr>)>,
}

impl FragmentVisitor<SlView> for CollectOutputs {
    fn accept(&mut self, path: &str, output: &Vec4) {
        self.outputs.push((path.to_string(), output.expr()));
    }
}
