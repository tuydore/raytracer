pub mod checkerboard;
pub mod mandelbrotplane;
pub mod plane;
pub mod rectangle;
pub mod sphere;
pub mod texturedrectangle;
pub mod zparaboloid;

// TODO: start grouping surfaces together

use {
    crate::{Shape, VOP},
    nalgebra::Point3,
    serde::Deserialize,
    std::collections::HashMap,
    std::sync::Arc,
};
pub use {
    checkerboard::CheckerboardBuilder, mandelbrotplane::MandelbrotPlaneBuilder,
    plane::PlaneBuilder, rectangle::RectangleBuilder, sphere::SphereBuilder,
    texturedrectangle::TexturedRectangleBuilder, zparaboloid::ZParaboloidBuilder,
};

#[derive(Debug, Clone, Copy, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SOP {
    Reflect,
    Refract,
    Light(u8, u8, u8),
    Dark,
}

// TODO: SOP by loading image texture?

pub trait Surface {
    fn geometry(&self) -> &dyn Shape;

    fn vop_above_at(&self, point: &Point3<f64>) -> Arc<VOP>;

    fn vop_below_at(&self, point: &Point3<f64>) -> Arc<VOP>;

    fn sop_at(&self, point: &Point3<f64>) -> SOP;
}

pub trait SurfaceBuilder {
    fn build(self, vop_map: &HashMap<String, Arc<VOP>>) -> Arc<dyn Surface + Send + Sync>;
}
