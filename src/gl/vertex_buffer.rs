use std::{marker::PhantomData, mem::size_of, rc::Rc};

use crate::{sl::program_def::VertexInputRate, Block, Gl, Sl};

use super::{raw, BufferUsage, PrimitiveMode, VertexSpec};

/// Stores vertex blocks in a buffer on the GPU.
///
/// Instances of `VertexBuffer` can be created with
/// [`Context::create_vertex_buffer`](crate::gl::Context::create_vertex_buffer).
pub struct VertexBuffer<B> {
    raw: Rc<raw::Buffer>,
    _phantom: PhantomData<B>,
}

#[derive(Clone)]
pub struct VertexBufferBinding<B> {
    raw: Rc<raw::Buffer>,
    input_rate: VertexInputRate,
    _phantom: PhantomData<B>,
}

impl<B> VertexBuffer<B>
where
    B: Block<Gl> + bytemuck::Pod,
{
    pub(super) fn from_raw(raw: raw::Buffer) -> Self {
        assert!(size_of::<B>() > 0);
        assert_eq!(raw.len() % size_of::<B>(), 0);

        Self {
            raw: Rc::new(raw),
            _phantom: PhantomData,
        }
    }

    pub fn usage(&self) -> BufferUsage {
        self.raw.usage()
    }

    pub fn len(&self) -> usize {
        assert_eq!(self.raw.len() % size_of::<B>(), 0);

        self.raw.len() / size_of::<B>()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn set(&self, data: &[B]) {
        self.raw.set(bytemuck::cast_slice(data));
    }

    pub fn as_binding(&self) -> VertexBufferBinding<B::Sl> {
        VertexBufferBinding {
            raw: self.raw.clone(),
            input_rate: VertexInputRate::Vertex,
            _phantom: PhantomData,
        }
    }

    pub fn as_vertex_spec(&self, mode: PrimitiveMode) -> VertexSpec<B::Sl> {
        VertexSpec::new(mode).with_vertex_data(self.as_binding())
    }
}

impl<B: Block<Sl>> VertexBufferBinding<B> {
    pub(crate) fn raw(&self) -> &Rc<raw::Buffer> {
        &self.raw
    }

    pub fn len(&self) -> usize {
        assert_eq!(self.raw.len() % size_of::<B::Gl>(), 0);

        self.raw.len() / size_of::<B::Gl>()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn with_instancing(mut self) -> Self {
        self.input_rate = VertexInputRate::Instance;
        self
    }

    pub(crate) fn input_rate(&self) -> VertexInputRate {
        self.input_rate
    }
}
