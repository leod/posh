use std::{marker::PhantomData, mem::size_of, rc::Rc};

use crevice::std140::{AsStd140, Std140};

use crate::{Block, Gl};

use super::{raw, BufferError, BufferUsage};

/// Stores a uniform block in a buffer on the GPU.
///
/// Instances of `UniformBuffer` can be created with
/// [`Context::create_uniform_buffer`](crate::gl::Context::create_uniform_buffer).
pub struct UniformBuffer<B> {
    pub(super) raw: Rc<raw::Buffer>,
    _phantom: PhantomData<B>,
}

#[derive(Clone)]
pub struct UniformBufferBinding<B> {
    raw: Rc<raw::Buffer>,
    _phantom: PhantomData<B>,
    // TODO: Uniform buffer slicing.
}

impl<B: Block<Gl>> UniformBuffer<B> {
    pub(super) fn new(
        ctx: &raw::Context,
        data: &B::Gl,
        usage: BufferUsage,
    ) -> Result<Self, BufferError> {
        let mut buffer = Vec::new();
        let data = data.as_std140();
        let bytes = to_bytes(&data, &mut buffer);

        let raw = ctx.create_buffer(bytes, glow::UNIFORM_BUFFER, usage)?;

        Ok(Self {
            raw: Rc::new(raw),
            _phantom: PhantomData,
        })
    }

    pub fn usage(&self) -> BufferUsage {
        self.raw.usage()
    }

    pub fn set(&self, data: B::Gl) {
        let mut buffer = Vec::new();
        let data = data.as_std140();
        let bytes = to_bytes(&data, &mut buffer);

        self.raw.set(bytes);
    }

    pub fn as_binding(&self) -> UniformBufferBinding<B::Sl> {
        UniformBufferBinding {
            raw: self.raw.clone(),
            _phantom: PhantomData,
        }
    }
}

impl<B> UniformBufferBinding<B> {
    pub(super) fn raw(&self) -> &raw::Buffer {
        &self.raw
    }
}

fn to_bytes<'a, B: Std140>(data: &'a B, buffer: &'a mut Vec<u8>) -> &'a [u8] {
    // FIXME: This is a workaround for cases like an uniform buffer that
    // contains only a `Vec2`. In this case, `crevice` gives us a type that is
    // only 8 bytes in size. However, as far as I can tell, OpenGL requires us
    // to round up to multiples of 16 bytes. At least Firefox complains
    // if we do not do this.

    const MIN_ALIGNMENT: usize = 16;

    let bytes = data.as_bytes();

    let rem = size_of::<<B as AsStd140>::Output>() % MIN_ALIGNMENT;

    if rem == 0 {
        return bytes;
    }

    assert!(rem < MIN_ALIGNMENT);

    let padded_len = bytes.len() + MIN_ALIGNMENT - rem;

    assert!(padded_len > bytes.len());
    assert_eq!(padded_len % MIN_ALIGNMENT, 0);

    buffer.resize(padded_len, 0);
    buffer[0..bytes.len()].copy_from_slice(bytes);

    buffer.as_slice()
}
