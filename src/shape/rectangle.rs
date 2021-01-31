use {
    super::{plane_contains_point, plane_intersects_ray, Shape},
    crate::ray::Ray,
    nalgebra::{Point3, Vector3},
};

pub struct RectangleShape {
    pub origin: Point3<f64>,
    pub normal: Vector3<f64>,
    pub orientation: Vector3<f64>,
    pub size: [f64; 2], // orientation is along 1st size dimension
}

impl Shape for RectangleShape {
    fn intersection(&self, ray: &Ray) -> Option<Point3<f64>> {
        if let Some(p) = plane_intersects_ray(&self.origin, &self.normal, ray) {
            if self.contains(&p) {
                return Some(p);
            }
        }
        None
    }
    fn normal_at(&self, point: &Point3<f64>) -> Option<Vector3<f64>> {
        if self.contains(point) {
            Some(self.normal)
        } else {
            None
        }
    }
    fn contains(&self, point: &Point3<f64>) -> bool {
        // check if point is on plane
        if !plane_contains_point(&self.origin, &self.normal, point) {
            return false;
        }

        // check if point is in rectangle
        let from_origin = *point - self.origin;
        let l0 = from_origin.dot(&self.orientation).abs();
        let l1 = from_origin.dot(&self.normal.cross(&self.orientation)).abs();
        l0 <= self.size[0] / 2.0 && l1 <= self.size[1] / 2.0
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

    fn xy_square() -> RectangleShape {
        RectangleShape {
            origin: Point3::new(0.0, 0.0, 0.0),
            normal: Vector3::new(0.0, 0.0, 1.0),
            orientation: Vector3::new(0.0, 1.0, 0.0),
            size: [2.0, 2.0],
        }
    }

    #[test]
    fn test_intersection() {
        let square = xy_square();
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
        assert!(square.intersects(&ray));
        assert_eq!(square.intersection(&ray), Some(Point3::new(0.0, 1.0, 0.0)));
    }

    #[test]
    fn test_intersection_by_missing() {
        let square = xy_square();
        let air = Arc::new(VOP {
            ior: 1.0,
            abs: [0.0; 3],
        });
        let ray = Ray {
            origin: Point3::new(0.0, 0.0, 1.0),
            direction: Vector3::new(0.0, 3.0, -1.0),
            vop: air,
            abs: [0.0; 3],
        };
        assert!(!square.intersects(&ray));
        assert_eq!(square.intersection(&ray), None);
    }

    #[test]
    fn test_no_intersection_by_direction() {
        let square = xy_square();
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
        assert!(!square.intersects(&ray));
        assert_eq!(square.intersection(&ray), None);
    }
}
