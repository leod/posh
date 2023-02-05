use std::{iter::once, rc::Rc};

use crevice::std140::AsStd140;

use crate::{
    codegen::glsl,
    dag::{Expr, SamplerType},
    interface::{FragmentInterfaceVisitor, UniformInterfaceVisitor, VertexInterfaceVisitor},
    program_def::{
        ProgramDef, UniformBlockDef, UniformSamplerDef, VertexAttributeDef, VertexDef,
        VertexInputRate,
    },
    sl::{
        ConstInput, FragmentInput, FromFragmentInput, FromVertexInput, IntoFragmentOutput,
        IntoVertexOutput, Object, Private, Sample, VertexInput,
    },
    Block, FragmentInterface, Numeric, Sl, UniformInterface, VertexInterface,
};

use crate::sl::{primitives::value_arg, Sampler2d, Varying, Vec4};

/// Compiles a vertex shader and a fragment shader into a type-erased program
/// definition.
///
/// This is used internally by `posh` in order to create
/// [`Program`](crate::gl::Program)s. It is exposed for the purpose of
/// inspecting generated shader source code.
pub fn compile_to_program_def<Unif, Vert, Frag, Vary, VertIn, VertOut, FragIn, FragOut>(
    vertex_shader: fn(Unif, VertIn) -> VertOut,
    fragment_shader: fn(Unif, FragIn) -> FragOut,
) -> ProgramDef
where
    Unif: UniformInterface<Sl>,
    Vert: VertexInterface<Sl>,
    Frag: FragmentInterface<Sl>,
    Vary: Varying,
    VertIn: FromVertexInput<Vert = Vert>,
    VertOut: IntoVertexOutput<Vary = Vary>,
    FragIn: FromFragmentInput<Vary = Vary>,
    FragOut: IntoFragmentOutput<Frag = Frag>,
{
    compile_to_program_def_with_consts_impl(
        (),
        |(), uniforms, input| vertex_shader(uniforms, input),
        |(), uniforms, input| fragment_shader(uniforms, input),
    )
}

/// Compiles a vertex shader and a fragment shader with constant input.
///
/// See also [`compile_to_program_def`].
pub fn compile_to_program_def_with_consts<
    Consts,
    Unif,
    Vert,
    Frag,
    Vary,
    VertIn,
    VertOut,
    FragIn,
    FragOut,
>(
    consts: Consts,
    vertex_shader: fn(Consts, Unif, VertIn) -> VertOut,
    fragment_shader: fn(Consts, Unif, FragIn) -> FragOut,
) -> ProgramDef
where
    Consts: ConstInput,
    Unif: UniformInterface<Sl>,
    Vert: VertexInterface<Sl>,
    Frag: FragmentInterface<Sl>,
    Vary: Varying,
    VertIn: FromVertexInput<Vert = Vert>,
    VertOut: IntoVertexOutput<Vary = Vary>,
    FragIn: FromFragmentInput<Vary = Vary>,
    FragOut: IntoFragmentOutput<Frag = Frag>,
{
    compile_to_program_def_with_consts_impl(consts, vertex_shader, fragment_shader)
}

fn compile_to_program_def_with_consts_impl<
    Consts,
    Unif,
    Vert,
    Frag,
    Vary,
    VertIn,
    VertOut,
    FragIn,
    FragOut,
>(
    consts: Consts,
    vertex_shader: impl FnOnce(Consts, Unif, VertIn) -> VertOut,
    fragment_shader: impl FnOnce(Consts, Unif, FragIn) -> FragOut,
) -> ProgramDef
where
    Consts: ConstInput,
    Unif: UniformInterface<Sl>,
    Vert: VertexInterface<Sl>,
    Frag: FragmentInterface<Sl>,
    Vary: Varying,
    VertIn: FromVertexInput<Vert = Vert>,
    VertOut: IntoVertexOutput<Vary = Vary>,
    FragIn: FromFragmentInput<Vary = Vary>,
    FragOut: IntoFragmentOutput<Frag = Frag>,
{
    let uniforms = Unif::shader_input("uniforms");

    let (block_defs, sampler_defs) = {
        let mut uniform_visitor = UniformVisitor::default();
        uniforms.visit("uniforms", &mut uniform_visitor);

        (uniform_visitor.block_defs, uniform_visitor.sampler_defs)
    };

    let (vertex_defs, varying_outputs, vertex_shader_source) = {
        let input = || VertexInput {
            vertex: Vert::shader_input("vertex_input"),
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
            .map(|attribute_def| ("in".to_string(), attribute_def.name, attribute_def.ty))
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
        glsl::write_shader_stage(
            &mut source,
            &block_defs,
            &sampler_defs,
            attributes,
            &exprs.collect::<Vec<_>>(),
        )
        .unwrap();

        (vertex_defs, varying_outputs, source)
    };

    let uniforms = Unif::shader_input("uniforms");

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
        glsl::write_shader_stage(
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

impl<'a> UniformInterfaceVisitor<'a, Sl> for UniformVisitor {
    fn accept_sampler2d<S: Sample>(&mut self, path: &str, _: &Sampler2d<S>) {
        // TODO: Allow user-specified sampler texture units.
        self.sampler_defs.push(UniformSamplerDef {
            name: path.to_string(),
            ty: SamplerType::Sampler2d {
                dimension: S::NUM_COMPONENTS,
                ty: <S::Component as Numeric>::NUMERIC_TYPE,
            },
            texture_unit: self.sampler_defs.len(),
        })
    }

    fn accept_block<U: Block<Sl>>(&mut self, path: &str, _: &U) {
        // TODO: Allow user-specified uniform block locations.
        self.block_defs.push(UniformBlockDef {
            block_name: path.to_string() + "_posh_block",
            arg_name: path.to_string(),
            ty: <U::InSl as Object>::ty(),
            location: self.block_defs.len(),
        })
    }
}

#[derive(Default)]
struct VertexVisitor {
    attribute_defs: Vec<VertexAttributeDef>,
    vertex_defs: Vec<VertexDef>,
}

impl<'a> VertexInterfaceVisitor<'a, Sl> for VertexVisitor {
    fn accept<V: Block<Sl>>(&mut self, path: &str, input_rate: VertexInputRate, _: &V) {
        self.attribute_defs.extend(V::vertex_attribute_defs(path));
        self.vertex_defs.push(VertexDef {
            input_rate,
            stride: std::mem::size_of::<<V::InGl as AsStd140>::Output>(),
            attributes: V::vertex_attribute_defs(path),
        })
    }
}

#[derive(Default)]
struct FragmentVisitor {
    outputs: Vec<(String, Rc<Expr>)>,
}

impl FragmentInterfaceVisitor<Sl> for FragmentVisitor {
    fn accept(&mut self, path: &str, output: &Vec4<f32>) {
        self.outputs.push((path.to_string(), output.expr()));
    }
}
