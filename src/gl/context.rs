use std::rc::Rc;

use crevice::std140::AsStd140;
use glow::HasContext;

use crate::{
    build::{build_program_def, build_program_def_with_consts},
    sl::{
        ConstInput, FromFragmentInput, FromVertexInput, IntoFragmentOutput, IntoVertexOutput,
        Varying,
    },
    Block, FragmentInterface, ResourceInterface, Sl, VertexInterface,
};

use super::{
    untyped, BufferUsage, CreateBufferError, CreateError, CreateProgramError,
    CreateVertexArrayError, Element, ElementBuffer, ElementOrUnit, Program, UniformBuffer,
    VertexArray, VertexBuffer,
};

/// The graphics context, which is used for creating GPU objects.
pub struct Context {
    pub(crate) untyped: untyped::Context,
}

impl Context {
    pub fn new(gl: glow::Context) -> Self {
        let untyped = untyped::Context::new(gl);

        Self { untyped }
    }

    pub fn gl(&self) -> &Rc<glow::Context> {
        self.untyped.gl()
    }

    pub fn create_vertex_buffer<V>(
        &self,
        data: &[<V::InGl as AsStd140>::Output],
        usage: BufferUsage,
    ) -> Result<VertexBuffer<V>, CreateBufferError>
    where
        V: Block<Sl>,
    {
        // TODO: This extra allocation for converting to `V::AsStd140::Output`
        // could be eliminated if we see the need.
        let data: Vec<_> = data.iter().map(AsStd140::as_std140).collect();

        let untyped = self.untyped.create_buffer(&data, usage)?;

        Ok(VertexBuffer::from_untyped(untyped))
    }

    pub fn create_element_buffer<E>(
        &self,
        data: &[E],
        usage: BufferUsage,
    ) -> Result<ElementBuffer<E>, CreateBufferError>
    where
        E: Element,
    {
        let untyped = self.untyped.create_buffer(data, usage)?;

        Ok(ElementBuffer::from_untyped(untyped))
    }

    pub fn create_uniform_buffer<U>(
        &self,
        uniform: U::InGl,
        usage: BufferUsage,
    ) -> Result<UniformBuffer<U>, CreateBufferError>
    where
        U: Block<Sl>,
    {
        let untyped = self.untyped.create_buffer(&[uniform.as_std140()], usage)?;

        Ok(UniformBuffer::from_untyped(untyped))
    }

    pub fn create_vertex_array<V, E>(
        &self,
        vertex_buffers: V::InGl,
        element_source: E::Source,
    ) -> Result<VertexArray<V, E>, CreateVertexArrayError>
    where
        V: VertexInterface<Sl>,
        E: ElementOrUnit,
    {
        VertexArray::new(&self.untyped, vertex_buffers, element_source)
    }

    pub fn create_simple_vertex_array<V, E>(
        &self,
        vertices: &[<V::InGl as AsStd140>::Output],
        usage: BufferUsage,
        element_source: E::Source,
    ) -> Result<VertexArray<V, E>, CreateError>
    where
        V: Block<Sl>,
        E: ElementOrUnit,
    {
        let vertex_buffer = self.create_vertex_buffer(vertices, usage)?;

        Ok(VertexArray::new(
            &self.untyped,
            vertex_buffer,
            element_source,
        )?)
    }

    pub fn create_program<Res, Vert, Frag, Vary, VertIn, VertOut, FragIn, FragOut>(
        &self,
        vertex_shader: fn(Res, VertIn) -> VertOut,
        fragment_shader: fn(Res, FragIn) -> FragOut,
    ) -> Result<Program<Res, Vert, Frag>, CreateProgramError>
    where
        Res: ResourceInterface<Sl>,
        Vert: VertexInterface<Sl>,
        Frag: FragmentInterface<Sl>,
        Vary: Varying,
        VertIn: FromVertexInput<Vert = Vert>,
        VertOut: IntoVertexOutput<Vary = Vary>,
        FragIn: FromFragmentInput<Vary = Vary>,
        FragOut: IntoFragmentOutput<Frag = Frag>,
    {
        let program_def = build_program_def(vertex_shader, fragment_shader);

        println!(
            "{}\n==================={}",
            program_def.vertex_shader_source, program_def.fragment_shader_source
        );

        let untyped = self.untyped.create_program(program_def)?;

        Ok(Program::unchecked_from_untyped(untyped))
    }

    pub fn create_program_with_consts<
        Consts,
        Res,
        Vert,
        Frag,
        Vary,
        VertIn,
        VertOut,
        FragIn,
        FragOut,
    >(
        &self,
        consts: Consts,
        vertex_shader: fn(Consts, Res, VertIn) -> VertOut,
        fragment_shader: fn(Consts, Res, FragIn) -> FragOut,
    ) -> Result<Program<Res, Vert, Frag>, CreateProgramError>
    where
        Consts: ConstInput,
        Res: ResourceInterface<Sl, InSl = Res>,
        Vert: VertexInterface<Sl, InSl = Vert>,
        Frag: FragmentInterface<Sl, InSl = Frag>,
        Vary: Varying,
        VertIn: FromVertexInput<Vert = Vert>,
        VertOut: IntoVertexOutput<Vary = Vary>,
        FragIn: FromFragmentInput<Vary = Vary>,
        FragOut: IntoFragmentOutput<Frag = Frag>,
    {
        let program_def = build_program_def_with_consts(consts, vertex_shader, fragment_shader);

        println!(
            "{}\n==================={}",
            program_def.vertex_shader_source, program_def.fragment_shader_source
        );

        let untyped = self.untyped.create_program(program_def)?;

        Ok(Program::unchecked_from_untyped(untyped))
    }

    // TODO: Clearing should move to some framebuffer thing.

    pub fn clear_color(&self, color: [f32; 4]) {
        let gl = self.untyped.gl();

        unsafe {
            gl.clear_color(color[0], color[1], color[2], color[3]);
            gl.clear(glow::COLOR_BUFFER_BIT);
        }
    }
}
