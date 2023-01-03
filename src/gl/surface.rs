use crate::{sl, FragmentInterface, Sl};

pub trait Surface<F: FragmentInterface<Sl>> {}

pub struct DefaultFramebuffer;

impl Surface<sl::Vec4<f32>> for DefaultFramebuffer {}
