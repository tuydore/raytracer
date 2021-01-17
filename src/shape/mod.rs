mod infiniteplane;
mod rectangle;
mod sphere;

use crate::{Point3D, Ray, Vector3D, SURFACE_INCLUSION};
pub use {infiniteplane::InfinitePlaneShape, rectangle::RectangleShape, sphere::SphereShape};

pub trait Shape {
    fn intersection(&self, ray: &Ray) -> Option<Point3D>;
    fn normal_at(&self, point: &Point3D) -> Option<Vector3D>;
    fn contains(&self, point: &Point3D) -> bool;
    fn intersects(&self, ray: &Ray) -> bool {
        self.intersection(ray).is_some()
    }
    fn origin(&self) -> &Point3D;
}

pub fn plane_intersects_line(
    plane_origin: &Point3D,
    plane_normal: &Vector3D,
    line_origin: &Point3D,
    line_direction: &Vector3D,
) -> Option<Point3D> {
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
    plane_origin: &Point3D,
    plane_normal: &Vector3D,
    point: &Point3D,
) -> bool {
    plane_normal.dot(&(*plane_origin - *point)).abs() <= SURFACE_INCLUSION
}

pub fn plane_intersects_ray(
    plane_origin: &Point3D,
    plane_normal: &Vector3D,
    ray: &Ray,
) -> Option<Point3D> {
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
