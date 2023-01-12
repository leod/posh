use std::{iter::once, rc::Rc};

use crate::{
    dag::Expr,
    gen::glsl,
    interface::{FragmentInterfaceVisitor, ResourceInterfaceVisitor, VertexInterfaceVisitor},
    program_def::{
        ProgramDef, SamplerDef, UniformDef, VertexAttributeDef, VertexDef, VertexInputRate,
    },
    sl::{FragmentInput, FragmentOutput, Object, Private, VertexInput, VertexOutput},
    FragmentInterface, Numeric, ResourceInterface, Sl, Uniform, Vertex, VertexInterface,
};

use crate::sl::{primitives::value_arg, Sampler2d, Varying, Vec4};

/// Compiles a vertex shader and a fragment shader into a type-erased program
/// definition.
///
/// This is used internally by `posh` in order to create
/// [`Program`](crate::gl::Program)s. It is exposed for the purpose of
/// inspecting generated shader source code.
pub fn build_program_def<R, V, F, W>(
    vertex_shader: fn(R, VertexInput<V>) -> VertexOutput<W>,
    fragment_shader: fn(R, FragmentInput<W>) -> FragmentOutput<F>,
) -> ProgramDef
where
    R: ResourceInterface<Sl, InSl = R>,
    V: VertexInterface<Sl, InSl = V>,
    F: FragmentInterface<Sl, InSl = F>,
    W: Varying,
{
    let resources = R::shader_input("resource");

    let (uniform_defs, sampler_defs) = {
        let mut resource_visitor = ResourceVisitor::default();
        resources.visit("resource", &mut resource_visitor);

        (resource_visitor.uniform_defs, resource_visitor.sampler_defs)
    };

    let (vertex_defs, varying_outputs, vertex_shader_source) = {
        let input = || VertexInput {
            vertex: V::shader_input("vertex_input"),
            vertex_id: value_arg("gl_VertexID"),
            instance_id: value_arg("gl_InstanceID"),
            _private: Private,
        };
        let output = vertex_shader(resources, input());

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
            &uniform_defs,
            attributes,
            &exprs.collect::<Vec<_>>(),
        )
        .unwrap();

        (vertex_defs, varying_outputs, source)
    };

    let resources = R::shader_input("resource");

    let fragment_shader_source = {
        let input = FragmentInput {
            varying: W::shader_input("vertex_output"),
            fragment_coord: value_arg("gl_FragCoord"),
            front_facing: value_arg("gl_FrontFacing"),
            point_coord: value_arg("gl_PointCoord"),
            _private: Private,
        };
        let output = fragment_shader(resources, input);

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
            &uniform_defs,
            attributes,
            &exprs.collect::<Vec<_>>(),
        )
        .unwrap();

        source
    };

    ProgramDef {
        uniform_defs,
        sampler_defs,
        vertex_defs,
        vertex_shader_source,
        fragment_shader_source,
    }
}

#[derive(Default)]
struct ResourceVisitor {
    uniform_defs: Vec<UniformDef>,
    sampler_defs: Vec<SamplerDef>,
}

impl ResourceInterfaceVisitor<Sl> for ResourceVisitor {
    fn accept_sampler2d<T: Numeric>(&mut self, path: &str, sampler: &Sampler2d<T>) {
        todo!()
    }

    fn accept_uniform<U: Uniform<Sl>>(&mut self, path: &str, _: &U) {
        // TODO: Allow user-specified uniform block locations.
        self.uniform_defs.push(UniformDef {
            block_name: path.to_string() + "_posh_block",
            arg_name: path.to_string(),
            ty: <U::InSl as Object>::ty(),
            location: self.uniform_defs.len(),
        })
    }
}

#[derive(Default)]
struct VertexVisitor {
    attribute_defs: Vec<VertexAttributeDef>,
    vertex_defs: Vec<VertexDef>,
}

impl VertexInterfaceVisitor<Sl> for VertexVisitor {
    fn accept<V: Vertex<Sl>>(&mut self, path: &str, input_rate: VertexInputRate, _: &V) {
        self.attribute_defs.extend(V::attribute_defs(path));
        self.vertex_defs.push(VertexDef {
            input_rate,
            stride: std::mem::size_of::<V::Pod>(),
            attributes: V::attribute_defs(path),
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
