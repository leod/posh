use bytemuck::{Pod, Zeroable};
use crevice::std140::AsStd140;

use crate::sl::{self, ToSl};

use super::Bool;

macro_rules! size_name {
    (2) => {
        "two-dimensional"
    };
    (3) => {
        "three-dimensional"
    };
    (4) => {
        "four-dimensional"
    };
}

macro_rules! scalar_name {
    (f32) => {
        "floating-point"
    };
    (i32) => {
        "signed integer"
    };
    (u32) => {
        "unsigned integer"
    };
    (Bool) => {
        "boolean"
    };
}

macro_rules! tuple {
    ($scalar:ident, 2) => {
        ($scalar, $scalar)
    };
    ($scalar:ident, 3) => {
        ($scalar, $scalar, $scalar)
    };
    ($scalar:ident, 4) => {
        ($scalar, $scalar, $scalar, $scalar)
    };
}

macro_rules! impl_vec {
    (
        $vec:ident,
        $size:tt,
        $mint:ty,
        $scalar:ident,
        ($($field:ident),+),
        ($($index:tt),+)
    ) => {
        #[doc = concat!(
            "A ",
            size_name!($size),
            " ",
            scalar_name!($scalar),
            " vector.",
            "\n\n",
        )]
        #[derive(Clone, Copy, Zeroable, Pod, Debug, PartialEq, PartialOrd)]
        #[repr(C)]
        pub struct $vec {
            $(
                pub $field: $scalar
            ),+
        }

        impl ToSl for $vec {
            type Output = sl::$vec;

            fn to_sl(self) -> sl::$vec {
                sl::$vec {
                    $(
                        $field: self.$field.to_sl()
                    ),+
                }
            }
        }

        impl AsStd140 for $vec {
            type Output = crevice::std140::$vec;

            fn as_std140(&self) -> Self::Output {
                Self::Output {
                    $(
                        $field: self.$field.into()
                    ),+
                }
            }

            fn from_std140(value: Self::Output) -> Self {
                Self {
                    $(
                        $field: value.$field.into()
                    ),+
                }
            }
        }

        impl From<[$scalar; $size]> for $vec {
            fn from(value: [$scalar; $size]) -> Self {
                Self { $($field: value[$index]),+ }
            }
        }

        impl From<$vec> for [$scalar; $size] {
            fn from(value: $vec) -> Self {
                [$(value.$field.into()),+]
            }
        }

        impl From<tuple!($scalar, $size)> for $vec {
            fn from(value: tuple!($scalar, $size)) -> Self {
                Self {
                    $($field: value.$index),+
                }
            }
        }

        impl From<$vec> for tuple!($scalar, $size) {
            fn from(value: $vec) -> Self {
                ($(value.$field),+)
            }
        }

        #[cfg(feature = "mint")]
        impl From<$mint> for $vec {
            fn from(value: $mint) -> Self {
                Self { $($field: value.$field.into()),+ }
            }
        }

        #[cfg(feature = "mint")]
        impl From<$vec> for $mint {
            fn from(value: $vec) -> Self {
                Self { $($field: value.$field.into()),+ }
            }
        }

        #[cfg(feature = "glam")]
        impl From<glam::$vec> for $vec {
            fn from(value: glam::$vec) -> Self {
                Self { $($field: value.$field.into()),+ }
            }
        }

        #[cfg(feature = "glam")]
        impl From<$vec> for glam::$vec {
            fn from(value: $vec) -> Self {
                glam::$vec::new($(value.$field.into()),+)
            }
        }
    };
}

impl_vec!(Vec2, 2, mint::Vector2<f32>, f32, (x, y), (0, 1));
impl_vec!(Vec3, 3, mint::Vector3<f32>, f32, (x, y, z), (0, 1, 2));
impl_vec!(Vec4, 4, mint::Vector4<f32>, f32, (x, y, z, w), (0, 1, 2, 3));

impl_vec!(IVec2, 2, mint::Vector2<i32>, i32, (x, y), (0, 1));
impl_vec!(IVec3, 3, mint::Vector3<i32>, i32, (x, y, z), (0, 1, 2));
impl_vec!(
    IVec4,
    4,
    mint::Vector4<i32>,
    i32,
    (x, y, z, w),
    (0, 1, 2, 3)
);

impl_vec!(UVec2, 2, mint::Vector2<u32>, u32, (x, y), (0, 1));
impl_vec!(UVec3, 3, mint::Vector3<u32>, u32, (x, y, z), (0, 1, 2));
impl_vec!(
    UVec4,
    4,
    mint::Vector4<u32>,
    u32,
    (x, y, z, w),
    (0, 1, 2, 3)
);

impl_vec!(BVec2, 2, mint::Vector2<bool>, Bool, (x, y), (0, 1));
impl_vec!(BVec3, 3, mint::Vector3<bool>, Bool, (x, y, z), (0, 1, 2));
impl_vec!(
    BVec4,
    4,
    mint::Vector4<bool>,
    Bool,
    (x, y, z, w),
    (0, 1, 2, 3)
);
