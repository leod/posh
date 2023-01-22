use std::marker::PhantomData;

use sealed::sealed;

use crate::sl;

pub enum Texture2dBinding {}

#[sealed]
pub trait TextureDim {}

pub struct Dim2d;

pub struct Dim3d;

pub struct Dim2dArray;

pub struct DimCubeMap;

#[sealed]
pub trait TextureValue {}

pub struct Texture<Dim, Val = sl::Vec4<f32>> {
    _phantom: PhantomData<(Dim, Val)>,
}

type Texture2d<Val = sl::Vec4<f32>> = Texture<Dim2d, Val>;
type Texture3d<Val = sl::Vec4<f32>> = Texture<Dim3d, Val>;
type Texture2dArray<Val = sl::Vec4<f32>> = Texture<Dim2dArray, Val>;
type TextureCubeMap<Val = sl::Vec4<f32>> = Texture<DimCubeMap, Val>;

impl<Dim, Val> Texture<Dim, Val>
where
    Dim: TextureDim,
    Val: TextureValue,
{
}
