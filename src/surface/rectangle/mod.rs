pub mod simple;
pub mod textured;
use super::plane::PlaneShape;
use {
    super::Shape,
    crate::ray::Ray,
    nalgebra::{Isometry3, Point3, Unit, Vector3},
};
pub use {simple::RectangleBuilder, textured::TexturedRectangleBuilder};

pub struct RectangleShape {
    plane: PlaneShape,
    pub origin: Point3<f64>,
    pub normal: Unit<Vector3<f64>>,
    pub orientation: Unit<Vector3<f64>>,
    pub size: [f64; 2], // orientation is along 1st size dimension
}

impl RectangleShape {
    pub fn new(
        origin: Point3<f64>,
        normal: Vector3<f64>,
        orientation: Vector3<f64>,
        size: [f64; 2],
    ) -> Self {
        Self {
            plane: PlaneShape::new(origin, normal, Some(orientation)),
            origin,
            normal: Unit::new_normalize(normal),
            orientation: Unit::new_normalize(orientation),
            size,
        }
    }
}

impl Shape for RectangleShape {
    fn intersection(&self, ray: &Ray) -> Option<Point3<f64>> {
        if let Some(p) = self.plane.intersection(ray) {
            if self.contains(&p) {
                return Some(p);
            }
        }
        None
    }
    fn unchecked_normal_at(&self, _: &Point3<f64>) -> Unit<Vector3<f64>> {
        self.normal
    }
    fn contains(&self, point: &Point3<f64>) -> bool {
        // check if point is on plane
        if !self.plane.contains(point) {
            return false;
        }

        // check if point is in rectangle
        let from_origin = *point - self.origin;
        let l0 = from_origin.dot(&self.orientation).abs();
        let l1 = from_origin.dot(&self.normal.cross(&self.orientation)).abs();
        l0 <= self.size[0] / 2.0 && l1 <= self.size[1] / 2.0
    }
    fn origin(&self) -> &Point3<f64> {
        &self.origin
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

    fn xy_square() -> RectangleShape {
        RectangleShape::new(
            Point3::new(0.0, 0.0, 0.0),
            Vector3::z(),
            Vector3::y(),
            [2.0, 2.0],
        )
    }

    #[test]
    fn test_intersection() {
        let square = xy_square();
        let ray = Ray {
            origin: Point3::new(0.0, 0.0, 1.0),
            direction: Vector3::new(0.0, 1.0, -1.0),
            ..Default::default()
        };
        assert!(square.intersects(&ray));
        assert_eq!(square.intersection(&ray), Some(Point3::new(0.0, 1.0, 0.0)));
    }

    #[test]
    fn test_intersection_by_missing() {
        let square = xy_square();
        let ray = Ray {
            origin: Point3::new(0.0, 0.0, 1.0),
            direction: Vector3::new(0.0, 3.0, -1.0),
            ..Default::default()
        };
        assert!(!square.intersects(&ray));
        assert_eq!(square.intersection(&ray), None);
    }

    #[test]
    fn test_no_intersection_by_direction() {
        let square = xy_square();
        let ray = Ray {
            origin: Point3::new(0.0, 0.0, 1.0),
            direction: Vector3::new(1.0, 0.0, 0.0),
            ..Default::default()
        };
        assert!(!square.intersects(&ray));
        assert_eq!(square.intersection(&ray), None);
    }
}
