use {
    super::{plane_contains_point, plane_intersects_ray, Shape},
    crate::Ray,
    nalgebra::{Point3, Vector3},
};

pub struct InfinitePlaneShape {
    pub origin: Point3<f64>,
    pub normal: Vector3<f64>,
}

impl Shape for InfinitePlaneShape {
    fn intersection(&self, ray: &Ray) -> Option<Point3<f64>> {
        plane_intersects_ray(&self.origin, &self.normal, ray)
    }
    fn normal_at(&self, point: &Point3<f64>) -> Option<Vector3<f64>> {
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
            normal: Vector3::new(0.0, 0.0, 1.0),
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
