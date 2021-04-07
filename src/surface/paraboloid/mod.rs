pub mod simple;
use super::plane::PlaneShape;
pub use simple::ParaboloidBuilder;
use {
    super::{pick_closest_intersection, Shape},
    crate::{Ray, TOLERANCE},
    nalgebra::{Isometry3, Point3, Unit, Vector3},
};
// TODO: check asq and bsq are > 0
pub struct ParaboloidShape {
    plane: PlaneShape,
    pub origin: Point3<f64>,
    pub normal: Unit<Vector3<f64>>,
    pub orientation: Unit<Vector3<f64>>, // coresponds to axis of asq
    pub asq: f64,
    pub bsq: f64,
}

impl ParaboloidShape {
    pub fn new(
        origin: Point3<f64>,
        normal: Vector3<f64>,
        orientation: Vector3<f64>,
        asq: f64,
        bsq: f64,
    ) -> Self {
        Self {
            plane: PlaneShape::new(origin, normal, Some(orientation)),
            origin,
            normal: Unit::new_normalize(normal),
            orientation: Unit::new_normalize(orientation),
            asq,
            bsq,
        }
    }

    /// Returns (alpha, delta) where solution is alpha +/- delta.sqrt().
    /// Assumes paraboloid is in Z direction, with (asq, bsq) mapped to X and Y axes.
    fn line_intersection_quadratic(
        &self,
        origin: &Point3<f64>,
        direction: &Vector3<f64>,
    ) -> (f64, f64) {
        // quadratic terms
        let a = direction.x.powi(2) / self.asq + direction.y.powi(2) / self.bsq;
        let b = 2.0 * direction.x * origin.x / self.asq + 2.0 * direction.y * origin.y / self.bsq
            - direction.z;
        let c = origin.x.powi(2) / self.asq + origin.y.powi(2) / self.bsq - origin.z;

        let alpha = -b / (2.0 * a);
        let delta = (b.powi(2) - 4.0 * a * c) / (4.0 * a.powi(2));
        (alpha, delta)
    }

    /// Returns intersection points
    fn line_intersection(
        &self,
        origin: &Point3<f64>,
        direction: &Vector3<f64>,
    ) -> Vec<Point3<f64>> {
        let (alpha, delta) = self.line_intersection_quadratic(origin, direction);

        // no solution
        if delta <= 0.0 {
            return vec![];
        // single solution
        } else if delta <= f64::EPSILON {
            return vec![*origin + alpha * *direction];
        } else {
            return vec![
                *origin + (alpha - delta.sqrt()) * *direction,
                *origin + (alpha + delta.sqrt()) * *direction,
            ];
        }
    }
}

impl Shape for ParaboloidShape {
    fn contains(&self, point: &Point3<f64>) -> bool {
        let point: Point3<f64> = self.to_local() * point;
        (point.x.powi(2) / self.asq + point.y.powi(2) / self.bsq - point.z).abs() <= TOLERANCE
    }
    fn origin(&self) -> &Point3<f64> {
        &self.origin
    }
    fn intersection(&self, ray: &Ray) -> Option<Point3<f64>> {
        let origin: Point3<f64> = self.to_local() * ray.origin;
        let direction: Vector3<f64> = self.to_local() * ray.direction;
        let intersection =
            pick_closest_intersection(self.line_intersection(&origin, &direction), ray);
        intersection.map(|p| self.to_global() * p).to_owned()
    }
    fn unchecked_normal_at(&self, point: &Point3<f64>) -> Unit<Vector3<f64>> {
        let point: Point3<f64> = self.to_local() * point;
        let rx: Vector3<f64> = Vector3::new(1.0, 0.0, 2.0 * point.x / self.asq);
        let ry: Vector3<f64> = Vector3::new(0.0, 1.0, 2.0 * point.y / self.bsq);
        Unit::new_normalize(self.to_global() * rx.cross(&ry))
    }
    fn to_local(&self) -> &Isometry3<f64> {
        self.plane.to_local()
    }
    fn to_global(&self) -> &Isometry3<f64> {
        self.plane.to_global()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn center_paraboloid() -> ParaboloidShape {
        ParaboloidShape::new(
            Point3::new(0.0, 0.0, 0.0),
            Vector3::z(),
            Vector3::x(),
            1.0,
            1.0,
        )
    }

    #[test]
    fn parabola_positions() {
        let p = center_paraboloid();
        assert!(p.contains(&Point3::new(0.0, 0.0, 0.0)));
        assert!(p.contains(&Point3::new(1.0, 1.0, 2.0)));
        assert!(p.contains(&Point3::new(-1.0, 1.0, 2.0)));
        assert!(p.contains(&Point3::new(1.0, -1.0, 2.0)));
        assert!(p.contains(&Point3::new(-1.0, -1.0, 2.0)));
        assert_eq!(
            p.unchecked_normal_at(&Point3::new(0.0, 0.0, 0.0)),
            Unit::new_normalize(Vector3::z())
        );
    }

    #[test]
    fn ray_intersections() {
        let p = center_paraboloid();
        let r = Ray {
            origin: Point3::new(0.0, -10.0, 2.0),
            direction: Vector3::y(),
            ..Default::default()
        };

        let intersections = p.line_intersection(&r.origin, &r.direction);
        assert!((intersections[0] - Point3::new(0.0, -(2.0f64.sqrt()), 2.0)).norm() <= TOLERANCE);
        assert!((intersections[1] - Point3::new(0.0, 2.0f64.sqrt(), 2.0)).norm() <= TOLERANCE);
    }
}
