//! Transpile a typed program to GLSL source code.
//!
//! This is exposed only in order to make the internally generated source code
//! more transparent. It is typically not necessary to use this module.

use std::{iter::once, rc::Rc};

use crate::{
    interface::{FragmentVisitor, UniformUnion, UniformVisitor, VertexVisitor},
    Block, FsInterface, Sl, VsInterface,
};

use super::{
    codegen,
    dag::{Expr, SamplerType, Trace, Type},
    primitives::value_arg,
    program_def::{ProgramDef, UniformBlockDef, UniformSamplerDef, VertexBlockDef},
    sig::{FromFsInput, FromVsInput, VsFunc, VsSig},
    ColorSample, ColorSampler2d, ComparisonSampler2d, Derivatives, FsFunc, FsInput, FsSig,
    Interpolant, IntoFullFsOutput, IntoFullVsOutput, Object, VsInput, I32,
};

/// Transpiles a vertex shader and a fragment shader to GLSL source code.
///
/// This is used internally by `posh` in order to create
/// [`Program`](crate::gl::Program)s. It is exposed for the purpose of
/// inspecting generated shader source code.
pub fn transpile_to_program_def<U, VSig, VFunc, FSig, FFunc>(
    vertex_shader: VFunc,
    fragment_shader: FFunc,
) -> ProgramDef
where
    U: UniformUnion<VSig::U, FSig::U>,
    VSig: VsSig<C = ()>,
    VFunc: VsFunc<VSig>,
    FSig: FsSig<C = (), W = VSig::W>,
    FFunc: FsFunc<FSig>,
{
    transpile_to_program_def_with_consts::<U, VSig, VFunc, FSig, FFunc>(
        &(),
        vertex_shader,
        fragment_shader,
    )
}

/// Transpiles a vertex shader and a fragment shader with constant input to GLSL
/// source code.
///
/// See also [`transpile_to_program_def`].
pub fn transpile_to_program_def_with_consts<U, VSig, VFunc, FSig, FFunc>(
    consts: &VSig::C,
    vertex_shader: VFunc,
    fragment_shader: FFunc,
) -> ProgramDef
where
    U: UniformUnion<VSig::U, FSig::U>,
    VSig: VsSig,
    VFunc: VsFunc<VSig>,
    FSig: FsSig<C = VSig::C, W = VSig::W>,
    FFunc: FsFunc<FSig>,
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
        let input = || VsInput {
            vertex: <VSig as VsSig>::V::shader_input("vertex_input"),
            vertex_id: value_arg::<I32>("gl_VertexID").as_u32(),
            instance_id: value_arg::<I32>("gl_InstanceID").as_u32(),
            _private: (),
        };
        let output = vertex_shader
            .call(consts, uniforms.lhs(), FromVsInput::from_vs_input(input()))
            .into_full_vs_output();

        let varying_outputs = output.interpolant.shader_outputs("vertex_output");
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
        let exprs = once(("gl_Position", output.clip_pos.expr()))
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
        let input = FsInput {
            interpolant: <VSig as VsSig>::W::shader_input("vertex_output"),
            fragment_coord: value_arg("gl_FragCoord"),
            front_facing: value_arg("gl_FrontFacing"),
            point_coord: value_arg("gl_PointCoord"),
            derivatives: Derivatives(()),
        };
        let output = fragment_shader
            .call(consts, uniforms.rhs(), FromFsInput::from_fs_input(input))
            .into_full_fs_output();

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

    Trace::clear_cache();

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
