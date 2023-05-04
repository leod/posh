use std::{marker::PhantomData, rc::Rc};

use crate::{sl::program_def::VertexInputRate, Block, Sl};

use super::{raw, BufferUsage, Mode, VertexSpec};

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
    B: Block<Sl, Sl = B>,
{
    pub(super) fn from_raw(raw: raw::Buffer) -> Self {
        assert!(vertex_size::<B>() > 0);
        assert_eq!(raw.len() % vertex_size::<B>(), 0);

        Self {
            raw: Rc::new(raw),
            _phantom: PhantomData,
        }
    }

    pub fn usage(&self) -> BufferUsage {
        self.raw.usage()
    }

    pub fn len(&self) -> usize {
        assert_eq!(self.raw.len() % vertex_size::<B>(), 0);

        self.raw.len() / vertex_size::<B>()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn set(&self, data: &[B::Gl]) {
        self.raw.set(bytemuck::cast_slice(data));
    }

    pub fn as_binding(&self) -> VertexBufferBinding<B> {
        VertexBufferBinding {
            raw: self.raw.clone(),
            input_rate: VertexInputRate::VsBindings,
            _phantom: PhantomData,
        }
    }

    pub fn as_vertex_spec(&self, mode: Mode) -> VertexSpec<B> {
        VertexSpec::new(mode).with_vertex_data(self.as_binding())
    }
}

impl<B: Block<Sl>> VertexBufferBinding<B> {
    pub(crate) fn raw(&self) -> &Rc<raw::Buffer> {
        &self.raw
    }

    pub fn len(&self) -> usize {
        assert_eq!(self.raw.len() % vertex_size::<B>(), 0);

        self.raw.len() / vertex_size::<B>()
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

pub(super) fn vertex_size<V: Block<Sl>>() -> usize {
    std::mem::size_of::<V::Gl>()
}
