use core::f64;

use crate::{Point3D, Ray, Shape, Vector3D, TOLERANCE};

use super::pick_closest_intersection;
// TODO: check asq and bsq are > 0
pub struct ParaboloidShape {
    pub x0: f64,
    pub y0: f64,
    pub z0: f64,
    pub asq: f64,
    pub bsq: f64,
}

impl ParaboloidShape {
    /// Returns (alpha, delta) where solution is alpha +/- delta.sqrt()
    fn line_intersection_quadratic(&self, origin: &Point3D, direction: &Vector3D) -> (f64, f64) {
        let alpha_x = origin.x - self.x0;
        let alpha_y = origin.y - self.y0;

        // quadratic terms
        let a = direction.x.powi(2) / self.asq + direction.y.powi(2) / self.bsq;
        let b = 2.0 * direction.x * alpha_x / self.asq + 2.0 * direction.y * alpha_y / self.bsq
            - direction.z;
        let c = self.z0 + alpha_x.powi(2) / self.asq + alpha_y.powi(2) / self.bsq - origin.z;

        let alpha = -b / (2.0 * a);
        let delta = (b.powi(2) - 4.0 * a * c) / (4.0 * a.powi(2));
        (alpha, delta)
    }

    /// Returns intersection points
    fn line_intersection(&self, origin: &Point3D, direction: &Vector3D) -> Vec<Point3D> {
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
    fn contains(&self, point: &Point3D) -> bool {
        ((point.x - self.x0).powi(2) / self.asq + (point.y - self.y0).powi(2) / self.bsq + self.z0
            - point.z)
            .abs()
            <= TOLERANCE
    }

    fn origin(&self) -> Point3D {
        Point3D::new(self.x0, self.y0, self.z0)
    }

    fn intersection(&self, ray: &Ray) -> Option<Point3D> {
        pick_closest_intersection(self.line_intersection(&ray.origin, &ray.direction), ray)
    }

    fn normal_at(&self, point: &Point3D) -> Option<Vector3D> {
        if self.contains(point) {
            let rx = Vector3D::new(1.0, 0.0, 2.0 * (point.x - self.x0) / self.asq);
            let ry = Vector3D::new(0.0, 1.0, 2.0 * (point.y - self.y0) / self.bsq);
            Some(rx.cross(&ry))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::VOP;
    use std::sync::Arc;

    fn center_paraboloid() -> ParaboloidShape {
        ParaboloidShape {
            x0: 0.0,
            y0: 0.0,
            z0: 0.0,
            asq: 1.0,
            bsq: 1.0,
        }
    }

    #[test]
    fn parabola_positions() {
        let p = center_paraboloid();
        assert!(p.contains(&Point3D::new(0.0, 0.0, 0.0)));
        assert!(p.contains(&Point3D::new(1.0, 1.0, 2.0)));
        assert!(p.contains(&Point3D::new(-1.0, 1.0, 2.0)));
        assert!(p.contains(&Point3D::new(1.0, -1.0, 2.0)));
        assert!(p.contains(&Point3D::new(-1.0, -1.0, 2.0)));
        assert_eq!(
            p.normal_at(&Point3D::new(0.0, 0.0, 0.0)),
            Some(Vector3D::pz())
        );
    }

    #[test]
    fn ray_intersections() {
        let p = center_paraboloid();
        let air = Arc::new(VOP {
            ior: 1.0,
            abs: [0.0; 3],
        });
        let r = Ray {
            origin: Point3D::new(0.0, -10.0, 2.0),
            direction: Vector3D::py(),
            vop: air,
            abs: [0.0; 3],
        };

        assert_eq!(
            p.line_intersection(&r.origin, &r.direction),
            vec![
                Point3D::new(0.0, -(2.0f64.sqrt()), 2.0),
                Point3D::new(0.0, 2.0f64.sqrt(), 2.0)
            ]
        )
    }
}
