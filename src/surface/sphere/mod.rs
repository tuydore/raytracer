pub mod simple;
pub use simple::SphereBuilder;
use {
    super::{pick_closest_intersection, plane::PlaneShape, Shape},
    crate::{Ray, TOLERANCE},
    nalgebra::{Isometry3, Point3, Unit, Vector3},
};

pub struct SphereShape {
    equator_plane: PlaneShape,
    pub center: Point3<f64>,
    pub radius: f64,
}

impl SphereShape {
    fn new(
        center: Point3<f64>,
        radius: f64,
        north: Option<Vector3<f64>>,
        greenwich: Option<Vector3<f64>>,
    ) -> Self {
        let north: Vector3<f64> = north.unwrap_or_else(Vector3::z);
        let equator_plane = PlaneShape::new(center, north, greenwich);
        Self {
            center,
            radius,
            equator_plane,
        }
    }
}

impl SphereShape {
    /// via https://en.wikipedia.org/wiki/Line%E2%80%93sphere_intersection
    /// This assumes the line has equation x = o + d * u and the sphere is centered at c, radius r.
    /// This will take into account the direction of the ray too, so if d<0 there will be no solutions.
    /// Solution is of form d = alpha +/- sqrt(delta).
    fn intersection_components(
        &self,
        origin: &Point3<f64>,
        direction: &Vector3<f64>,
    ) -> (f64, f64) {
        let u = direction.normalize();
        let alpha = -(u.dot(&(*origin - self.center)));
        let delta = alpha.powi(2) - ((*origin - self.center).norm_squared() - self.radius.powi(2));
        (alpha, delta)
    }

    /// Intersection of ray line with sphere
    fn line_intersection(
        &self,
        origin: &Point3<f64>,
        direction: &Vector3<f64>,
    ) -> Vec<Point3<f64>> {
        let (alpha, delta) = self.intersection_components(origin, direction);

        // no intersection
        if delta < 0.0 {
            vec![]
        } else {
            let dn = direction.normalize();
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
    fn intersection(&self, ray: &Ray) -> Option<Point3<f64>> {
        pick_closest_intersection(self.line_intersection(&ray.origin, &ray.direction), ray)
    }
    fn contains(&self, point: &Point3<f64>) -> bool {
        ((self.center - *point).norm() - self.radius).abs() <= TOLERANCE
    }
    fn unchecked_normal_at(&self, point: &Point3<f64>) -> Unit<Vector3<f64>> {
        Unit::new_normalize(*point - self.center)
    }
    fn origin(&self) -> &Point3<f64> {
        &self.center
    }
    fn to_local(&self) -> &Isometry3<f64> {
        self.equator_plane.to_local()
    }
    fn to_global(&self) -> &Isometry3<f64> {
        self.equator_plane.to_global()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn center_unit_sphere() -> SphereShape {
        SphereShape::new(Point3::origin(), 1.0, None, None)
    }

    #[test]
    fn no_line_intersection() {
        assert!(center_unit_sphere()
            .line_intersection(&Point3::new(0.0, 0.0, 2.0), &Vector3::new(0.0, 1.0, 0.0))
            .is_empty());
    }

    #[test]
    fn line_intersection_single_point() {
        assert_eq!(
            center_unit_sphere()
                .line_intersection(&Point3::new(0.0, 0.0, 1.0), &Vector3::new(0.0, 1.0, 0.0)),
            vec![Point3::new(0.0, 0.0, 1.0)]
        );
    }

    #[test]
    fn line_intersection_two_points() {
        let mut result = vec![Point3::new(0.0, 0.0, -1.0), Point3::new(0.0, 0.0, 1.0)];

        assert_eq!(
            center_unit_sphere()
                .line_intersection(&Point3::new(0.0, 0.0, 1.0), &Vector3::new(0.0, 0.0, 1.0)),
            result
        );
        result.reverse();

        assert_eq!(
            center_unit_sphere()
                .line_intersection(&Point3::new(0.0, 0.0, 1.0), &Vector3::new(0.0, 0.0, -1.0)),
            result
        );
    }

    #[test]
    fn ray_intersection() {
        let ray = Ray {
            origin: Point3::new(0.0, 0.0, 10.0),
            direction: Vector3::new(0.0, 0.0, -1.0),
            ..Default::default()
        };
        assert_eq!(
            center_unit_sphere().intersection(&ray).unwrap(),
            Point3::new(0.0, 0.0, 1.0)
        );
    }
}
