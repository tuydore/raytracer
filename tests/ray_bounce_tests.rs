use raytracer::{ray::BounceResult, Plane, Point3D, Ray, Sphere, Vector3D, SOP, VOP};

fn downwards_ray() -> Ray {
    Ray::new(
        Point3D::new(0.0, 0.0, 10.0),
        Vector3D::new(0.0, 0.0, -1.0),
        VOP::new(1.0),
    )
}

#[cfg(test)]
mod sphere_tests {
    use super::*;

    fn light_sphere() -> Box<Sphere> {
        Box::new(Sphere::new(
            Point3D::new(0.0, 0.0, 0.0),
            1.0,
            SOP::Light(255, 255, 255),
            VOP::new(1.0),
            VOP::new(1.0),
        ))
    }

    fn dark_sphere() -> Box<Sphere> {
        Box::new(Sphere::new(
            Point3D::new(0.0, 0.0, 0.0),
            1.0,
            SOP::Dark,
            VOP::new(1.0),
            VOP::new(1.0),
        ))
    }

    fn reflective_sphere() -> Box<Sphere> {
        Box::new(Sphere::new(
            Point3D::new(0.0, 0.0, 0.0),
            1.0,
            SOP::Reflect,
            VOP::new(1.0),
            VOP::new(1.0),
        ))
    }

    #[test]
    fn counted_ray() {
        let mut ray = downwards_ray();
        assert_eq!(
            ray.launch(&[light_sphere()]),
            BounceResult::Count(255, 255, 255)
        );
    }

    #[test]
    fn killed_ray_at_dark_plane() {
        let mut ray = downwards_ray();
        assert_eq!(ray.launch(&[dark_sphere()]), BounceResult::Kill);
    }

    #[test]
    fn killed_ray_no_more_intersections() {
        let mut ray = downwards_ray();
        assert_eq!(ray.launch(&[reflective_sphere()]), BounceResult::Kill);
    }
}

#[cfg(test)]
mod plane_tests {
    use super::*;

    fn light_plane() -> Box<Plane> {
        Box::new(Plane::new(
            Point3D::new(0.0, 0.0, 0.0),
            Vector3D::new(0.0, 0.0, 1.0),
            SOP::Light(255, 255, 255),
            VOP::new(1.0),
            VOP::new(1.0),
        ))
    }

    fn dark_plane() -> Box<Plane> {
        Box::new(Plane::new(
            Point3D::new(0.0, 0.0, 0.0),
            Vector3D::new(0.0, 0.0, 1.0),
            SOP::Dark,
            VOP::new(1.0),
            VOP::new(1.0),
        ))
    }

    fn reflective_plane() -> Box<Plane> {
        Box::new(Plane::new(
            Point3D::new(0.0, 0.0, 0.0),
            Vector3D::new(0.0, 0.0, 1.0),
            SOP::Reflect,
            VOP::new(1.0),
            VOP::new(1.0),
        ))
    }

    #[test]
    fn counted_ray() {
        let mut ray = downwards_ray();
        assert_eq!(
            ray.launch(&[light_plane()]),
            BounceResult::Count(255, 255, 255)
        );
    }

    #[test]
    fn killed_ray_at_dark_plane() {
        let mut ray = downwards_ray();
        assert_eq!(ray.launch(&[dark_plane()]), BounceResult::Kill);
    }

    #[test]
    fn killed_ray_no_more_intersections() {
        let mut ray = downwards_ray();
        assert_eq!(ray.launch(&[reflective_plane()]), BounceResult::Kill);
    }
}

#[cfg(test)]
mod rectangle_tests {
    use raytracer::Rectangle;

    use super::*;

    fn light_rectangle() -> Box<Rectangle> {
        Box::new(Rectangle::new(
            Point3D::new(0.0, 0.0, 0.0),
            Vector3D::new(0.0, 0.0, 1.0),
            Vector3D::new(0.0, 1.0, 0.0),
            (2.0, 2.0),
            SOP::Light(255, 255, 255),
            VOP::new(1.0),
            VOP::new(1.0),
        ))
    }

    fn dark_rectangle() -> Box<Rectangle> {
        Box::new(Rectangle::new(
            Point3D::new(0.0, 0.0, 0.0),
            Vector3D::new(0.0, 0.0, 1.0),
            Vector3D::new(0.0, 1.0, 0.0),
            (2.0, 2.0),
            SOP::Dark,
            VOP::new(1.0),
            VOP::new(1.0),
        ))
    }

    fn reflective_rectangle() -> Box<Rectangle> {
        Box::new(Rectangle::new(
            Point3D::new(0.0, 0.0, 0.0),
            Vector3D::new(0.0, 0.0, 1.0),
            Vector3D::new(0.0, 1.0, 0.0),
            (2.0, 2.0),
            SOP::Reflect,
            VOP::new(1.0),
            VOP::new(1.0),
        ))
    }

    #[test]
    fn counted_ray() {
        let mut ray = downwards_ray();
        assert_eq!(
            ray.launch(&[light_rectangle()]),
            BounceResult::Count(255, 255, 255)
        );
    }

    #[test]
    fn killed_ray_at_dark_rectangle() {
        let mut ray = downwards_ray();
        assert_eq!(ray.launch(&[dark_rectangle()]), BounceResult::Kill);
    }

    #[test]
    fn killed_ray_no_more_intersections() {
        let mut ray = downwards_ray();
        assert_eq!(ray.launch(&[reflective_rectangle()]), BounceResult::Kill);
    }
}
