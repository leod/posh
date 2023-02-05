use std::rc::Rc;

use crevice::std140::AsStd140;
use glow::HasContext;

use crate::{
    compile::{compile_to_program_def, compile_to_program_def_with_consts},
    sl::{
        ConstInput, FromFragmentInput, FromVertexInput, IntoFragmentOutput, IntoVertexOutput,
        Varying,
    },
    Block, FragmentInterface, Sl, UniformInterface, VertexInterface,
};

use super::{
    raw, BufferError, BufferUsage, Caps, Element, ElementBuffer, ElementOrUnit, Error, Image,
    ImageFormat, Program, ProgramError, Texture2d, TextureError, UniformBuffer, VertexArray,
    VertexArrayError, VertexBuffer,
};

/// The graphics context, which is used for creating GPU objects.
pub struct Context {
    raw: raw::Context,
}

impl Context {
    pub fn new(gl: glow::Context) -> Self {
        let raw = raw::Context::new(gl);

        Self { raw }
    }

    pub fn gl(&self) -> &Rc<glow::Context> {
        self.raw.gl()
    }

    pub fn caps(&self) -> &Caps {
        self.raw.caps()
    }

    pub fn create_vertex_buffer<V>(
        &self,
        data: &[<V::InGl as AsStd140>::Output],
        usage: BufferUsage,
    ) -> Result<VertexBuffer<V>, BufferError>
    where
        V: Block<Sl>,
    {
        // TODO: This extra allocation for converting to `V::AsStd140::Output`
        // could be eliminated if we see the need.
        let data: Vec<_> = data.iter().map(AsStd140::as_std140).collect();

        let raw = self.raw.create_buffer(&data, usage)?;

        Ok(VertexBuffer::from_raw(raw))
    }

    pub fn create_element_buffer<E>(
        &self,
        data: &[E],
        usage: BufferUsage,
    ) -> Result<ElementBuffer<E>, BufferError>
    where
        E: Element,
    {
        let raw = self.raw.create_buffer(data, usage)?;

        Ok(ElementBuffer::from_raw(raw))
    }

    pub fn create_uniform_buffer<U>(
        &self,
        uniform: U::InGl,
        usage: BufferUsage,
    ) -> Result<UniformBuffer<U>, BufferError>
    where
        U: Block<Sl>,
    {
        let raw = self.raw.create_buffer(&[uniform.as_std140()], usage)?;

        Ok(UniformBuffer::from_raw(raw))
    }

    pub fn create_vertex_array<V, E>(
        &self,
        vertex_buffers: V::InGl,
        element_source: E::Source,
    ) -> Result<VertexArray<V, E>, VertexArrayError>
    where
        V: VertexInterface<Sl>,
        E: ElementOrUnit,
    {
        VertexArray::new(&self.raw, vertex_buffers, element_source)
    }

    pub fn create_simple_vertex_array<V, E>(
        &self,
        vertices: &[<V::InGl as AsStd140>::Output],
        usage: BufferUsage,
        element_source: E::Source,
    ) -> Result<VertexArray<V, E>, Error>
    where
        V: Block<Sl>,
        E: ElementOrUnit,
    {
        let vertex_buffer = self.create_vertex_buffer(vertices, usage)?;

        Ok(VertexArray::new(&self.raw, vertex_buffer, element_source)?)
    }

    pub fn create_program<Unif, Vert, Frag, Vary, VertIn, VertOut, FragIn, FragOut>(
        &self,
        vertex_shader: fn(Unif, VertIn) -> VertOut,
        fragment_shader: fn(Unif, FragIn) -> FragOut,
    ) -> Result<Program<Unif, Vert, Frag>, ProgramError>
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
        let program_def = compile_to_program_def(vertex_shader, fragment_shader);

        println!(
            "{}\n==================={}",
            program_def.vertex_shader_source, program_def.fragment_shader_source
        );

        let raw = self.raw.create_program(program_def)?;

        Ok(Program::unchecked_from_raw(raw))
    }

    pub fn create_program_with_consts<
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
        &self,
        consts: Consts,
        vertex_shader: fn(Consts, Unif, VertIn) -> VertOut,
        fragment_shader: fn(Consts, Unif, FragIn) -> FragOut,
    ) -> Result<Program<Unif, Vert, Frag>, ProgramError>
    where
        Consts: ConstInput,
        Unif: UniformInterface<Sl, InSl = Unif>,
        Vert: VertexInterface<Sl, InSl = Vert>,
        Frag: FragmentInterface<Sl, InSl = Frag>,
        Vary: Varying,
        VertIn: FromVertexInput<Vert = Vert>,
        VertOut: IntoVertexOutput<Vary = Vary>,
        FragIn: FromFragmentInput<Vary = Vary>,
        FragOut: IntoFragmentOutput<Frag = Frag>,
    {
        let program_def =
            compile_to_program_def_with_consts(consts, vertex_shader, fragment_shader);

        println!(
            "{}\n==================={}",
            program_def.vertex_shader_source, program_def.fragment_shader_source
        );

        let raw = self.raw.create_program(program_def)?;

        Ok(Program::unchecked_from_raw(raw))
    }

    pub fn create_texture_2d<Format: ImageFormat>(
        &self,
        image: Format::Image<'_>,
    ) -> Result<Texture2d<Format>, TextureError> {
        let raw = self.raw.create_texture_2d(image.raw().clone())?;

        Ok(Texture2d::from_raw(raw))
    }

    pub fn create_texture_2d_with_mipmap<Format: ImageFormat>(
        &self,
        image: Format::Image<'_>,
    ) -> Result<Texture2d<Format>, TextureError> {
        let raw = self
            .raw
            .create_texture_2d_with_mipmap(image.raw().clone())?;

        Ok(Texture2d::from_raw(raw))
    }

    // TODO: Clearing should move to some framebuffer thing.

    pub fn clear_color(&self, color: [f32; 4]) {
        let gl = self.raw.gl();

        unsafe {
            gl.clear_color(color[0], color[1], color[2], color[3]);
            gl.clear(glow::COLOR_BUFFER_BIT);
        }
    }
}
