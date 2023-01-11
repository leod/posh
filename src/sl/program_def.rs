use std::marker::PhantomData;

use crate::{
    dag::Type,
    gen::glsl::{self, UniformBlockDef},
    interface::{ResourceInterfaceVisitor, VertexInterfaceVisitor},
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
    private: Private,
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
    private: Private,
}

#[derive(Debug, Clone)]
pub struct FragmentOutput<F> {
    pub fragment: F,
    pub fragment_depth: Option<F32>,
}

pub struct ProgramDef<R, A, F> {
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

        let mut resource_visitor = ResourceVisitor::default();
        resources.visit("resource", &mut resource_visitor);

        let vertex_source = {
            let input = VertexInput {
                vertex: V::shader_input("vertex"),
                vertex_id: value_arg("gl_VertexID"),
                instance_id: value_arg("gl_InstanceID"),
                private: Private,
            };

            let varying_attributes = W::attributes("output");

            let attributes = {
                let mut visitor = VertexVisitor::default();
                input.vertex.visit(&mut visitor);

                visitor
                    .attributes
                    .into_iter()
                    .map(|(name, ty)| ("in".to_string(), name, ty))
            }
            .chain(
                // TODO: Interpolation type.
                varying_attributes
                    .iter()
                    .cloned()
                    .map(|(name, ty)| ("out".to_string(), name, ty)),
            );

            let output = vertex_shader(resources, input);
            let mut exprs = vec![("gl_Position", output.position.expr())];
            exprs.extend(
                varying_attributes
                    .iter()
                    .zip(output.varying.shader_outputs())
                    .map(|((name, _), expr)| (name.as_str(), expr)),
            );

            let mut source = String::new();
            glsl::write_shader_stage(
                &mut source,
                &resource_visitor.uniform_block_defs,
                attributes,
                &exprs,
            )
            .unwrap();

            source
        };

        println!("{vertex_source}");

        Self {
            _phantom: PhantomData,
        }
    }
}

#[derive(Default)]
struct VertexVisitor {
    attributes: Vec<(String, Type)>,
}

impl VertexInterfaceVisitor<Sl> for VertexVisitor {
    fn accept<V: Vertex<Sl>>(&mut self, path: &str, _: VertexInputRate, _: &V) {
        for attribute in V::attributes(path) {
            self.attributes.push((attribute.name, attribute.ty));
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
        self.uniform_block_defs.push(UniformBlockDef {
            block_name: path.to_string() + "_posh_block",
            arg_name: path.to_string(),
            ty: <U::InSl as Object>::ty(),
        })
    }
}
