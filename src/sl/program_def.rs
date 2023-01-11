use std::{iter::once, marker::PhantomData, rc::Rc};

use crate::{
    dag::{Expr, Type},
    gen::glsl::{self, UniformBlockDef},
    gl::untyped::VertexInfo,
    interface::{FragmentInterfaceVisitor, ResourceInterfaceVisitor, VertexInterfaceVisitor},
    sl::Object,
    FragmentInterface, Numeric, ResourceInterface, Sl, Uniform, Vertex, VertexInputRate,
    VertexInterface,
};

use super::{primitives::value_arg, Bool, Sampler2d, Varying, Vec2, Vec4, F32, U32};

#[derive(Debug, Clone)]
struct Private;

#[derive(Debug, Clone)]
pub struct VertexInput<V> {
    pub vertex: V,
    pub vertex_id: U32,
    pub instance_id: U32,
    _private: Private,
}

#[derive(Debug, Clone)]
pub struct VertexOutput<W> {
    pub position: Vec4<f32>,
    pub varying: W,
    pub point_size: Option<F32>,
}

impl<W> VertexOutput<W> {
    pub fn new(position: Vec4<f32>, varying: W) -> Self {
        Self {
            position,
            varying,
            point_size: None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct FragmentInput<W> {
    pub varying: W,
    pub fragment_coord: Vec4<f32>,
    pub front_facing: Bool,
    pub point_coord: Vec2<f32>,
    _private: Private,
}

#[derive(Debug, Clone)]
pub struct FragmentOutput<F> {
    pub fragment: F,
    pub fragment_depth: Option<F32>,
}

impl<F> FragmentOutput<F> {
    pub fn new(fragment: F) -> Self {
        FragmentOutput {
            fragment,
            fragment_depth: None,
        }
    }
}

pub struct ProgramDef<R, A, F> {
    pub uniform_block_defs: Vec<UniformBlockDef>,
    pub vertex_infos: Vec<VertexInfo>,
    pub vertex_shader_source: String,
    pub fragment_shader_source: String,
    _phantom: PhantomData<(R, A, F)>,
}

impl<R, V, F> ProgramDef<R, V, F>
where
    R: ResourceInterface<Sl, InSl = R>,
    V: VertexInterface<Sl, InSl = V>,
    F: FragmentInterface<Sl, InSl = F>,
{
    pub fn new<W>(
        vertex_shader: fn(R, VertexInput<V>) -> VertexOutput<W>,
        fragment_shader: fn(R, FragmentInput<W>) -> FragmentOutput<F>,
    ) -> Self
    where
        W: Varying,
    {
        let resources = R::shader_input("resource");

        let uniform_block_defs = {
            let mut resource_visitor = ResourceVisitor::default();
            resources.visit("resource", &mut resource_visitor);

            resource_visitor.uniform_block_defs
        };

        let (vertex_infos, varying_outputs, vertex_source) = {
            let input = || VertexInput {
                vertex: V::shader_input("vertex_input"),
                vertex_id: value_arg("gl_VertexID"),
                instance_id: value_arg("gl_InstanceID"),
                _private: Private,
            };
            let output = vertex_shader(resources, input());

            let varying_outputs = output.varying.shader_outputs("vertex_output");
            let (vertex_attributes, vertex_infos) = {
                let mut visitor = VertexVisitor::default();
                input().vertex.visit("vertex_input", &mut visitor);

                (visitor.attributes, visitor.infos)
            };

            let attributes = vertex_attributes
                .into_iter()
                .map(|(name, ty)| ("in".to_string(), name, ty))
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
                &uniform_block_defs,
                attributes,
                &exprs.collect::<Vec<_>>(),
            )
            .unwrap();

            (vertex_infos, varying_outputs, source)
        };

        let resources = R::shader_input("resource");

        let fragment_source =
            {
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

                let attributes =
                    varying_outputs
                        .iter()
                        .map(|(name, expr)| {
                            // TODO: Interpolation type.
                            ("in".to_string(), name.clone(), expr.ty())
                        })
                        .chain(fragment_visitor.outputs.iter().enumerate().map(
                            |(i, (name, expr))| {
                                (
                                    format!("layout(location = {i}) out"),
                                    name.clone(),
                                    expr.ty(),
                                )
                            },
                        ));

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
                    &uniform_block_defs,
                    attributes,
                    &exprs.collect::<Vec<_>>(),
                )
                .unwrap();

                source
            };

        Self {
            uniform_block_defs,
            vertex_infos,
            vertex_shader_source: vertex_source,
            fragment_shader_source: fragment_source,
            _phantom: PhantomData,
        }
    }
}

#[derive(Default)]
struct ResourceVisitor {
    uniform_block_defs: Vec<UniformBlockDef>,
}

impl ResourceInterfaceVisitor<Sl> for ResourceVisitor {
    fn accept_sampler2d<T: Numeric>(&mut self, path: &str, sampler: &Sampler2d<T>) {
        todo!()
    }

    fn accept_uniform<U: Uniform<Sl>>(&mut self, path: &str, _: &U) {
        // TODO: Allow user-specified uniform block locations.
        self.uniform_block_defs.push(UniformBlockDef {
            block_name: path.to_string() + "_posh_block",
            arg_name: path.to_string(),
            ty: <U::InSl as Object>::ty(),
            location: self.uniform_block_defs.len(),
        })
    }
}

#[derive(Default)]
struct VertexVisitor {
    attributes: Vec<(String, Type)>,
    infos: Vec<VertexInfo>,
}

impl VertexInterfaceVisitor<Sl> for VertexVisitor {
    fn accept<V: Vertex<Sl>>(&mut self, path: &str, input_rate: VertexInputRate, _: &V) {
        for attribute in V::attributes(path) {
            self.attributes.push((attribute.name, attribute.ty));
        }

        self.infos.push(VertexInfo {
            input_rate,
            stride: std::mem::size_of::<V::Pod>(),
            attributes: V::attributes(path),
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
