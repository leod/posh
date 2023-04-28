use bytemuck::{Pod, Zeroable};
use crevice::std140::AsStd140;

use crate::sl::{self, ToSl};

use super::{Vec2, Vec3, Vec4};

/// A two-by-two floating-point matrix.
#[derive(Clone, Copy, Zeroable, Pod, Debug, PartialEq, PartialOrd)]
#[repr(C)]
pub struct Mat2 {
    pub x_axis: Vec2,
    pub y_axis: Vec2,
}

/// A three-by-three floating-point matrix.
#[derive(Clone, Copy, Zeroable, Pod, Debug, PartialEq, PartialOrd)]
#[repr(C)]
pub struct Mat3 {
    pub x_axis: Vec3,
    pub y_axis: Vec3,
    pub z_axis: Vec3,
}

/// A four-by-four floating-point matrix.
#[derive(Clone, Copy, Zeroable, Pod, Debug, PartialEq, PartialOrd)]
#[repr(C)]
pub struct Mat4 {
    pub x_axis: Vec4,
    pub y_axis: Vec4,
    pub z_axis: Vec4,
    pub w_axis: Vec4,
}

macro_rules! impl_convs {
    (
        $mat:ident,
        $size:tt,
        $mint:ty,
        ($($field:ident),+),
        ($($field_crevice:ident),+)
    ) => {
        impl ToSl for $mat {
            type Output = sl::$mat;

            fn to_sl(self) -> sl::$mat {
                sl::$mat {
                    $(
                        $field: self.$field.to_sl()
                    ),+
                }
            }
        }

        impl AsStd140 for $mat {
            type Output = crevice::std140::$mat;

            fn as_std140(&self) -> Self::Output {
                let mut output = <Self::Output as Zeroable>::zeroed();

                $(
                    output.$field_crevice = self.$field.as_std140();
                )+

                output
            }

            fn from_std140(value: Self::Output) -> Self {
                Self {
                    $(
                        $field: AsStd140::from_std140(value.$field_crevice)
                    ),+
                }
            }
        }

        impl From<[[f32; $size]; $size]> for $mat {
            #[allow(unused)]
            fn from(value: [[f32; $size]; $size]) -> Self {
                let mut i = 0;

                $(
                    let $field = value[i].into();
                    i += 1;
                )+

                Self { $($field),+ }
            }
        }

        impl From<$mat> for [[f32; $size]; $size] {
            fn from(value: $mat) -> Self {
                [$(value.$field.into()),+]
            }
        }

        #[cfg(feature = "mint")]
        impl From<$mint> for $mat {
            fn from(value: $mint) -> Self {
                Self { $($field: value.$field_crevice.into()),+ }
            }
        }

        #[cfg(feature = "mint")]
        impl From<$mat> for $mint {
            fn from(value: $mat) -> Self {
                Self { $($field_crevice: value.$field.into()),+ }
            }
        }

        #[cfg(feature = "glam")]
        impl From<glam::$mat> for $mat {
            fn from(value: glam::$mat) -> Self {
                Self { $($field: value.$field.into()),+ }
            }
        }

        #[cfg(feature = "glam")]
        impl From<$mat> for glam::$mat {
            fn from(value: $mat) -> Self {
                glam::$mat::from_cols($(value.$field.into()),+)
            }
        }
    };
}

impl_convs!(Mat2, 2, mint::ColumnMatrix2<f32>, (x_axis, y_axis), (x, y));
impl_convs!(
    Mat3,
    3,
    mint::ColumnMatrix3<f32>,
    (x_axis, y_axis, z_axis),
    (x, y, z)
);
impl_convs!(
    Mat4,
    4,
    mint::ColumnMatrix4<f32>,
    (x_axis, y_axis, z_axis, w_axis),
    (x, y, z, w)
);
