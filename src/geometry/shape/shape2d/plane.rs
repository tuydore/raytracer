use crate::{
    geometry::{shape::Shape, Point3D, Vector3D, SOP, VOP},
    ray::{BounceResult, Ray},
    SURFACE_INCLUSION,
};

pub struct Plane {
    pub(crate) origin: Point3D,
    pub(crate) normal: Vector3D,
    pub(crate) surface: SOP,
    pub(crate) above: VOP,
    pub(crate) below: VOP,
}

impl Plane {
    pub fn new(origin: Point3D, normal: Vector3D, surface: SOP, above: VOP, below: VOP) -> Self {
        Self {
            origin,
            normal,
            surface,
            above,
            below,
        }
    }

    /// Returns the intersection of a line and the plane.
    pub(crate) fn line_intersection(
        &self,
        origin: &Point3D,
        direction: &Vector3D,
    ) -> Option<Point3D> {
        // if line is parallel to plane
        if direction.dot(&self.normal).abs() <= f64::EPSILON {
            None
        } else {
            Some(
                *origin
                    + *direction * self.normal.dot(&(self.origin - *origin))
                        / self.normal.dot(direction),
            )
        }
    }
}

impl Shape for Plane {
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
        self.normal.dot(&(self.origin - *point)).abs() <= SURFACE_INCLUSION
    }

    fn normal_at(&self, point: &Point3D) -> Option<Vector3D> {
        if self.contains(point) {
            Some(self.normal)
        } else {
            None
        }
    }

    fn bounce(&self, ray: &mut Ray) -> BounceResult {
        self.surface.bounce(ray, self)
    }

    fn vop_above(&self) -> &VOP {
        &self.above
    }

    fn vop_below(&self) -> &VOP {
        &self.below
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn xy_plane() -> Plane {
        Plane {
            origin: Point3D::new(0.0, 0.0, 0.0),
            normal: Vector3D::new(0.0, 0.0, 1.0),
            surface: SOP::Reflect,
            above: VOP::new(1.0),
            below: VOP::new(1.5),
        }
    }

    #[test]
    fn test_intersection() {
        let plane = xy_plane();
        let ray = Ray::new(
            Point3D::new(0.0, 0.0, 1.0),
            Vector3D::new(0.0, 1.0, -1.0),
            VOP::new(1.0),
        );
        assert!(plane.intersects(&ray));
        assert_eq!(
            plane.intersection(&ray),
            Some((Point3D::new(0.0, 1.0, 0.0), 2.0))
        );
    }
    #[test]
    fn test_no_intersection() {
        let plane = xy_plane();
        let ray = Ray::new(
            Point3D::new(0.0, 0.0, 1.0),
            Vector3D::new(1.0, 0.0, 0.0),
            VOP::new(1.0),
        );
        assert!(!plane.intersects(&ray));
        assert_eq!(plane.intersection(&ray), None);
    }
}
