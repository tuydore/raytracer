use crate::{
    geometry::{shape::Shape, Point3D, Vector3D, SOP, VOP},
    light::Ray,
};

pub struct Sphere {
    center: Point3D,
    radius: f64,
    surface: SOP,
    above: VOP,
    below: VOP,
}

impl Sphere {
    /// via https://en.wikipedia.org/wiki/Line%E2%80%93sphere_intersection
    /// This assumes the line has equation x = o + d * u and the sphere is centered at c, radius r.
    /// This will take into account the direction of the ray too, so if d<0 there will be no solutions.
    /// Solution is of form d = alpha +/- sqrt(delta).
    fn intersection_components(&self, ray: &Ray) -> (f64, f64) {
        let u = ray.direction.normalized();
        let alpha = -(u.dot(&(ray.origin - self.center)));
        let delta =
            alpha.powi(2) - ((ray.origin - self.center).length_squared() - self.radius.powi(2));
        (alpha, delta)
    }

    /// Avoid duplicate generation of alpha and delta in Shape implementation.
    fn intersection_aux(alpha: f64, delta: f64) -> bool {
        delta >= 0.0 && alpha + delta.sqrt() >= 0.0
    }
}

impl Shape for Sphere {
    fn intersections(&self, ray: &Ray) -> Vec<Point3D> {
        let (alpha, delta) = self.intersection_components(ray);
        if !Self::intersection_aux(alpha, delta) {
            return Vec::new();
        }
        let u = ray.direction.normalized();
        if delta.abs() <= f64::EPSILON {
            vec![ray.origin + alpha * u]
        } else {
            vec![
                ray.origin + (alpha + delta.sqrt()) * u,
                ray.origin - (alpha + delta.sqrt()) * u,
            ]
        }
    }
    fn intersects(&self, ray: &Ray) -> bool {
        let (alpha, delta) = self.intersection_components(ray);
        Self::intersection_aux(alpha, delta)
    }
    fn contains(&self, point: &Point3D) -> bool {
        ((self.center - *point).length() - self.radius).abs() <= f64::EPSILON
    }
    fn normal_at(&self, point: &Point3D) -> Option<Vector3D> {
        if self.contains(point) {
            Some(*point - self.center)
        } else {
            None
        }
    }
    fn vop_above(&self) -> &VOP {
        &self.above
    }
    fn vop_below(&self) -> &VOP {
        &self.below
    }
    fn bounce(&self, ray: &mut Ray) {
        self.surface.bounce(ray, self)
    }
}
