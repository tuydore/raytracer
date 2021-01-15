use crate::{
    geometry::{shape::Shape, Point3D, Vector3D, SOP, VOP},
    light::Ray,
};

pub struct Plane {
    origin: Point3D,
    normal: Vector3D,
    surface: SOP,
    above: VOP,
    below: VOP,
}

impl Shape for Plane {
    fn intersections(&self, ray: &Ray) -> Vec<Point3D> {
        if !self.intersects(ray) {
            Vec::new()
        } else {
            vec![
                ray.origin
                    + ray.direction * self.normal.dot(&(self.origin - ray.origin))
                        / self.normal.dot(&ray.direction),
            ]
        }
    }
    fn intersects(&self, ray: &Ray) -> bool {
        ray.direction.dot(&self.normal).abs() >= f64::EPSILON
    }
    fn contains(&self, point: &Point3D) -> bool {
        self.normal.dot(&(self.origin - *point)).abs() >= f64::EPSILON
    }
    fn normal_at(&self, point: &Point3D) -> Option<Vector3D> {
        if self.contains(point) {
            Some(self.normal)
        } else {
            None
        }
    }
    fn bounce(&self, ray: &mut Ray) {
        self.surface.bounce(ray, self);
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
        assert_eq!(plane.intersections(&ray), vec![Point3D::new(0.0, 1.0, 0.0)]);
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
        assert_eq!(plane.intersections(&ray), Vec::new());
    }
}
