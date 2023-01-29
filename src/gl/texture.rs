use std::marker::PhantomData;

use sealed::sealed;

use crate::sl;

pub enum Texture2dBinding {}

#[sealed]
pub trait TextureFormat {
    //type
}

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

/*

RGBA - UNSIGNED_BYTE - RGBA8, SRGB8_ALPHA8
RGBA - BYTE          - RGBA8_SNORM
RGBA - FLOAT         - RGBA32F, RGBA16F

RGB  - UNSIGNED_BYTE - RGB8, SRGB8
RGB  - BYTE          - RGB8_SNORM
RGB  - FLOAT         - RGB32, RGB16F

RG   - UNSIGNED_BYTE - RG8
RG   - BYTE          - RG8_SNORM
RG   - FLOAT         - RG32F, RG16F

RED  - UNSIGNED_BYTE - R8
RED  - BYTE          - R8_SNORM
RED  - FLOAT         - R32F, R16F

RGBA_INTEGER - UNSIGNED_BYTE - RGBA8UI
RGBA_INTEGER - BYTE          - RGBA8I
...

RGB_INTEGER  - UNSIGNED_BYTE  - RGB8UI
RGB_INTEGER  - BYTE           - RGB8I
...

RG_INTEGER   - UNSIGNED_BYTE  - RG8UI
RG_INTEGER   - BYTE           - RG 8I
...

RED_INTEGER  - UNSIGNED_BYTE  - R8UI
RED_INTEGER  - BYTE           - R8I
...

DEPTH_COMPONENT - UNSIGNED_SHORT - DEPTH_COMPONENT16
DEPTH_COMPONENT - UNSIGNED_INT   - DEPTH_COMPONENT24
DEPTH_COMPONENT - FLOAT          - DEPTH_COMPONENT32F

DEPTH_STENCIL   - UNSIGNED_INT_24_8 - DEPTH24_STENCIL8

*/
