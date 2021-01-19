use raytracer::{ray::BounceResult, Plane, Point3D, Ray, Sphere, Vector3D, SOP, VOP};

fn downwards_ray(vop: &VOP) -> Ray {
    Ray::new(
        Point3D::new(0.0, 0.0, 10.0),
        Vector3D::new(0.0, 0.0, -1.0),
        vop,
    )
}

#[cfg(test)]
mod sphere_tests {
    use super::*;

    fn light_sphere(vop: &VOP) -> Sphere {
        Sphere::new(
            Point3D::new(0.0, 0.0, 0.0),
            1.0,
            SOP::Light(255, 255, 255),
            vop,
            vop,
        )
    }

    fn dark_sphere(vop: &VOP) -> Sphere {
        Sphere::new(Point3D::new(0.0, 0.0, 0.0), 1.0, SOP::Dark, vop, vop)
    }

    fn reflective_sphere(vop: &VOP) -> Sphere {
        Sphere::new(Point3D::new(0.0, 0.0, 0.0), 1.0, SOP::Reflect, vop, vop)
    }

    #[test]
    fn counted_ray() {
        let air = VOP::new(1.0);
        let sphere = light_sphere(&air);
        let mut ray = downwards_ray(&air);
        assert_eq!(ray.launch(&[&sphere]), BounceResult::Count(255, 255, 255));
    }

    #[test]
    fn killed_ray_at_dark_plane() {
        let air = VOP::new(1.0);
        let mut ray = downwards_ray(&air);
        let sphere = dark_sphere(&air);
        assert_eq!(ray.launch(&[&sphere]), BounceResult::Kill);
    }

    #[test]
    fn killed_ray_no_more_intersections() {
        let air = VOP::new(1.0);
        let mut ray = downwards_ray(&air);
        let sphere = reflective_sphere(&air);
        assert_eq!(ray.launch(&[&sphere]), BounceResult::Kill);
    }
}

#[cfg(test)]
mod plane_tests {
    use super::*;

    fn light_plane(vop: &VOP) -> Plane {
        Plane::new(
            Point3D::new(0.0, 0.0, 0.0),
            Vector3D::new(0.0, 0.0, 1.0),
            SOP::Light(255, 255, 255),
            vop,
            vop,
        )
    }

    fn dark_plane(vop: &VOP) -> Plane {
        Plane::new(
            Point3D::new(0.0, 0.0, 0.0),
            Vector3D::new(0.0, 0.0, 1.0),
            SOP::Dark,
            vop,
            vop,
        )
    }

    fn reflective_plane(vop: &VOP) -> Plane {
        Plane::new(
            Point3D::new(0.0, 0.0, 0.0),
            Vector3D::new(0.0, 0.0, 1.0),
            SOP::Reflect,
            vop,
            vop,
        )
    }

    #[test]
    fn counted_ray() {
        let air = VOP::new(1.0);
        let mut ray = downwards_ray(&air);
        let plane = light_plane(&air);
        assert_eq!(ray.launch(&[&plane]), BounceResult::Count(255, 255, 255));
    }

    #[test]
    fn killed_ray_at_dark_plane() {
        let air = VOP::new(1.0);
        let mut ray = downwards_ray(&air);
        let plane = dark_plane(&air);
        assert_eq!(ray.launch(&[&plane]), BounceResult::Kill);
    }

    #[test]
    fn killed_ray_no_more_intersections() {
        let air = VOP::new(1.0);
        let mut ray = downwards_ray(&air);
        let plane = reflective_plane(&air);
        assert_eq!(ray.launch(&[&plane]), BounceResult::Kill);
    }
}

#[cfg(test)]
mod rectangle_tests {
    use raytracer::Rectangle;

    use super::*;

    fn light_rectangle(vop: &VOP) -> Rectangle {
        Rectangle::new(
            Point3D::new(0.0, 0.0, 0.0),
            Vector3D::new(0.0, 0.0, 1.0),
            Vector3D::new(0.0, 1.0, 0.0),
            (2.0, 2.0),
            SOP::Light(255, 255, 255),
            vop,
            vop,
        )
    }

    fn dark_rectangle(vop: &VOP) -> Rectangle {
        Rectangle::new(
            Point3D::new(0.0, 0.0, 0.0),
            Vector3D::new(0.0, 0.0, 1.0),
            Vector3D::new(0.0, 1.0, 0.0),
            (2.0, 2.0),
            SOP::Dark,
            vop,
            vop,
        )
    }

    fn reflective_rectangle(vop: &VOP) -> Rectangle {
        Rectangle::new(
            Point3D::new(0.0, 0.0, 0.0),
            Vector3D::new(0.0, 0.0, 1.0),
            Vector3D::new(0.0, 1.0, 0.0),
            (2.0, 2.0),
            SOP::Reflect,
            vop,
            vop,
        )
    }

    #[test]
    fn counted_ray() {
        let air = VOP::new(1.0);
        let mut ray = downwards_ray(&air);
        let rectangle = light_rectangle(&air);
        assert_eq!(
            ray.launch(&[&rectangle]),
            BounceResult::Count(255, 255, 255)
        );
    }

    #[test]
    fn killed_ray_at_dark_rectangle() {
        let air = VOP::new(1.0);
        let mut ray = downwards_ray(&air);
        let rectangle = dark_rectangle(&air);
        assert_eq!(ray.launch(&[&rectangle]), BounceResult::Kill);
    }

    #[test]
    fn killed_ray_no_more_intersections() {
        let air = VOP::new(1.0);
        let mut ray = downwards_ray(&air);
        let rectangle = reflective_rectangle(&air);
        assert_eq!(ray.launch(&[&rectangle]), BounceResult::Kill);
    }
}
