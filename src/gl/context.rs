use std::{
    any::{type_name, TypeId},
    cell::RefCell,
    collections::hash_map,
    marker::PhantomData,
    rc::Rc,
};

use fxhash::FxHashMap;

use crate::{
    sl::{
        transpile::{transpile_to_program_def, transpile_to_program_def_with_consts},
        ColorSample, FsFunc, FsSig, VsFunc, VsSig,
    },
    Block, Gl, Sl, UniformInterface, UniformUnion,
};

use super::{
    program::{DrawBuilder, DrawBuilderWithUniforms},
    raw, BufferError, BufferUsage, Caps, ColorImage, ColorTexture2d, ContextError, DepthImage,
    DepthTexture2d, Element, ElementBuffer, Program, ProgramError, TextureError, UniformBuffer,
    VertexBuffer,
};

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct ProgramCacheKey {
    vertex_shader: TypeId,
    fragment_shader: TypeId,
    uniform_union: TypeId,
}

#[derive(Default)]
pub(crate) struct ProgramCache(FxHashMap<ProgramCacheKey, Rc<raw::Program>>);

impl ProgramCache {
    pub fn get<U, VSig, VFunc, FSig, FFunc>(
        &mut self,
        raw: &raw::Context,
        vertex_shader: VFunc,
        fragment_shader: FFunc,
    ) -> Result<Program<U, VSig::V, FSig::F>, ProgramError>
    where
        U: UniformUnion<VSig::U, FSig::U> + 'static,
        VSig: VsSig<C = ()>,
        VFunc: VsFunc<VSig>,
        FSig: FsSig<C = (), W = VSig::W>,
        FFunc: FsFunc<FSig>,
    {
        let key = ProgramCacheKey {
            vertex_shader: TypeId::of::<VFunc>(),
            fragment_shader: TypeId::of::<FFunc>(),
            uniform_union: TypeId::of::<U>(),
        };

        let raw = match self.0.entry(key) {
            hash_map::Entry::Occupied(entry) => entry.get().clone(),
            hash_map::Entry::Vacant(entry) => {
                let program_def = transpile_to_program_def::<U, VSig, VFunc, FSig, FFunc>(
                    vertex_shader,
                    fragment_shader,
                );

                log::info!(
                    "Caching vertex shader for `{}`:\n{}",
                    type_name::<VFunc>(),
                    program_def.vertex_shader_source
                );
                log::info!(
                    "Caching fragment shader for `{}`:\n{}",
                    type_name::<FFunc>(),
                    program_def.fragment_shader_source
                );

                let raw = raw.create_program(program_def)?;

                entry.insert(Rc::new(raw)).clone()
            }
        };

        Ok(Program::unchecked_from_raw(raw))
    }
}

pub struct CacheDrawBuilder<'a, VSig, VFunc, FSig, FFunc> {
    gl: &'a Context,
    vertex_shader: VFunc,
    fragment_shader: FFunc,
    _phantom: PhantomData<(VSig, FSig)>,
}

impl<'a, VSig, VFunc, FSig, FFunc> CacheDrawBuilder<'a, VSig, VFunc, FSig, FFunc>
where
    VSig: VsSig<C = ()>,
    VFunc: VsFunc<VSig>,
    FSig: FsSig<C = (), W = VSig::W>,
    FFunc: FsFunc<FSig>,
{
    pub fn with_uniforms<U>(
        self,
        uniforms: U,
    ) -> Result<DrawBuilderWithUniforms<U::Sl, VSig::V, FSig::F>, ProgramError>
    where
        U: UniformInterface<Gl>,
        U::Sl: UniformUnion<VSig::U, FSig::U> + UniformInterface<Sl, Gl = U> + 'static,
    {
        let program = self
            .gl
            .program_cache
            .borrow_mut()
            .get::<U::Sl, VSig, VFunc, FSig, FFunc>(
                &self.gl.raw,
                self.vertex_shader,
                self.fragment_shader,
            )?;

        let inner = DrawBuilder {
            raw: program.raw().clone(),
            settings: Default::default(),
            _phantom: PhantomData,
        };

        Ok(DrawBuilderWithUniforms { inner, uniforms })
    }

    // TODO: Also needs `with_framebuffer` and `with_settings`.
}

/// The graphics context, which is used for creating GPU objects.
#[derive(Clone)]
pub struct Context {
    raw: Rc<raw::Context>,
    program_cache: Rc<RefCell<ProgramCache>>,
}

impl Context {
    pub fn new(gl: glow::Context) -> Result<Self, ContextError> {
        let raw = raw::Context::new(gl)?;

        Ok(Self {
            raw: Rc::new(raw),
            program_cache: Default::default(),
        })
    }

    pub fn caps(&self) -> &Caps {
        self.raw.caps()
    }

    pub fn create_vertex_buffer<B>(
        &self,
        data: &[B],
        usage: BufferUsage,
    ) -> Result<VertexBuffer<B>, BufferError>
    where
        B: Block<Gl> + bytemuck::Pod,
    {
        let raw = self
            .raw
            .create_buffer(bytemuck::cast_slice(data), glow::ARRAY_BUFFER, usage)?;

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
        let raw = self.raw.create_buffer(
            bytemuck::cast_slice(data),
            glow::ELEMENT_ARRAY_BUFFER,
            usage,
        )?;

        Ok(ElementBuffer::from_raw(raw))
    }

    pub fn create_uniform_buffer<B>(
        &self,
        data: B::Gl,
        usage: BufferUsage,
    ) -> Result<UniformBuffer<B>, BufferError>
    where
        B: Block<Gl>,
    {
        UniformBuffer::new(&self.raw, &data, usage)
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

    pub fn create_program<U, VSig, VFunc, FSig, FFunc>(
        &self,
        vertex_shader: VFunc,
        fragment_shader: FFunc,
    ) -> Result<Program<U, VSig::V, FSig::F>, ProgramError>
    where
        U: UniformUnion<VSig::U, FSig::U>,
        VSig: VsSig<C = ()>,
        VFunc: VsFunc<VSig>,
        FSig: FsSig<C = (), W = VSig::W>,
        FFunc: FsFunc<FSig>,
    {
        let program_def =
            transpile_to_program_def::<U, VSig, VFunc, FSig, FFunc>(vertex_shader, fragment_shader);

        log::info!("Vertex shader:\n{}", program_def.vertex_shader_source);
        log::info!("Fragment shader:\n{}", program_def.fragment_shader_source);

        let raw = self.raw.create_program(program_def)?;

        Ok(Program::unchecked_from_raw(Rc::new(raw)))
    }

    pub fn create_program_with_consts<U, VSig, VFunc, FSig, FFunc>(
        &self,
        consts: &VSig::C,
        vertex_shader: VFunc,
        fragment_shader: FFunc,
    ) -> Result<Program<U, VSig::V, FSig::F>, ProgramError>
    where
        U: UniformUnion<VSig::U, FSig::U>,
        VSig: VsSig,
        VFunc: VsFunc<VSig>,
        FSig: FsSig<C = VSig::C, W = VSig::W>,
        FFunc: FsFunc<FSig>,
    {
        let program_def = transpile_to_program_def_with_consts::<U, VSig, VFunc, FSig, FFunc>(
            consts,
            vertex_shader,
            fragment_shader,
        );

        log::info!("Vertex shader:\n{}", program_def.vertex_shader_source);
        log::info!("Fragment shader:\n{}", program_def.fragment_shader_source);

        let raw = self.raw.create_program(program_def)?;

        Ok(Program::unchecked_from_raw(Rc::new(raw)))
    }

    pub fn get_program<VSig, VFunc, FSig, FFunc>(
        &self,
        vertex_shader: VFunc,
        fragment_shader: FFunc,
    ) -> CacheDrawBuilder<'_, VSig, VFunc, FSig, FFunc>
    where
        VSig: VsSig<C = ()>,
        VFunc: VsFunc<VSig>,
        FSig: FsSig<C = (), W = VSig::W>,
        FFunc: FsFunc<FSig>,
    {
        CacheDrawBuilder {
            gl: self,
            vertex_shader,
            fragment_shader,
            _phantom: PhantomData,
        }
    }

    pub fn default_framebuffer_size(&self) -> [u32; 2] {
        self.raw.default_framebuffer_size()
    }

    pub fn set_default_framebuffer_size(&self, size: [u32; 2]) {
        self.raw.set_default_framebuffer_size(size);
    }

    pub fn finish(&self) {
        self.raw.finish();
    }
}
