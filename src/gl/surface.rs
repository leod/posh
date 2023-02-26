use crate::{sl, Fragment, SlView};

pub trait Surface<F: Fragment<SlView>> {}

pub struct DefaultFramebuffer;

impl Surface<sl::Vec4> for DefaultFramebuffer {}
