use {
    super::{plane::PlaneShape, Shape},
    crate::Ray,
    nalgebra::{Isometry3, Point3, Unit, Vector3},
};

pub struct DiskShape {
    plane: PlaneShape,
    pub origin: Point3<f64>,
    pub normal: Unit<Vector3<f64>>,
    pub radius: f64,
}

impl DiskShape {
    pub fn new(origin: Point3<f64>, normal: Vector3<f64>, radius: f64) -> Self {
        let unormal: Unit<Vector3<f64>> = Unit::new_normalize(normal);
        Self {
            plane: PlaneShape::new(origin, normal, None),
            origin,
            normal: unormal,
            radius,
        }
    }
}

impl Shape for DiskShape {
    fn intersection(&self, ray: &Ray) -> Option<Point3<f64>> {
        if let Some(p) = self.plane.intersection(ray) {
            if (p - self.origin).norm() <= self.radius {
                return Some(p);
            };
        }
        None
    }
    fn unchecked_normal_at(&self, _: &Point3<f64>) -> Unit<Vector3<f64>> {
        self.normal
    }
    fn contains(&self, point: &Point3<f64>) -> bool {
        // I think it's more computationally effective to not convert to local in this case.
        self.plane.contains(point) && (point - self.origin).norm() <= self.radius
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

    fn xy_circle() -> DiskShape {
        DiskShape::new(Point3::new(0.0, 0.0, 0.0), Vector3::new(0.0, 0.0, 1.0), 1.0)
    }

    #[test]
    fn test_intersection() {
        let plane = xy_circle();
        let ray = Ray {
            origin: Point3::new(0.0, 0.0, 1.0),
            direction: Vector3::new(0.0, 0.8, -1.0),
            ..Default::default()
        };
        assert!(plane.intersects(&ray));
        assert_eq!(plane.intersection(&ray), Some(Point3::new(0.0, 0.8, 0.0)));
    }
    #[test]
    fn test_no_intersection() {
        let plane = xy_circle();
        let ray = Ray {
            origin: Point3::new(0.0, 0.0, 1.0),
            direction: Vector3::new(1.2, 0.0, -1.0),
            ..Default::default()
        };
        assert!(!plane.intersects(&ray));
        assert_eq!(plane.intersection(&ray), None);
    }
}
