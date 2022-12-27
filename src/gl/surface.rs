use std::marker::PhantomData;

use crate::{FragmentInterface, Gl};

pub struct SurfaceBinding<F: FragmentInterface<Gl>>(PhantomData<F>);

pub struct DefaultSurface;
