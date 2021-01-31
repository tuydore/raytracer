mod infiniteplane;
mod paraboloid;
mod rectangle;
mod sphere;

use crate::{Ray, TOLERANCE};
pub use {
    infiniteplane::InfinitePlaneShape,
    nalgebra::{Point3, Vector3},
    paraboloid::ParaboloidShape,
    rectangle::RectangleShape,
    sphere::SphereShape,
};

pub trait Shape {
    fn intersection(&self, ray: &Ray) -> Option<Point3<f64>>;
    fn normal_at(&self, point: &Point3<f64>) -> Option<Vector3<f64>>;
    fn contains(&self, point: &Point3<f64>) -> bool;
    fn intersects(&self, ray: &Ray) -> bool {
        self.intersection(ray).is_some()
    }
    fn origin(&self) -> Point3<f64>;
}

pub fn plane_intersects_line(
    plane_origin: &Point3<f64>,
    plane_normal: &Vector3<f64>,
    line_origin: &Point3<f64>,
    line_direction: &Vector3<f64>,
) -> Option<Point3<f64>> {
    // if line is parallel to plane
    if line_direction.dot(plane_normal).abs() <= f64::EPSILON {
        None
    } else {
        Some(
            *line_origin
                + *line_direction * plane_normal.dot(&(*plane_origin - *line_origin))
                    / plane_normal.dot(line_direction),
        )
    }
}

pub fn plane_contains_point(
    plane_origin: &Point3<f64>,
    plane_normal: &Vector3<f64>,
    point: &Point3<f64>,
) -> bool {
    plane_normal.dot(&(*plane_origin - *point)).abs() <= TOLERANCE
}

pub fn plane_intersects_ray(
    plane_origin: &Point3<f64>,
    plane_normal: &Vector3<f64>,
    ray: &Ray,
) -> Option<Point3<f64>> {
    // if the line of the ray intersects the plane
    if let Some(intersection) =
        plane_intersects_line(plane_origin, plane_normal, &ray.origin, &ray.direction)
    {
        let origin_to_intersection = intersection - ray.origin;
        // ray origin is not on plane
        if !plane_contains_point(plane_origin, plane_normal, &ray.origin) &&
            // ray is going towards plane
            origin_to_intersection.dot(&ray.direction) >= 0.0
        {
            return Some(intersection);
        }
    }
    None
}

/// Pick closest ray intersection out of all possible line intersections.
pub fn pick_closest_intersection(
    mut line_intersections: Vec<Point3<f64>>,
    ray: &Ray,
) -> Option<Point3<f64>> {
    // filter out the ray's current position
    line_intersections = line_intersections
        .into_iter()
        .filter(|p| (*p - ray.origin).norm_squared() >= TOLERANCE)
        .collect();

    // filter out line intersections that are behind the ray's direction
    line_intersections = line_intersections
        .into_iter()
        .filter(|p| (*p - ray.origin).dot(&ray.direction) >= 0.0)
        .collect();

    match line_intersections.len() {
        0 => None,
        1 => Some(line_intersections[0]),
        // QUESTION: can this be inferred from definition of `line_intersection`?
        _ => {
            let squared_distances: Vec<f64> = line_intersections
                .iter()
                .map(|p| (*p - ray.origin).norm_squared())
                .collect();
            let min_index = if squared_distances[0] < squared_distances[1] {
                0
            } else {
                1
            };
            Some(line_intersections[min_index])
        }
    }
}
