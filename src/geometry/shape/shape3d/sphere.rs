use crate::{
    geometry::{shape::Shape, Point3D, Vector3D, SOP, VOP},
    ray::{BounceResult, Ray},
    SURFACE_INCLUSION,
};

pub struct Sphere {
    center: Point3D,
    radius: f64,
    surface: SOP,
    above: VOP,
    below: VOP,
}

impl Sphere {
    pub fn new(center: Point3D, radius: f64, surface: SOP, above: VOP, below: VOP) -> Self {
        Self {
            center,
            radius,
            surface,
            above,
            below,
        }
    }
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

impl Shape for Sphere {
    fn intersection(&self, ray: &Ray) -> Option<(Point3D, f64)> {
        // get all line intersections
        let mut line_intersections = self.line_intersection(&ray.origin, &ray.direction);

        // filter out the ray's current position
        line_intersections = line_intersections
            .into_iter()
            .filter(|p| *p != ray.origin)
            .collect();

        // filter out line intersections that are behind the ray's direction
        line_intersections = line_intersections
            .into_iter()
            .filter(|p| (*p - ray.origin).dot(&ray.direction) >= 0.0)
            .collect();

        match line_intersections.len() {
            0 => None,
            1 => Some((
                line_intersections[0],
                (line_intersections[0] - ray.origin).length_squared(),
            )),
            // QUESTION: can this be inferred from definition of `line_intersection`?
            2 => {
                let squared_distances: Vec<f64> = line_intersections
                    .iter()
                    .map(|p| (*p - ray.origin).length_squared())
                    .collect();
                let min_index = if squared_distances[0] < squared_distances[1] {
                    0
                } else {
                    1
                };
                Some((line_intersections[min_index], squared_distances[min_index]))
            },
            _ => panic!("A line cannot intersect a sphere in more than 2 points. You are either using non-Euclidean geometry or something is majorly screwed up.")
        }
    }
    fn intersects(&self, ray: &Ray) -> bool {
        self.intersection_components(&ray.origin, &ray.direction).1 >= 0.0
            && self.intersection(&ray).is_some()
    }
    fn contains(&self, point: &Point3D) -> bool {
        ((self.center - *point).length() - self.radius).abs() <= SURFACE_INCLUSION
        //f64::EPSILON
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
    fn bounce(&self, ray: &mut Ray) -> BounceResult {
        self.surface.bounce(ray, self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn center_unit_sphere() -> Sphere {
        Sphere::new(
            Point3D::new(0.0, 0.0, 0.0),
            1.0,
            SOP::Reflect,
            VOP::new(1.0),
            VOP::new(1.0),
        )
    }

    fn downwards_ray() -> Ray {
        Ray::new(
            Point3D::new(0.0, 0.0, 10.0),
            Vector3D::new(0.0, 0.0, -1.0),
            VOP::new(1.0),
        )
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
        assert_eq!(
            center_unit_sphere().intersection(&downwards_ray()).unwrap(),
            (Point3D::new(0.0, 0.0, 1.0), 81.0)
        );
    }
}
