use std::marker::PhantomData;

use crate::{Fragment, Gl};

pub struct SurfaceBinding<F: Fragment<Gl>>(PhantomData<F>);

pub struct DefaultSurface;
