use std::marker::PhantomData;

use super::{ImageData, ImageFormat};

pub struct Texture2d<Format: ImageFormat> {
    _phantom: PhantomData<Format>,
}

impl<Format: ImageFormat> Texture2d<Format> {}
