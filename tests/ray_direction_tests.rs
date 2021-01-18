use raytracer::{Plane, Point3D, Ray, Sphere, Vector3D, SOP, VECTOR_IDENTITY, VOP};

fn air() -> VOP {
    VOP::new(1.0)
}

fn glass() -> VOP {
    VOP::new(1.5)
}

#[cfg(test)]
mod refraction_tests {
    use raytracer::Surface;

    use super::*;

    fn refractive_plane() -> Plane {
        Plane::new(
            Point3D::new(0.0, 0.0, 0.0),
            Vector3D::new(0.0, 0.0, 1.0),
            SOP::Refract,
            air(),
            glass(),
        )
    }

    #[test]
    fn test_refraction_orthogonal() {
        let mut downward_ray = Ray::new(
            Point3D::new(0.0, 0.0, 1.0),
            Vector3D::new(0.0, 0.0, -1.0),
            air(),
        );
        downward_ray.bounce_unchecked(&refractive_plane(), &Point3D::new(0.0, 0.0, 0.0));
        assert_eq!(downward_ray.direction.normalized(), Vector3D::mz());
        assert_eq!(downward_ray.origin, Point3D::new(0.0, 0.0, 0.0));
    }

    #[test]
    fn test_snells_law_from_above() {
        let original_ray = Ray::new(
            Point3D::new(1.0, 0.0, 1.0),
            Vector3D::new(-1.0, 0.0, -1.0),
            air(),
        );
        let mut ray = original_ray.clone();
        let intersection = refractive_plane()
            .geometry()
            .intersection(&original_ray)
            .unwrap();
        ray.bounce_unchecked(&refractive_plane(), &Point3D::new(0.0, 0.0, 0.0));

        // calculate via snell's law
        let normal = refractive_plane()
            .geometry()
            .normal_at(&ray.origin)
            .unwrap()
            .normalized();
        let theta_i = normal
            .dot(&(-1.0 * original_ray.direction).normalized())
            .acos();
        let theta_t = (-1.0 * normal).dot(&ray.direction.normalized()).acos();
        assert!(
            refractive_plane()
                .vop_above_at(&intersection)
                .index_of_refraction
                * theta_i.sin()
                - refractive_plane()
                    .vop_below_at(&intersection)
                    .index_of_refraction
                    * theta_t.sin()
                <= f64::EPSILON
        );
    }

    #[test]
    fn test_snells_law_from_below() {
        let original_ray = Ray::new(
            Point3D::new(0.2, 0.0, -1.0),
            Vector3D::new(-0.2, 0.0, 1.0),
            glass(),
        );
        let intersection = refractive_plane()
            .geometry()
            .intersection(&original_ray)
            .unwrap();
        let mut ray = original_ray.clone();
        ray.bounce_unchecked(&refractive_plane(), &Point3D::new(0.0, 0.0, 0.0));

        // calculate via snell's law
        let normal = refractive_plane()
            .geometry()
            .normal_at(&ray.origin)
            .unwrap()
            .normalized();
        let theta_i = normal.dot(&(original_ray.direction).normalized()).acos();
        let theta_t = normal.dot(&ray.direction.normalized());
        assert!(
            refractive_plane()
                .vop_below_at(&intersection)
                .index_of_refraction
                * theta_i.sin()
                - refractive_plane()
                    .vop_above_at(&intersection)
                    .index_of_refraction
                    * theta_t.sin()
                <= f64::EPSILON
        );
    }
}

#[cfg(test)]
mod reflection_tests {
    use super::*;

    fn reflective_sphere_air() -> Sphere {
        Sphere::new(Point3D::new(0.0, 0.0, 0.0), 1.0, SOP::Reflect, air(), air())
    }

    fn reflective_sphere_glass() -> Sphere {
        Sphere::new(
            Point3D::new(0.0, 0.0, 0.0),
            1.0,
            SOP::Reflect,
            air(),
            glass(),
        )
    }

    #[allow(dead_code)]
    fn reflective_plane() -> Plane {
        Plane::new(
            Point3D::new(0.0, 0.0, 0.0),
            Vector3D::new(0.0, 0.0, 1.0),
            SOP::Reflect,
            air(),
            air(),
        )
    }

    #[test]
    fn test_sphere_reflection_air() {
        let mut ray = Ray::new(
            Point3D::new(1.0, 0.0, 2.0),
            Vector3D::new(-1.0, 0.0, -1.0),
            air(),
        );
        ray.bounce_unchecked(&reflective_sphere_air(), &Point3D::new(0.0, 0.0, 1.0));
        assert_eq!(ray.origin, Point3D::new(0.0, 0.0, 1.0));
        assert!(
            (ray.direction.normalized() - Vector3D::new(-1.0, 0.0, 1.0).normalized())
                .length_squared()
                <= VECTOR_IDENTITY
        );
    }

    #[test]
    fn test_sphere_reflection_glass() {
        let mut ray = Ray::new(
            Point3D::new(1.0, 0.0, 2.0),
            Vector3D::new(-1.0, 0.0, -1.0),
            air(),
        );
        ray.bounce_unchecked(&reflective_sphere_glass(), &Point3D::new(0.0, 0.0, 1.0));
        assert_eq!(ray.origin, Point3D::new(0.0, 0.0, 1.0));
        assert!(
            (ray.direction.normalized() - Vector3D::new(-1.0, 0.0, 1.0).normalized())
                .length_squared()
                <= VECTOR_IDENTITY
        );
    }
}
