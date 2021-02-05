pub mod cylinder;
pub mod disk;
pub mod paraboloid;
pub mod plane;
pub mod rectangle;
pub mod sphere;
pub use {
    cylinder::CylinderBuilder,
    paraboloid::ParaboloidBuilder,
    plane::{CheckerboardBuilder, MandelbrotPlaneBuilder, PlaneBuilder},
    rectangle::{RectangleBuilder, TexturedRectangleBuilder},
    sphere::SphereBuilder,
};

use {
    crate::{Ray, TOLERANCE, VOP},
    nalgebra::{Isometry3, Point3, Unit, Vector3},
    serde::Deserialize,
    std::collections::HashMap,
    std::sync::Arc,
};

/// Generate a random vector that's orthogonal to the first.
fn random_orthogonal(vector: &Vector3<f64>) -> Vector3<f64> {
    // check input is not null
    if vector.norm_squared() <= TOLERANCE {
        panic!("Cannot generate random normal to null vector.")
    }

    // create some other vector
    let other: Vector3<f64> = if vector.y <= TOLERANCE && vector.z <= TOLERANCE {
        Vector3::y()
    } else {
        Vector3::x()
    };

    other.cross(vector)
}

/// Pick closest ray intersection out of all possible line intersections.
pub fn pick_closest_intersection(
    line_intersections: Vec<Point3<f64>>,
    ray: &Ray,
) -> Option<Point3<f64>> {
    if line_intersections.is_empty() {
        return None;
    }
    let mut enumerated_dsq: Vec<(usize, f64)> = line_intersections
        .iter()
        .enumerate()
        .map(|(i, p)| (i, *p - ray.origin))
        .filter(|(_, d)| d.dot(&ray.direction) >= 0.0)
        .map(|(i, d)| (i, d.norm_squared()))
        .filter(|(_, d2)| *d2 >= TOLERANCE)
        .collect();

    match enumerated_dsq.len() {
        0 => None,
        1 => Some(line_intersections[enumerated_dsq[0].0]),
        _ => {
            enumerated_dsq.sort_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap());
            Some(line_intersections[enumerated_dsq[0].0])
        }
    }
}

#[derive(Debug, Clone, Copy, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SOP {
    Reflect,
    Refract,
    Light(u8, u8, u8),
    Dark,
}

// TODO: SOP by loading image texture?

pub trait Shape {
    fn intersection(&self, ray: &Ray) -> Option<Point3<f64>>;
    fn unchecked_normal_at(&self, point: &Point3<f64>) -> Unit<Vector3<f64>>;
    fn contains(&self, point: &Point3<f64>) -> bool;
    fn intersects(&self, ray: &Ray) -> bool {
        self.intersection(ray).is_some()
    }
    fn origin(&self) -> &Point3<f64>;
    fn to_local(&self) -> &Isometry3<f64>;
    fn to_global(&self) -> &Isometry3<f64>;
}

pub trait Surface {
    fn intersection(&self, ray: &Ray) -> Option<Point3<f64>>;
    fn unchecked_normal_at(&self, point: &Point3<f64>) -> Unit<Vector3<f64>>;
    fn unchecked_vop_above_at(&self, point: &Point3<f64>) -> Arc<VOP>;
    fn unchecked_vop_below_at(&self, point: &Point3<f64>) -> Arc<VOP>;
    fn unchecked_sop_at(&self, point: &Point3<f64>) -> SOP;
}

pub trait SurfaceBuilder {
    fn build(self, vop_map: &HashMap<String, Arc<VOP>>) -> Arc<dyn Surface + Send + Sync>;
}
