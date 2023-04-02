use std::rc::Rc;

use crevice::std140::AsStd140;

use crate::{
    sl::{
        transpile::transpile_to_program_def,
        transpile::{FromFragmentInput, FromVertexInput, IntoFragmentOutput, IntoVertexOutput},
        ColorSample, Varying,
    },
    Block, Fragment, Sl, Uniform, UniformUnion, Vertex,
};

use super::{
    raw, BufferError, BufferUsage, Caps, ColorImage, ColorTexture2d, ContextError, DepthImage,
    DepthTexture2d, Element, ElementBuffer, Program, ProgramError, TextureError, UniformBuffer,
    VertexBuffer,
};

/// The graphics context, which is used for creating GPU objects.
#[derive(Clone)]
pub struct Context {
    raw: Rc<raw::Context>,
}

impl Context {
    pub fn new(gl: glow::Context) -> Result<Self, ContextError> {
        let raw = raw::Context::new(gl)?;

        Ok(Self { raw: Rc::new(raw) })
    }

    pub fn caps(&self) -> &Caps {
        self.raw.caps()
    }

    pub fn create_vertex_buffer<B>(
        &self,
        data: &[B::Gl],
        usage: BufferUsage,
    ) -> Result<VertexBuffer<B>, BufferError>
    where
        B: Block<Sl>,
    {
        // FIXME
        let data: Vec<_> = data.iter().map(AsStd140::as_std140).collect();

        let raw = self.raw.create_buffer(&data, glow::ARRAY_BUFFER, usage)?;

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
        let raw = self
            .raw
            .create_buffer(data, glow::ELEMENT_ARRAY_BUFFER, usage)?;

        Ok(ElementBuffer::from_raw(raw))
    }

    pub fn create_uniform_buffer<B>(
        &self,
        data: B::Gl,
        usage: BufferUsage,
    ) -> Result<UniformBuffer<B>, BufferError>
    where
        B: Block<Sl>,
    {
        let raw = self
            .raw
            .create_buffer(&[data.as_std140()], glow::UNIFORM_BUFFER, usage)?;

        Ok(UniformBuffer::from_raw(raw))
    }

    pub fn create_color_texture_2d<S: ColorSample>(
        &self,
        image: ColorImage<'_, S>,
    ) -> Result<ColorTexture2d<S>, TextureError> {
        let raw = self.raw.create_texture_2d(image.raw().clone())?;

        Ok(ColorTexture2d::from_raw(raw))
    }

    pub fn create_color_texture_2d_with_mipmap<S: ColorSample>(
        &self,
        image: ColorImage<'_, S>,
    ) -> Result<ColorTexture2d<S>, TextureError> {
        let raw = self
            .raw
            .create_texture_2d_with_mipmap(image.raw().clone())?;

        Ok(ColorTexture2d::from_raw(raw))
    }

    pub fn create_depth_texture_2d(
        &self,
        image: DepthImage<'_>,
    ) -> Result<DepthTexture2d, TextureError> {
        let raw = self.raw.create_texture_2d(image.raw().clone())?;

        Ok(DepthTexture2d::from_raw(raw))
    }

    pub fn create_program<U, U1, U2, V, F, W, InV, OutW, InW, OutF>(
        &self,
        vertex_shader: fn(U1, InV) -> OutW,
        fragment_shader: fn(U2, InW) -> OutF,
    ) -> Result<Program<U, V, F>, ProgramError>
    where
        U: UniformUnion<U1, U2>,
        U1: Uniform<Sl>,
        U2: Uniform<Sl>,
        V: Vertex<Sl>,
        F: Fragment<Sl>,
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

        log::info!("Vertex shader:\n{}", program_def.vertex_shader_source);
        log::info!("Fragment shader:\n{}", program_def.fragment_shader_source);

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
            UData: Uniform<Sl, Sl = UData>,
            VData: Vertex<Sl, Sl = VData>,
            FData: Fragment<Sl, Sl = FData>,
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

    pub fn set_default_framebuffer_size(&self, size: glam::UVec2) {
        self.raw.set_default_framebuffer_size(size);
    }
}
