use raytracer::{
    surface::plane::Plane, surface::sphere::Sphere, Point3D, Ray, Vector3D, SOP, VECTOR_IDENTITY,
    VOP,
};
use std::sync::Arc;

#[cfg(test)]
mod refraction_tests {
    use raytracer::{shape::InfinitePlaneShape, Surface};

    use super::*;

    fn refractive_plane(air: Arc<VOP>, glass: Arc<VOP>) -> Plane {
        Plane {
            geometry: InfinitePlaneShape {
                origin: Point3D::new(0.0, 0.0, 0.0),
                normal: Vector3D::new(0.0, 0.0, 1.0),
            },
            sop: SOP::Refract,
            vop_above: air,
            vop_below: glass,
        }
    }

    #[test]
    fn test_refraction_orthogonal() {
        let air = Arc::new(VOP { ior: 1.0 });
        let glass = Arc::new(VOP { ior: 1.5 });
        let plane = refractive_plane(air.clone(), glass);
        let mut downward_ray = Ray {
            origin: Point3D::new(0.0, 0.0, 1.0),
            direction: Vector3D::new(0.0, 0.0, -1.0),
            vop: air,
        };
        downward_ray.bounce_unchecked(&plane, &Point3D::new(0.0, 0.0, 0.0));
        assert_eq!(downward_ray.direction.normalized(), Vector3D::mz());
        assert_eq!(downward_ray.origin, Point3D::new(0.0, 0.0, 0.0));
    }

    #[test]
    fn test_snells_law_from_above() {
        let air = Arc::new(VOP { ior: 1.0 });
        let glass = Arc::new(VOP { ior: 1.5 });
        let plane = refractive_plane(air.clone(), glass);
        let original_ray = Ray {
            origin: Point3D::new(1.0, 0.0, 1.0),
            direction: Vector3D::new(-1.0, 0.0, -1.0),
            vop: air,
        };
        let mut ray = original_ray.clone();
        let intersection = plane.geometry().intersection(&original_ray).unwrap();
        ray.bounce_unchecked(&plane, &Point3D::new(0.0, 0.0, 0.0));

        // calculate via snell's law
        let normal = plane
            .geometry()
            .normal_at(&ray.origin)
            .unwrap()
            .normalized();
        let theta_i = normal
            .dot(&(-1.0 * original_ray.direction).normalized())
            .acos();
        let theta_t = (-1.0 * normal).dot(&ray.direction.normalized()).acos();
        assert!(
            plane.vop_above_at(&intersection).ior * theta_i.sin()
                - plane.vop_below_at(&intersection).ior * theta_t.sin()
                <= f64::EPSILON
        );
    }

    #[test]
    fn test_snells_law_from_below() {
        let air = Arc::new(VOP { ior: 1.0 });
        let glass = Arc::new(VOP { ior: 1.5 });
        let plane = refractive_plane(air, glass.clone());
        let original_ray = Ray {
            origin: Point3D::new(0.2, 0.0, 1.0),
            direction: Vector3D::new(-0.2, 0.0, -1.0),
            vop: glass,
        };
        let intersection = plane.geometry().intersection(&original_ray).unwrap();
        let mut ray = original_ray.clone();
        ray.bounce_unchecked(&plane, &Point3D::new(0.0, 0.0, 0.0));

        // calculate via snell's law
        let normal = plane
            .geometry()
            .normal_at(&ray.origin)
            .unwrap()
            .normalized();
        let theta_i = normal.dot(&(original_ray.direction).normalized()).acos();
        let theta_t = normal.dot(&ray.direction.normalized());
        assert!(
            plane.vop_below_at(&intersection).ior * theta_i.sin()
                - plane.vop_above_at(&intersection).ior * theta_t.sin()
                <= f64::EPSILON
        );
    }
}

#[cfg(test)]
mod reflection_tests {
    use raytracer::shape::{InfinitePlaneShape, SphereShape};

    use super::*;

    fn reflective_sphere_air(air: Arc<VOP>) -> Sphere {
        Sphere {
            geometry: SphereShape {
                center: Point3D::new(0.0, 0.0, 0.0),
                radius: 1.0,
            },
            sop: SOP::Reflect,
            vop_above: air.clone(),
            vop_below: air,
        }
    }

    fn reflective_sphere_glass(air: Arc<VOP>, glass: Arc<VOP>) -> Sphere {
        Sphere {
            geometry: SphereShape {
                center: Point3D::new(0.0, 0.0, 0.0),
                radius: 1.0,
            },
            sop: SOP::Reflect,
            vop_above: air,
            vop_below: glass,
        }
    }

    #[allow(dead_code)]
    fn reflective_plane(air: Arc<VOP>) -> Plane {
        Plane {
            geometry: InfinitePlaneShape {
                origin: Point3D::new(0.0, 0.0, 0.0),
                normal: Vector3D::new(0.0, 0.0, 1.0),
            },
            sop: SOP::Reflect,
            vop_above: air.clone(),
            vop_below: air,
        }
    }

    #[test]
    fn test_sphere_reflection_air() {
        let air = Arc::new(VOP { ior: 1.0 });
        let sphere = reflective_sphere_air(air.clone());
        let mut ray = Ray {
            origin: Point3D::new(1.0, 0.0, 2.0),
            direction: Vector3D::new(-1.0, 0.0, -1.0),
            vop: air,
        };
        ray.bounce_unchecked(&sphere, &Point3D::new(0.0, 0.0, 1.0));
        assert_eq!(ray.origin, Point3D::new(0.0, 0.0, 1.0));
        assert!(
            (ray.direction.normalized() - Vector3D::new(-1.0, 0.0, 1.0).normalized())
                .length_squared()
                <= VECTOR_IDENTITY
        );
    }

    #[test]
    fn test_sphere_reflection_glass() {
        let air = Arc::new(VOP { ior: 1.0 });
        let glass = Arc::new(VOP { ior: 1.5 });
        let sphere = reflective_sphere_glass(air.clone(), glass);
        let mut ray = Ray {
            origin: Point3D::new(1.0, 0.0, 2.0),
            direction: Vector3D::new(-1.0, 0.0, -1.0),
            vop: air,
        };
        ray.bounce_unchecked(&sphere, &Point3D::new(0.0, 0.0, 1.0));
        assert_eq!(ray.origin, Point3D::new(0.0, 0.0, 1.0));
        assert!(
            (ray.direction.normalized() - Vector3D::new(-1.0, 0.0, 1.0).normalized())
                .length_squared()
                <= VECTOR_IDENTITY
        );
    }
}
