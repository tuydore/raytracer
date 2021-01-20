use super::{plane_contains_point, plane_intersects_ray, Shape};
use crate::{ray::Ray, Point3D, Vector3D};

pub struct RectangleShape {
    pub origin: Point3D,
    pub normal: Vector3D,
    pub orientation: Vector3D,
    pub size: [f64; 2], // orientation is along 1st size dimension
}

impl Shape for RectangleShape {
    fn intersection(&self, ray: &Ray) -> Option<Point3D> {
        if let Some(p) = plane_intersects_ray(&self.origin, &self.normal, ray) {
            if self.contains(&p) {
                return Some(p);
            }
        }
        None
    }
    fn normal_at(&self, point: &Point3D) -> Option<Vector3D> {
        if self.contains(point) {
            Some(self.normal)
        } else {
            None
        }
    }
    fn contains(&self, point: &Point3D) -> bool {
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
    fn origin(&self) -> Point3D {
        self.origin
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::VOP;

    fn xy_square() -> RectangleShape {
        RectangleShape {
            origin: Point3D::new(0.0, 0.0, 0.0),
            normal: Vector3D::new(0.0, 0.0, 1.0),
            orientation: Vector3D::new(0.0, 1.0, 0.0),
            size: [2.0, 2.0],
        }
    }

    #[test]
    fn test_intersection() {
        let square = xy_square();
        let air = VOP::new(1.0);
        let ray = Ray::new(
            Point3D::new(0.0, 0.0, 1.0),
            Vector3D::new(0.0, 1.0, -1.0),
            &air,
        );
        assert!(square.intersects(&ray));
        assert_eq!(square.intersection(&ray), Some(Point3D::new(0.0, 1.0, 0.0)));
    }

    #[test]
    fn test_intersection_by_missing() {
        let square = xy_square();
        let air = VOP::new(1.0);
        let ray = Ray::new(
            Point3D::new(0.0, 0.0, 1.0),
            Vector3D::new(0.0, 3.0, -1.0),
            &air,
        );
        assert!(!square.intersects(&ray));
        assert_eq!(square.intersection(&ray), None);
    }

    #[test]
    fn test_no_intersection_by_direction() {
        let square = xy_square();
        let air = VOP::new(1.0);
        let ray = Ray::new(
            Point3D::new(0.0, 0.0, 1.0),
            Vector3D::new(1.0, 0.0, 0.0),
            &air,
        );
        assert!(!square.intersects(&ray));
        assert_eq!(square.intersection(&ray), None);
    }
}
