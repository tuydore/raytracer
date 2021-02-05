use crate::TOLERANCE;

use super::pick_closest_intersection;

use {
    super::{disk::DiskShape, Shape},
    crate::Ray,
    nalgebra::{Isometry3, Point3, Unit, Vector3},
};

/// The direction of the cylinder is bottom -> top, while the normals of each disk are defined
/// outwards. This means that, taking the bottom disk as to_local/global reference, the cylinder
/// is mapped to Z in [-height, 0]
pub struct CylinderShape {
    top_disk: DiskShape,
    bottom_disk: DiskShape,
    pub origin: Point3<f64>,
    pub height: f64,
    pub direction: Unit<Vector3<f64>>,
    pub radius: f64,
}

impl CylinderShape {
    fn new(origin: Point3<f64>, direction: Vector3<f64>, height: f64, radius: f64) -> Self {
        let udirection: Unit<Vector3<f64>> = Unit::new_normalize(direction);
        Self {
            origin,
            height,
            direction: udirection,
            radius,
            top_disk: DiskShape::new(
                origin + height / 2.0 * udirection.into_inner(),
                udirection.into_inner(),
                radius,
            ),
            bottom_disk: DiskShape::new(
                origin - height / 2.0 * udirection.into_inner(),
                -udirection.into_inner(),
                radius,
            ),
        }
    }
}

impl Shape for CylinderShape {
    fn intersection(&self, ray: &Ray) -> Option<Point3<f64>> {
        let mut intersections: Vec<Point3<f64>> = Vec::new();

        // TODO: optimize this
        // check disk intersections
        if let Some(p) = self.top_disk.intersection(ray) {
            intersections.push(p);
        }
        if let Some(p) = self.bottom_disk.intersection(ray) {
            intersections.push(p);
        }

        // if ray has already intersected both disks, it cannot intersect cylindrical surface
        if intersections.len() < 2 {
            // check cylinder itself intersection
            // get new ray origin and direction
            let o: Point3<f64> = self.to_local() * ray.origin;
            let d: Vector3<f64> = self.to_local() * ray.direction;

            // (ox + lambda * dx)^2 + (oy + lambda * dy)^2 = r^2
            // a = dx^2 + dy^2; b = 2 * (ox * dx + oy * dy), c = ox^2 + oy^2 - r^2
            let a: f64 = d.x.powi(2) + d.y.powi(2);
            let b: f64 = 2.0 * (o.x * d.x + o.y * d.y);
            let c: f64 = o.x.powi(2) + o.y.powi(2) - self.radius.powi(2);

            // define beta as (b^2 - 4ac)
            let beta: f64 = b.powi(2) - 4.0 * a * c;

            // -height <= oz + lambda * dz <= 0
            let mut lambda: f64;
            let mut z: f64;
            if beta >= 0.0 {
                // + case
                lambda = (-b + beta.sqrt()) / (2.0 * a);
                z = o.z + lambda * d.z;
                if z <= 0.0 && -self.height <= z {
                    intersections.push(o + lambda * d);
                }
                // - case
                lambda = (-b - beta.sqrt()) / (2.0 * a);
                z = o.z + lambda * d.z;
                if z <= 0.0 && -self.height <= z {
                    intersections.push(o + lambda * d);
                }
            }
        }

        // return closest intersection in global coords
        match intersections.len() {
            0 => None,
            1 => Some(self.to_global() * intersections.first().unwrap()),
            _ => Some(self.to_global() * pick_closest_intersection(intersections, ray).unwrap()),
        }
    }
    fn unchecked_normal_at(&self, point: &Point3<f64>) -> Unit<Vector3<f64>> {
        let local_point: Point3<f64> = self.to_local() * point;
        // if local point is at zero, this is the bottom surface
        if (local_point.z - 0.0) <= TOLERANCE {
            -self.direction
        // if it is at -height, it is at the top
        } else if (local_point.z + self.height) <= TOLERANCE {
            self.direction
        // otherwise construct vector out of X and Y coords and convert back to global
        } else {
            Unit::new_normalize(self.to_global() * Vector3::new(local_point.x, local_point.y, 0.0))
        }
    }
    fn contains(&self, point: &Point3<f64>) -> bool {
        // either on one of the disks
        if self.top_disk.contains(point) || self.bottom_disk.contains(point) {
            return true;
        }
        let local_point: Point3<f64> = self.to_local() * point;
        // or on the cylinder
        local_point.z <= 0.0
            && -self.height <= local_point.z
            && local_point.x.powi(2) + local_point.y.powi(2) - self.radius.powi(2) <= TOLERANCE
    }
    fn origin(&self) -> &Point3<f64> {
        &self.origin
    }
    fn to_local(&self) -> &Isometry3<f64> {
        self.bottom_disk.to_local()
    }
    fn to_global(&self) -> &Isometry3<f64> {
        self.bottom_disk.to_global()
    }
}
