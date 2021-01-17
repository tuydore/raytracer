use crate::{
    geometry::{
        shape::{Plane, Shape},
        Point3D, Vector3D, SOP, VOP,
    },
    ray::{BounceResult, Ray},
    SURFACE_INCLUSION,
};

pub struct Rectangle {
    plane: Plane,
    orientation: Vector3D,
    size: (f64, f64), // orientation is along 1st size dimension
}

impl Rectangle {
    pub fn new(
        origin: Point3D,
        normal: Vector3D,
        orientation: Vector3D,
        size: (f64, f64),
        surface: SOP,
        above: VOP,
        below: VOP,
    ) -> Self {
        Self {
            plane: Plane::new(origin, normal, surface, above, below),
            orientation,
            size,
        }
    }

    /// Returns the intersection of a line and the plane.
    fn line_intersection(&self, origin: &Point3D, direction: &Vector3D) -> Option<Point3D> {
        if let Some(p) = self.plane.line_intersection(origin, direction) {
            if self.contains(&p) {
                Some(p)
            } else {
                None
            }
        } else {
            None
        }
    }
}

impl Shape for Rectangle {
    fn intersection(&self, ray: &Ray) -> Option<(Point3D, f64)> {
        // if the line of the ray intersects the plane
        if let Some(intersection) = self.line_intersection(&ray.origin, &ray.direction) {
            let origin_to_intersection = intersection - ray.origin;
            // ray origin is not on plane
            if !self.contains(&ray.origin) &&
            // ray is going towards plane
            origin_to_intersection.dot(&ray.direction) >= 0.0
            {
                return Some((intersection, origin_to_intersection.length_squared()));
            }
        }
        None
    }

    /// More optimized version of intersects, that will discard ray automatically if line
    /// intersection fails.
    fn intersects(&self, ray: &Ray) -> bool {
        self.line_intersection(&ray.origin, &ray.direction)
            .is_some()
            && self.intersection(&ray).is_some()
    }

    fn contains(&self, point: &Point3D) -> bool {
        // check if point is on plane
        if !self.plane.contains(point) {
            return false;
        }

        // check if point is in rectangle
        let from_origin = *point - self.plane.origin;
        let l0 = from_origin.dot(&self.orientation).abs();
        let l1 = from_origin
            .dot(&self.plane.normal.cross(&self.orientation))
            .abs();
        l0 <= self.size.0 / 2.0 && l1 <= self.size.1 / 2.0
    }

    fn normal_at(&self, point: &Point3D) -> Option<Vector3D> {
        if self.contains(point) {
            Some(self.plane.normal)
        } else {
            None
        }
    }

    fn bounce(&self, ray: &mut Ray) -> BounceResult {
        self.plane.surface.bounce(ray, self)
    }

    fn vop_above(&self) -> &VOP {
        &self.plane.above
    }

    fn vop_below(&self) -> &VOP {
        &self.plane.below
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn xy_square() -> Rectangle {
        Rectangle::new(
            Point3D::new(0.0, 0.0, 0.0),
            Vector3D::new(0.0, 0.0, 1.0),
            Vector3D::new(0.0, 1.0, 0.0),
            (2.0, 2.0),
            SOP::Reflect,
            VOP::new(1.0),
            VOP::new(1.5),
        )
    }

    #[test]
    fn test_intersection() {
        let square = xy_square();
        let ray = Ray::new(
            Point3D::new(0.0, 0.0, 1.0),
            Vector3D::new(0.0, 1.0, -1.0),
            VOP::new(1.0),
        );
        assert!(square.intersects(&ray));
        assert_eq!(
            square.intersection(&ray),
            Some((Point3D::new(0.0, 1.0, 0.0), 2.0))
        );
    }

    #[test]
    fn test_intersection_by_missing() {
        let square = xy_square();
        let ray = Ray::new(
            Point3D::new(0.0, 0.0, 1.0),
            Vector3D::new(0.0, 3.0, -1.0),
            VOP::new(1.0),
        );
        assert!(!square.intersects(&ray));
        assert_eq!(square.intersection(&ray), None);
    }

    #[test]
    fn test_no_intersection_by_direction() {
        let square = xy_square();
        let ray = Ray::new(
            Point3D::new(0.0, 0.0, 1.0),
            Vector3D::new(1.0, 0.0, 0.0),
            VOP::new(1.0),
        );
        assert!(!square.intersects(&ray));
        assert_eq!(square.intersection(&ray), None);
    }
}
