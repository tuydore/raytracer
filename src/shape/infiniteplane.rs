use {
    super::{plane_contains_point, plane_intersects_ray, Shape},
    crate::{Point3D, Ray, Vector3D},
};

pub struct InfinitePlaneShape {
    pub origin: Point3D,
    pub normal: Vector3D,
}

impl Shape for InfinitePlaneShape {
    fn intersection(&self, ray: &Ray) -> Option<Point3D> {
        plane_intersects_ray(&self.origin, &self.normal, ray)
    }
    fn normal_at(&self, point: &Point3D) -> Option<Vector3D> {
        if self.contains(point) {
            Some(self.normal)
        } else {
            None
        }
    }
    fn contains(&self, point: &Point3D) -> bool {
        plane_contains_point(&self.origin, &self.normal, point)
    }
    fn origin(&self) -> Point3D {
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
            origin: Point3D::new(0.0, 0.0, 0.0),
            normal: Vector3D::new(0.0, 0.0, 1.0),
        }
    }

    #[test]
    fn test_intersection() {
        let plane = xy_plane();
        let air = Arc::new(VOP { ior: 1.0 });
        let ray = Ray {
            origin: Point3D::new(0.0, 0.0, 1.0),
            direction: Vector3D::new(0.0, 1.0, -1.0),
            vop: air,
        };
        assert!(plane.intersects(&ray));
        assert_eq!(plane.intersection(&ray), Some(Point3D::new(0.0, 1.0, 0.0)));
    }
    #[test]
    fn test_no_intersection() {
        let plane = xy_plane();
        let air = Arc::new(VOP { ior: 1.0 });
        let ray = Ray {
            origin: Point3D::new(0.0, 0.0, 1.0),
            direction: Vector3D::new(1.0, 0.0, 0.0),
            vop: air,
        };
        assert!(!plane.intersects(&ray));
        assert_eq!(plane.intersection(&ray), None);
    }
}
