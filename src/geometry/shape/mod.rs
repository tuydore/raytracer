mod shape2d;
mod shape3d;

pub use crate::{
    geometry::{Point3D, Vector3D, VOP},
    ray::{BounceResult, Ray},
};

pub use self::{
    shape2d::{Plane, Rectangle},
    shape3d::Sphere,
};

pub trait Shape {
    /// Returns the closest intersection and the distance^2 to it.
    fn intersection(&self, ray: &Ray) -> Option<(Point3D, f64)>;
    fn intersects(&self, ray: &Ray) -> bool {
        self.intersection(&ray).is_some()
    }
    fn contains(&self, point: &Point3D) -> bool;
    fn normal_at(&self, point: &Point3D) -> Option<Vector3D>;
    fn bounce(&self, ray: &mut Ray) -> BounceResult;
    fn vop_above(&self) -> &VOP;
    fn vop_below(&self) -> &VOP;
}
