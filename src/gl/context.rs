use std::rc::Rc;

use crevice::std140::AsStd140;
use glow::HasContext;

use crate::{
    sl::{
        transpile::transpile_to_program_def,
        transpile::{FromFragmentInput, FromVertexInput, IntoFragmentOutput, IntoVertexOutput},
        Sample, Varying,
    },
    Block, Fragment, SlView, Uniform, UniformUnion, Vertex,
};

use super::{
    raw::{self, FramebufferError},
    BufferError, BufferUsage, Caps, ContextError, Element, ElementBuffer, Framebuffer, Image,
    Program, ProgramError, Texture2d, TextureError, UniformBuffer, VertexBuffer,
};

/// The graphics context, which is used for creating GPU objects.
pub struct Context {
    raw: raw::Context,
}

impl Context {
    pub fn new(gl: glow::Context) -> Result<Self, ContextError> {
        let raw = raw::Context::new(gl)?;

        Ok(Self { raw })
    }

    pub fn gl(&self) -> &Rc<glow::Context> {
        self.raw.gl()
    }

    pub fn caps(&self) -> &Caps {
        self.raw.caps()
    }

    pub fn create_vertex_buffer<B>(
        &self,
        data: &[B::GlView],
        usage: BufferUsage,
    ) -> Result<VertexBuffer<B>, BufferError>
    where
        B: Block<SlView>,
    {
        // TODO
        let data: Vec<_> = data.iter().map(|vertex| vertex.as_std140()).collect();

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

    pub fn create_uniform_buffer<B>(
        &self,
        data: B::GlView,
        usage: BufferUsage,
    ) -> Result<UniformBuffer<B>, BufferError>
    where
        B: Block<SlView>,
    {
        let raw = self.raw.create_buffer(&[data.as_std140()], usage)?;

        Ok(UniformBuffer::from_raw(raw))
    }

    pub fn create_texture_2d<S: Sample>(
        &self,
        image: Image<'_, S>,
    ) -> Result<Texture2d<S>, TextureError> {
        let raw = self.raw.create_texture_2d(image.raw().clone())?;

        Ok(Texture2d::from_raw(raw))
    }

    pub fn create_texture_2d_with_mipmap<S: Sample>(
        &self,
        image: Image<'_, S>,
    ) -> Result<Texture2d<S>, TextureError> {
        let raw = self
            .raw
            .create_texture_2d_with_mipmap(image.raw().clone())?;

        Ok(Texture2d::from_raw(raw))
    }

    pub fn create_framebuffer<F: Fragment<SlView>>(
        &self,
        attachments: F::GlView,
    ) -> Result<Framebuffer<F>, FramebufferError> {
        Framebuffer::new(&self.raw, attachments)
    }

    pub fn create_program<U, U1, U2, V, F, W, InV, OutW, InW, OutF>(
        &self,
        vertex_shader: fn(U1, InV) -> OutW,
        fragment_shader: fn(U2, InW) -> OutF,
    ) -> Result<Program<U, V, F>, ProgramError>
    where
        U: UniformUnion<U1, U2>,
        U1: Uniform<SlView>,
        U2: Uniform<SlView>,
        V: Vertex<SlView>,
        F: Fragment<SlView>,
        W: Varying,
        InV: FromVertexInput<Vertex = V>,
        OutW: IntoVertexOutput<Varying = W>,
        InW: FromFragmentInput<Varying = W>,
        OutF: IntoFragmentOutput<Fragment = F>,
    {
        let program_def = transpile_to_program_def::<U, _, _, _, _, _, _, _, _, _>(
            vertex_shader,
            fragment_shader,
        );

        println!(
            "{}\n==================={}",
            program_def.vertex_shader_source, program_def.fragment_shader_source
        );

        let raw = self.raw.create_program(program_def)?;

        Ok(Program::unchecked_from_raw(raw))
    }

    // TODO: Program creation with consts
    /*
        pub fn create_program_with_consts<
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
            &self,
            consts: Consts,
            vertex_shader: fn(Consts, UData, VertIn) -> VertOut,
            fragment_shader: fn(Consts, UData, FragIn) -> FragOut,
        ) -> Result<Program<UData, VData, FData>, ProgramError>
        where
            Consts: ConstParams,
            UData: Uniform<SlView, SlView = UData>,
            VData: Vertex<SlView, SlView = VData>,
            FData: Fragment<SlView, SlView = FData>,
            Vary: Varying,
            VertIn: FromVertexInput<Vert = VData>,
            VertOut: IntoVertexOutput<Vary = Vary>,
            FragIn: FromFragmentInput<Vary = Vary>,
            FragOut: IntoFragmentOutput<Frag = FData>,
        {
            let program_def =
                transpile_to_program_def_with_consts(consts, vertex_shader, fragment_shader);

            println!(
                "{}\n==================={}",
                program_def.vertex_shader_source, program_def.fragment_shader_source
            );

            let raw = self.raw.create_program(program_def)?;

            Ok(Program::unchecked_from_raw(raw))
        }
    */

    // TODO: Clearing should move to some framebuffer thing.

    pub fn clear_color(&self, color: [f32; 4]) {
        let gl = self.raw.gl();

        unsafe {
            gl.clear_color(color[0], color[1], color[2], color[3]);
            gl.clear(glow::COLOR_BUFFER_BIT);
        }
    }
}
