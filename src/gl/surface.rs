use crate::{sl, FragmentData, Logical};

pub trait Surface<F: FragmentData<Logical>> {}

pub struct DefaultFramebuffer;

impl Surface<sl::Vec4<f32>> for DefaultFramebuffer {}
