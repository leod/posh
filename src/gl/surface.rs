use std::marker::PhantomData;

use crate::{FragmentInterface, Sl};

pub struct SurfaceBinding<F: FragmentInterface<Sl>>(PhantomData<F>);

pub struct DefaultSurface;
