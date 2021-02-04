
use {
    super::{plane_contains_point, plane_intersects_ray, Shape, CircleShape},
    crate::Ray,
    nalgebra::{Point3, Unit, Vector3, Isometry3, Isometry},
};


pub struct CylinderShape {
    to_local: Isometry3<f64>,
    to_global: Isometry3<f64>,
    top_circle: CircleShape,
    bottom_circle: CircleShape,
    pub origin: Point3<f64>,
    pub height: f64,
    pub orientation: Unit<Vector3<f64>>,
    pub radius: f64,
}

impl CylinderShape {
    fn new(origin: Point3<f64>, orientation: Vector3<f64>, height: f64, radius: f64) -> Self {
        let uorientation: Unit<Vector3<f64>> = Unit::new_normalize(orientation);

        // create arbitrary vector orthogonal to orientation
        // TODO: separate and test this
        let mut orth = orientation;
        orth.x *= 0.5;
        orth.y *= 2.0;
        orth.z *= 4.0;
        orth = orth

        // create top and bottom circles
        let top_circle = CircleShape::new(origin + uorientation.into_inner() * height / 2.0, uorientation.into_inner(), radius);
        let bottom_circle = CircleShape::new(origin - uorientation.into_inner() * height / 2.0, -1.0 * uorientation.into_inner(), radius);

        // create to_local and to_global transformation
        let to_local = Isometry::face_towards(origin, top_circle.origin, )
        todo!()
    }
}

impl Shape for CylinderShape{
    fn intersection
}

impl Shape for InfinitePlaneShape {
    fn intersection(&self, ray: &Ray) -> Option<Point3<f64>> {
        plane_intersects_ray(&self.origin, &self.normal, ray)
    }
    fn normal_at(&self, point: &Point3<f64>) -> Option<Unit<Vector3<f64>>> {
        if self.contains(point) {
            Some(self.normal)
        } else {
            None
        }
    }
    fn contains(&self, point: &Point3<f64>) -> bool {
        plane_contains_point(&self.origin, &self.normal, point)
    }
    fn origin(&self) -> Point3<f64> {
        self.origin
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::VOP;
    use std::sync::Arc;

    fn xy_plane() -> InfinitePlaneShape {
        InfinitePlaneShape {
            origin: Point3::new(0.0, 0.0, 0.0),
            normal: Unit::new_normalize(Vector3::new(0.0, 0.0, 1.0)),
        }
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
