pub mod checkerboard;
pub mod mandelbrot;
pub mod simple;
use nalgebra::Isometry3;
pub use {
    checkerboard::CheckerboardBuilder, mandelbrot::MandelbrotPlaneBuilder, simple::PlaneBuilder,
};

use crate::TOLERANCE;

use {
    super::{random_orthogonal, Shape},
    crate::Ray,
    nalgebra::{Point3, Unit, Vector3},
};

// TODO: exception for *PERFECTLY* parallel case
pub fn plane_intersects_line(
    plane_origin: &Point3<f64>,
    plane_normal: &Unit<Vector3<f64>>,
    line_origin: &Point3<f64>,
    line_direction: &Vector3<f64>,
) -> Option<Point3<f64>> {
    // if line is parallel to plane
    if line_direction.dot(plane_normal).abs() <= f64::EPSILON {
        None
    } else {
        Some(
            *line_origin
                + *line_direction * plane_normal.dot(&(*plane_origin - *line_origin))
                    / plane_normal.dot(line_direction),
        )
    }
}

pub fn plane_contains_point(
    plane_origin: &Point3<f64>,
    plane_normal: &Unit<Vector3<f64>>,
    point: &Point3<f64>,
) -> bool {
    plane_normal.dot(&(*plane_origin - *point)).abs() <= TOLERANCE
}

pub fn plane_intersects_ray(
    plane_origin: &Point3<f64>,
    plane_normal: &Unit<Vector3<f64>>,
    ray: &Ray,
) -> Option<Point3<f64>> {
    // if the line of the ray intersects the plane
    if let Some(intersection) =
        plane_intersects_line(plane_origin, plane_normal, &ray.origin, &ray.direction)
    {
        let origin_to_intersection = intersection - ray.origin;
        // ray origin is not on plane
        if !plane_contains_point(plane_origin, plane_normal, &ray.origin) &&
            // ray is going towards plane
            origin_to_intersection.dot(&ray.direction) >= 0.0
        {
            return Some(intersection);
        }
    }
    None
}

pub struct PlaneShape {
    pub origin: Point3<f64>,
    pub normal: Unit<Vector3<f64>>,
    orientation: Unit<Vector3<f64>>,
    to_local: Isometry3<f64>,
    to_global: Isometry3<f64>,
}

impl PlaneShape {
    pub fn new(
        origin: Point3<f64>,
        normal: Vector3<f64>,
        orientation: Option<Vector3<f64>>,
    ) -> Self {
        // if no orientation is given, choose one at random
        let orientation: Unit<Vector3<f64>> =
            Unit::new_normalize(orientation.unwrap_or_else(|| random_orthogonal(&normal)));

        // QUESTION: is this the right way around?
        // create to_local
        let to_local: Isometry3<f64> =
            Isometry3::face_towards(&origin, &(origin + normal), &orientation.into_inner());
        Self {
            origin,
            normal: Unit::new_normalize(normal),
            orientation,
            to_local,
            to_global: to_local.inverse(),
        }
    }
}

impl Shape for PlaneShape {
    fn intersection(&self, ray: &Ray) -> Option<Point3<f64>> {
        plane_intersects_ray(&self.origin, &self.normal, ray)
    }
    fn unchecked_normal_at(&self, _: &Point3<f64>) -> Unit<Vector3<f64>> {
        self.normal
    }
    fn contains(&self, point: &Point3<f64>) -> bool {
        plane_contains_point(&self.origin, &self.normal, point)
    }
    fn origin(&self) -> &Point3<f64> {
        &self.origin
    }
    fn to_local(&self) -> &Isometry3<f64> {
        &self.to_local
    }
    fn to_global(&self) -> &Isometry3<f64> {
        &self.to_global
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::VOP;
    use std::sync::Arc;

    fn xy_plane() -> PlaneShape {
        PlaneShape::new(Point3::new(0.0, 0.0, 0.0), Vector3::z(), None)
    }

    #[test]
    fn test_intersection() {
        let plane = xy_plane();
        let air = Arc::new(VOP {
            ior: 1.0,
            abs: [0.0; 3],
        });
        let ray = Ray {
            origin: Point3::new(0.0, 0.0, 1.0),
            direction: Vector3::new(0.0, 1.0, -1.0),
            vop: air,
            abs: [0.0; 3],
        };
        assert!(plane.intersects(&ray));
        assert_eq!(plane.intersection(&ray), Some(Point3::new(0.0, 1.0, 0.0)));
    }
    #[test]
    fn test_no_intersection() {
        let plane = xy_plane();
        let air = Arc::new(VOP {
            ior: 1.0,
            abs: [0.0; 3],
        });
        let ray = Ray {
            origin: Point3::new(0.0, 0.0, 1.0),
            direction: Vector3::new(1.0, 0.0, 0.0),
            vop: air,
            abs: [0.0; 3],
        };
        assert!(!plane.intersects(&ray));
        assert_eq!(plane.intersection(&ray), None);
    }
}
