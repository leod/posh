use std::{marker::PhantomData, rc::Rc};

use smallvec::SmallVec;

use crate::{
    interface::UniformVisitor,
    sl::{self, ColorSample},
    Block, FsInterface, Gl, Sl, Uniform, VsInterface,
};

use super::{
    raw, ColorSampler2d, ComparisonSampler2d, DrawError, DrawParams, Framebuffer,
    UniformBufferBinding, VertexSpec,
};

type RawProgram = Result<Rc<raw::Program>, DrawError>;

pub struct DrawBuilder<U, V, F>
where
    U: Uniform<Sl>,
    V: VsInterface<Sl>,
    F: FsInterface<Sl>,
{
    pub(crate) raw: RawProgram,
    pub(crate) params: DrawParams,
    pub(crate) _phantom: PhantomData<(U, V, F)>,
}

impl<U, V, F> DrawBuilder<U, V, F>
where
    U: Uniform<Sl>,
    V: VsInterface<Sl>,
    F: FsInterface<Sl>,
{
    #[must_use]
    pub fn with_params(mut self, params: DrawParams) -> Self {
        self.params = params;
        self
    }

    #[must_use]
    pub fn with_uniforms(self, uniforms: U::Gl) -> DrawBuilderWithUniforms<U, V, F> {
        DrawBuilderWithUniforms {
            inner: self,
            uniforms,
        }
    }

    #[must_use]
    pub fn with_framebuffer(
        self,
        framebuffer: impl Into<Framebuffer<F>>,
    ) -> DrawBuilderWithFramebuffer<U, V, F> {
        DrawBuilderWithFramebuffer {
            inner: self,
            framebuffer: framebuffer.into(),
        }
    }
}

impl<V> DrawBuilder<(), V, sl::Vec4>
where
    V: VsInterface<Sl>,
{
    pub fn draw(self, vertex_spec: VertexSpec<V>) -> Result<Self, DrawError> {
        let DrawBuilderWithUniformsAndFramebuffer { inner, .. } =
            DrawBuilderWithUniformsAndFramebuffer {
                inner: self,
                uniforms: (),
                framebuffer: Framebuffer::default(),
            }
            .draw(vertex_spec)?;

        Ok(inner)
    }
}

pub struct DrawBuilderWithUniforms<U, V, F>
where
    U: Uniform<Sl>,
    V: VsInterface<Sl>,
    F: FsInterface<Sl>,
{
    pub(crate) inner: DrawBuilder<U, V, F>,
    pub(crate) uniforms: U::Gl,
}

impl<U, V, F> DrawBuilderWithUniforms<U, V, F>
where
    U: Uniform<Sl>,
    V: VsInterface<Sl>,
    F: FsInterface<Sl>,
{
    #[must_use]
    pub fn with_params(mut self, params: DrawParams) -> Self {
        self.inner.params = params;
        self
    }

    #[must_use]
    pub fn with_uniforms(mut self, uniforms: U::Gl) -> Self {
        self.uniforms = uniforms;
        self
    }

    #[must_use]
    pub fn with_framebuffer(
        self,
        framebuffer: impl Into<Framebuffer<F>>,
    ) -> DrawBuilderWithUniformsAndFramebuffer<U, V, F> {
        DrawBuilderWithUniformsAndFramebuffer {
            inner: self.inner,
            uniforms: self.uniforms,
            framebuffer: framebuffer.into(),
        }
    }
}

impl<U, V> DrawBuilderWithUniforms<U, V, sl::Vec4>
where
    U: Uniform<Sl>,
    V: VsInterface<Sl>,
{
    pub fn draw(self, vertex_spec: VertexSpec<V>) -> Result<Self, DrawError> {
        let DrawBuilderWithUniformsAndFramebuffer {
            inner, uniforms, ..
        } = DrawBuilderWithUniformsAndFramebuffer {
            inner: self.inner,
            uniforms: self.uniforms,
            framebuffer: Framebuffer::default(),
        }
        .draw(vertex_spec)?;

        Ok(Self { inner, uniforms })
    }
}

pub struct DrawBuilderWithFramebuffer<U, V, F>
where
    U: Uniform<Sl>,
    V: VsInterface<Sl>,
    F: FsInterface<Sl>,
{
    inner: DrawBuilder<U, V, F>,
    framebuffer: Framebuffer<F>,
}

impl<U, V, F> DrawBuilderWithFramebuffer<U, V, F>
where
    U: Uniform<Sl>,
    V: VsInterface<Sl>,
    F: FsInterface<Sl>,
{
    #[must_use]
    pub fn with_params(mut self, params: DrawParams) -> Self {
        self.inner.params = params;
        self
    }

    #[must_use]
    pub fn with_uniforms(self, uniforms: U::Gl) -> DrawBuilderWithUniformsAndFramebuffer<U, V, F> {
        DrawBuilderWithUniformsAndFramebuffer {
            inner: self.inner,
            uniforms,
            framebuffer: self.framebuffer,
        }
    }

    #[must_use]
    pub fn with_framebuffer(mut self, framebuffer: impl Into<Framebuffer<F>>) -> Self {
        self.framebuffer = framebuffer.into();
        self
    }
}

impl<V, F> DrawBuilderWithFramebuffer<(), V, F>
where
    V: VsInterface<Sl>,
    F: FsInterface<Sl>,
{
    pub fn draw(self, vertex_spec: VertexSpec<V>) -> Result<Self, DrawError> {
        let DrawBuilderWithUniformsAndFramebuffer {
            inner, framebuffer, ..
        } = DrawBuilderWithUniformsAndFramebuffer {
            inner: self.inner,
            uniforms: (),
            framebuffer: self.framebuffer,
        }
        .draw(vertex_spec)?;

        Ok(Self { inner, framebuffer })
    }
}

pub struct DrawBuilderWithUniformsAndFramebuffer<U, V, F>
where
    U: Uniform<Sl>,
    V: VsInterface<Sl>,
    F: FsInterface<Sl>,
{
    inner: DrawBuilder<U, V, F>,
    uniforms: U::Gl,
    framebuffer: Framebuffer<F>,
}

impl<U, V, F> DrawBuilderWithUniformsAndFramebuffer<U, V, F>
where
    U: Uniform<Sl>,
    V: VsInterface<Sl>,
    F: FsInterface<Sl>,
{
    #[must_use]
    pub fn with_params(mut self, params: DrawParams) -> Self {
        self.inner.params = params;
        self
    }

    #[must_use]
    pub fn with_uniforms(mut self, uniforms: U::Gl) -> Self {
        self.uniforms = uniforms;
        self
    }

    #[must_use]
    pub fn with_framebuffer(mut self, framebuffer: impl Into<Framebuffer<F>>) -> Self {
        self.framebuffer = framebuffer.into();
        self
    }

    pub fn draw(self, vertex_spec: VertexSpec<V>) -> Result<Self, DrawError> {
        // TODO: Remove hardcoded path names.
        let mut uniform_visitor = CollectUniforms::default();
        self.uniforms.visit("", &mut uniform_visitor);

        // FIXME: Safety: check that all vertex buffers are large enough for the
        // values in the element buffer (if we have one).

        let raw = self.inner.raw.clone()?;

        unsafe {
            raw.draw(
                &uniform_visitor.raw_uniform_buffers,
                &uniform_visitor.raw_samplers,
                &vertex_spec.raw(),
                &self.framebuffer.raw(),
                &self.inner.params,
            )
        }?;

        // Make borrow checker happy.
        std::mem::drop(uniform_visitor);

        Ok(self)
    }
}

pub struct Program<U, V, F = sl::Vec4> {
    raw: Rc<raw::Program>,
    _phantom: PhantomData<(U, V, F)>,
}

impl<U, V, F> Program<U, V, F>
where
    U: Uniform<Sl>,
    V: VsInterface<Sl>,
    F: FsInterface<Sl>,
{
    pub(super) fn unchecked_from_raw(raw: Rc<raw::Program>) -> Self {
        Program {
            raw,
            _phantom: PhantomData,
        }
    }

    pub(super) fn raw(&self) -> &Rc<raw::Program> {
        &self.raw
    }

    #[must_use]
    pub fn with_params(&self, params: DrawParams) -> DrawBuilder<U, V, F> {
        DrawBuilder {
            raw: Ok(self.raw.clone()),
            params,
            _phantom: PhantomData,
        }
    }

    #[must_use]
    pub fn with_uniforms(&self, uniforms: U::Gl) -> DrawBuilderWithUniforms<U, V, F> {
        DrawBuilderWithUniforms {
            inner: self.with_params(DrawParams::new()),
            uniforms,
        }
    }

    #[must_use]
    pub fn with_framebuffer(
        &self,
        framebuffer: impl Into<Framebuffer<F>>,
    ) -> DrawBuilderWithFramebuffer<U, V, F> {
        DrawBuilderWithFramebuffer {
            inner: self.with_params(DrawParams::new()),
            framebuffer: framebuffer.into(),
        }
    }
}

impl<V> Program<(), V, sl::Vec4>
where
    V: VsInterface<Sl>,
{
    pub fn draw(
        &self,
        vertex_spec: VertexSpec<V>,
    ) -> Result<DrawBuilder<(), V, sl::Vec4>, DrawError> {
        self.with_params(DrawParams::new()).draw(vertex_spec)
    }
}

#[derive(Default)]
struct CollectUniforms<'a> {
    raw_uniform_buffers: SmallVec<[&'a raw::Buffer; 8]>,
    raw_samplers: SmallVec<[raw::Sampler; 8]>,
}

impl<'a> UniformVisitor<'a, Gl> for CollectUniforms<'a> {
    fn accept_block<B: Block<Sl, Sl = B>>(
        &mut self,
        _: &str,
        uniform: &'a UniformBufferBinding<B>,
    ) {
        self.raw_uniform_buffers.push(uniform.raw());
    }

    fn accept_color_sampler_2d<S: ColorSample>(&mut self, _: &str, sampler: &ColorSampler2d<S>) {
        self.raw_samplers
            .push(raw::Sampler::Sampler2d(sampler.raw().clone()))
    }

    fn accept_comparison_sampler_2d(&mut self, _: &str, sampler: &ComparisonSampler2d) {
        self.raw_samplers
            .push(raw::Sampler::Sampler2d(sampler.raw().clone()))
    }
}
