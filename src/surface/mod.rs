pub mod checkerboard;
pub mod plane;
pub mod rectangle;
pub mod sphere;
pub mod zparaboloid;
use crate::{Point3D, Shape, VOP};
use serde::Deserialize;
use std::collections::HashMap;
use std::sync::Arc;

pub use {
    checkerboard::CheckerboardBuilder, plane::PlaneBuilder, rectangle::RectangleBuilder,
    sphere::SphereBuilder, zparaboloid::ZParaboloidBuilder,
};

#[derive(Debug, Clone, Copy, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SOP {
    Reflect,
    Refract,
    Light(u8, u8, u8),
    Dark,
}

pub trait Surface {
    fn geometry(&self) -> &dyn Shape;

    fn vop_above_at(&self, point: &Point3D) -> Arc<VOP>;

    fn vop_below_at(&self, point: &Point3D) -> Arc<VOP>;

    fn sop_at(&self, point: &Point3D) -> SOP;
}

pub trait SurfaceBuilder {
    fn build(self, vop_map: &HashMap<String, Arc<VOP>>) -> Arc<dyn Surface>;
}
