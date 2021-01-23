use super::{pick_closest_intersection, Shape};
use crate::{Point3D, Ray, Vector3D, SURFACE_INCLUSION};

pub struct SphereShape {
    pub center: Point3D,
    pub radius: f64,
}

impl SphereShape {
    /// via https://en.wikipedia.org/wiki/Line%E2%80%93sphere_intersection
    /// This assumes the line has equation x = o + d * u and the sphere is centered at c, radius r.
    /// This will take into account the direction of the ray too, so if d<0 there will be no solutions.
    /// Solution is of form d = alpha +/- sqrt(delta).
    fn intersection_components(&self, origin: &Point3D, direction: &Vector3D) -> (f64, f64) {
        let u = direction.normalized();
        let alpha = -(u.dot(&(*origin - self.center)));
        let delta =
            alpha.powi(2) - ((*origin - self.center).length_squared() - self.radius.powi(2));
        (alpha, delta)
    }

    /// Intersection of ray line with sphere
    fn line_intersection(&self, origin: &Point3D, direction: &Vector3D) -> Vec<Point3D> {
        let (alpha, delta) = self.intersection_components(origin, direction);

        // no intersection
        if delta < 0.0 {
            vec![]
        } else {
            let dn = direction.normalized();
            // single intersection
            if delta.abs() <= f64::EPSILON {
                vec![*origin + alpha * dn]
            // two intersections
            } else {
                vec![
                    *origin + (alpha - delta.sqrt()) * dn,
                    *origin + (alpha + delta.sqrt()) * dn,
                ]
            }
        }
    }
}

impl Shape for SphereShape {
    fn intersection(&self, ray: &Ray) -> Option<Point3D> {
        pick_closest_intersection(self.line_intersection(&ray.origin, &ray.direction), ray)
    }

    fn contains(&self, point: &Point3D) -> bool {
        ((self.center - *point).length() - self.radius).abs() <= SURFACE_INCLUSION
    }

    fn normal_at(&self, point: &Point3D) -> Option<Vector3D> {
        if self.contains(point) {
            Some(*point - self.center)
        } else {
            None
        }
    }

    fn origin(&self) -> Point3D {
        self.center
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::VOP;

    fn center_unit_sphere() -> SphereShape {
        SphereShape {
            center: Point3D::new(0.0, 0.0, 0.0),
            radius: 1.0,
        }
    }

    fn downwards_ray(vop: &VOP) -> Ray {
        Ray {
            origin: Point3D::new(0.0, 0.0, 10.0),
            direction: Vector3D::new(0.0, 0.0, -1.0),
            vop,
        }
    }

    #[test]
    fn no_line_intersection() {
        assert!(center_unit_sphere()
            .line_intersection(&Point3D::new(0.0, 0.0, 2.0), &Vector3D::new(0.0, 1.0, 0.0))
            .is_empty());
    }

    #[test]
    fn line_intersection_single_point() {
        assert_eq!(
            center_unit_sphere()
                .line_intersection(&Point3D::new(0.0, 0.0, 1.0), &Vector3D::new(0.0, 1.0, 0.0)),
            vec![Point3D::new(0.0, 0.0, 1.0)]
        );
    }

    #[test]
    fn line_intersection_two_points() {
        let mut result = vec![Point3D::new(0.0, 0.0, -1.0), Point3D::new(0.0, 0.0, 1.0)];

        assert_eq!(
            center_unit_sphere()
                .line_intersection(&Point3D::new(0.0, 0.0, 1.0), &Vector3D::new(0.0, 0.0, 1.0)),
            result
        );
        result.reverse();

        assert_eq!(
            center_unit_sphere()
                .line_intersection(&Point3D::new(0.0, 0.0, 1.0), &Vector3D::new(0.0, 0.0, -1.0)),
            result
        );
    }

    #[test]
    fn ray_intersection() {
        let air = VOP { ior: 1.0 };
        assert_eq!(
            center_unit_sphere()
                .intersection(&downwards_ray(&air))
                .unwrap(),
            Point3D::new(0.0, 0.0, 1.0)
        );
    }
}
