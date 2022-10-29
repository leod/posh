use nalgebra as na;

use crate::{Expose, IntoPosh, Posh, Vec2, Vec3, Vec4};

use super::ScalarType;

impl<T: ScalarType> Expose for na::Vector2<T> {
    type Rep = Vec2<T>;
}

impl<T: ScalarType> Expose for na::Vector3<T> {
    type Rep = Vec3<T>;
}

impl<T: ScalarType> Expose for na::Vector4<T> {
    type Rep = Vec4<T>;
}

impl<T: ScalarType> IntoPosh for na::Vector3<T> {
    fn into_posh(self) -> Posh<Self> {
        self.data.0[0].into_posh()
    }
}

impl<T: ScalarType> IntoPosh for na::Vector4<T> {
    fn into_posh(self) -> Posh<Self> {
        self.data.0[0].into_posh()
    }
}
