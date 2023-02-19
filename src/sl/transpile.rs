//! Transpile a typed program to GLSL source code.
//!
//! This is exposed only in order to make the internally generated source code
//! more transparent. It is typically not necessary to use this module.

use std::{iter::once, rc::Rc};

use crevice::std140::AsStd140;

use crate::{
    interface::{FragmentDataVisitor, UniformDataVisitor, VertexDataVisitor},
    Block, FragmentData, Logical, UniformData, VertexData,
};

use super::{
    codegen,
    dag::{Expr, SamplerType, Type},
    primitives::value_arg,
    program_def::{
        ProgramDef, UniformBlockDef, UniformSamplerDef, VertexAttributeDef, VertexDef,
        VertexInputRate,
    },
    ConstParams, FragmentInput, FragmentOutput, Object, Private, Sampler2d, Varying, VaryingOutput,
    Vec4, VertexInput, VertexOutput,
};

/// Types that can be used as vertex input for a vertex shader.
pub trait FromVertexInput {
    type Vert: VertexData<Logical>;

    fn from(input: VertexInput<Self::Vert>) -> Self;
}

impl<VData: VertexData<Logical>> FromVertexInput for VertexInput<VData> {
    type Vert = VData;

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

impl IntoVertexOutput for Vec4 {
    type Vary = ();

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

/// Transpiles a vertex shader and a fragment shader to GLSL source code.
///
/// This is used internally by `posh` in order to create
/// [`Program`](crate::gl::Program)s. It is exposed for the purpose of
/// inspecting generated shader source code.
pub fn transpile_to_program_def<UData, VData, FData, Vary, VertIn, VertOut, FragIn, FragOut>(
    vertex_shader: fn(UData, VertIn) -> VertOut,
    fragment_shader: fn(UData, FragIn) -> FragOut,
) -> ProgramDef
where
    UData: UniformData<Logical>,
    VData: VertexData<Logical>,
    FData: FragmentData<Logical>,
    Vary: Varying,
    VertIn: FromVertexInput<Vert = VData>,
    VertOut: IntoVertexOutput<Vary = Vary>,
    FragIn: FromFragmentInput<Vary = Vary>,
    FragOut: IntoFragmentOutput<Frag = FData>,
{
    transpile_to_program_def_with_consts_impl(
        (),
        |(), uniforms, input| vertex_shader(uniforms, input),
        |(), uniforms, input| fragment_shader(uniforms, input),
    )
}

/// Transpiles a vertex shader and a fragment shader with constant input to GLSL
/// source code.
///
/// See also [`transpile_to_program_def`].
pub fn transpile_to_program_def_with_consts<
    Consts,
    UData,
    VData,
    FData,
    Vary,
    VertIn,
    VertOut,
    FragIn,
    FragOut,
>(
    consts: Consts,
    vertex_shader: fn(Consts, UData, VertIn) -> VertOut,
    fragment_shader: fn(Consts, UData, FragIn) -> FragOut,
) -> ProgramDef
where
    Consts: ConstParams,
    UData: UniformData<Logical>,
    VData: VertexData<Logical>,
    FData: FragmentData<Logical>,
    Vary: Varying,
    VertIn: FromVertexInput<Vert = VData>,
    VertOut: IntoVertexOutput<Vary = Vary>,
    FragIn: FromFragmentInput<Vary = Vary>,
    FragOut: IntoFragmentOutput<Frag = FData>,
{
    transpile_to_program_def_with_consts_impl(consts, vertex_shader, fragment_shader)
}

fn transpile_to_program_def_with_consts_impl<
    Consts,
    UData,
    VData,
    FData,
    Vary,
    VertIn,
    VertOut,
    FragIn,
    FragOut,
>(
    consts: Consts,
    vertex_shader: impl FnOnce(Consts, UData, VertIn) -> VertOut,
    fragment_shader: impl FnOnce(Consts, UData, FragIn) -> FragOut,
) -> ProgramDef
where
    Consts: ConstParams,
    UData: UniformData<Logical>,
    VData: VertexData<Logical>,
    FData: FragmentData<Logical>,
    Vary: Varying,
    VertIn: FromVertexInput<Vert = VData>,
    VertOut: IntoVertexOutput<Vary = Vary>,
    FragIn: FromFragmentInput<Vary = Vary>,
    FragOut: IntoFragmentOutput<Frag = FData>,
{
    let uniforms = UData::shader_input("uniforms");

    let (block_defs, sampler_defs) = {
        let mut uniform_visitor = UniformVisitor::default();
        uniforms.visit("uniforms", &mut uniform_visitor);

        (uniform_visitor.block_defs, uniform_visitor.sampler_defs)
    };

    let (vertex_defs, varying_outputs, vertex_shader_source) = {
        let input = || VertexInput {
            vertex: VData::shader_input("vertex_input"),
            vertex_id: value_arg("gl_VertexID"),
            instance_id: value_arg("gl_InstanceID"),
            _private: Private,
        };
        let output = vertex_shader(consts, uniforms, VertIn::from(input())).into();

        let varying_outputs = output.varying.shader_outputs("vertex_output");
        let (vertex_attributes, vertex_defs) = {
            let mut visitor = VertexVisitor::default();
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

    let uniforms = UData::shader_input("uniforms");

    let fragment_shader_source = {
        let input = FragmentInput {
            varying: Vary::shader_input("vertex_output"),
            fragment_coord: value_arg("gl_FragCoord"),
            front_facing: value_arg("gl_FrontFacing"),
            point_coord: value_arg("gl_PointCoord"),
            _private: Private,
        };
        let output = fragment_shader(consts, uniforms, FragIn::from(input)).into();

        let mut fragment_visitor = FragmentVisitor::default();
        output
            .fragment
            .visit("fragment_output", &mut fragment_visitor);

        let attributes = varying_outputs
            .iter()
            .map(|(name, expr)| {
                // TODO: Interpolation type.
                ("in".to_string(), name.clone(), expr.ty())
            })
            .chain(
                fragment_visitor
                    .outputs
                    .iter()
                    .enumerate()
                    .map(|(i, (name, expr))| {
                        (
                            format!("layout(location = {i}) out"),
                            name.clone(),
                            expr.ty(),
                        )
                    }),
            );

        let exprs = fragment_visitor
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
struct UniformVisitor {
    sampler_defs: Vec<UniformSamplerDef>,
    block_defs: Vec<UniformBlockDef>,
}

impl<'a> UniformDataVisitor<'a, Logical> for UniformVisitor {
    fn accept_sampler2d(&mut self, path: &str, _: &Sampler2d) {
        // TODO: Allow user-specified sampler texture units.
        self.sampler_defs.push(UniformSamplerDef {
            name: path.to_string(),
            ty: SamplerType::Sampler2d,
            texture_unit: self.sampler_defs.len(),
        })
    }

    fn accept_block<U: Block<Logical>>(&mut self, path: &str, _: &U) {
        // TODO: Allow user-specified uniform block locations.
        self.block_defs.push(UniformBlockDef {
            block_name: path.to_string() + "_posh_block",
            arg_name: path.to_string(),
            ty: <U::Logical as Object>::ty(),
            location: self.block_defs.len(),
        })
    }
}

#[derive(Default)]
struct VertexVisitor {
    attribute_defs: Vec<VertexAttributeDef>,
    vertex_defs: Vec<VertexDef>,
}

impl<'a> VertexDataVisitor<'a, Logical> for VertexVisitor {
    fn accept<V: Block<Logical>>(&mut self, path: &str, input_rate: VertexInputRate, _: &V) {
        self.attribute_defs.extend(V::vertex_attribute_defs(path));
        self.vertex_defs.push(VertexDef {
            input_rate,
            stride: std::mem::size_of::<<V::Physical as AsStd140>::Output>(),
            attributes: V::vertex_attribute_defs(path),
        })
    }
}

#[derive(Default)]
struct FragmentVisitor {
    outputs: Vec<(String, Rc<Expr>)>,
}

impl FragmentDataVisitor<Logical> for FragmentVisitor {
    fn accept(&mut self, path: &str, output: &Vec4) {
        self.outputs.push((path.to_string(), output.expr()));
    }
}
