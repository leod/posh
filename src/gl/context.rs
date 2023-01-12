use std::rc::Rc;

use crevice::std140::AsStd140;
use glow::HasContext;

use crate::{
    sl::{FragmentInput, FragmentOutput, Varying, VertexInput, VertexOutput},
    FragmentInterface, ResourceInterface, Sl, ToPod, Uniform, Vertex, VertexInterface,
};

use super::{
    untyped, BufferUsage, CreateBufferError, CreateError, CreateProgramError,
    CreateVertexArrayError, Element, ElementBuffer, ElementOrUnit, Program, UniformBuffer,
    VertexArray, VertexBuffer,
};

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
        data: &[V::InGl],
        usage: BufferUsage,
    ) -> Result<VertexBuffer<V>, CreateBufferError>
    where
        V: Vertex<Sl>,
    {
        // TODO: This extra allocation for converting to `V::Pod` could be
        // eliminated if we see the need.
        let data: Vec<_> = data.iter().copied().map(ToPod::to_pod).collect();

        // TODO: We should also allow passing `V::Pod` directly.

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
        U: Uniform<Sl>,
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
        VertexArray::new(self, vertex_buffers, element_source)
    }

    pub fn create_simple_vertex_array<V, E>(
        &self,
        vertices: &[V::InGl],
        usage: BufferUsage,
        element_source: E::Source,
    ) -> Result<VertexArray<V, E>, CreateError>
    where
        V: Vertex<Sl>,
        E: ElementOrUnit,
    {
        let vertex_buffer = self.create_vertex_buffer(vertices, usage)?;

        Ok(VertexArray::new(self, vertex_buffer, element_source)?)
    }

    pub fn create_program<R, V, F, W>(
        &self,
        vertex_shader: fn(R, VertexInput<V>) -> VertexOutput<W>,
        fragment_shader: fn(R, FragmentInput<W>) -> FragmentOutput<F>,
    ) -> Result<Program<R, V, F>, CreateProgramError>
    where
        R: ResourceInterface<Sl, InSl = R>,
        V: VertexInterface<Sl, InSl = V>,
        F: FragmentInterface<Sl, InSl = F>,
        W: Varying,
    {
        Program::new(self, vertex_shader, fragment_shader)
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
