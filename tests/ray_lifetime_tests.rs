use raytracer::{ray::BounceResult, Point3D, Ray, Vector3D, SOP, VOP};

fn downwards_ray(vop: &VOP) -> Ray {
    Ray {
        origin: Point3D::new(0.0, 0.0, 10.0),
        direction: Vector3D::new(0.0, 0.0, -1.0),
        vop,
    }
}

#[cfg(test)]
mod sphere_tests {
    use raytracer::{shape::SphereShape, surface::sphere::Sphere};

    use super::*;

    fn light_sphere(vop: &VOP) -> Sphere {
        Sphere {
            geometry: SphereShape {
                center: Point3D::new(0.0, 0.0, 0.0),
                radius: 1.0,
            },
            sop: SOP::Light(255, 255, 255),
            vop_above: vop,
            vop_below: vop,
        }
    }

    fn dark_sphere(vop: &VOP) -> Sphere {
        Sphere {
            geometry: SphereShape {
                center: Point3D::new(0.0, 0.0, 0.0),
                radius: 1.0,
            },
            sop: SOP::Dark,
            vop_above: vop,
            vop_below: vop,
        }
    }

    fn reflective_sphere(vop: &VOP) -> Sphere {
        Sphere {
            geometry: SphereShape {
                center: Point3D::new(0.0, 0.0, 0.0),
                radius: 1.0,
            },
            sop: SOP::Reflect,
            vop_above: vop,
            vop_below: vop,
        }
    }

    #[test]
    fn counted_ray() {
        let air = VOP { ior: 1.0 };
        let sphere = light_sphere(&air);
        let mut ray = downwards_ray(&air);
        assert_eq!(ray.launch(&[&sphere]), BounceResult::Count(255, 255, 255));
    }

    #[test]
    fn killed_ray_at_dark_plane() {
        let air = VOP { ior: 1.0 };
        let mut ray = downwards_ray(&air);
        let sphere = dark_sphere(&air);
        assert_eq!(ray.launch(&[&sphere]), BounceResult::Kill);
    }

    #[test]
    fn killed_ray_no_more_intersections() {
        let air = VOP { ior: 1.0 };
        let mut ray = downwards_ray(&air);
        let sphere = reflective_sphere(&air);
        assert_eq!(ray.launch(&[&sphere]), BounceResult::Kill);
    }
}

#[cfg(test)]
mod plane_tests {
    use raytracer::{shape::InfinitePlaneShape, surface::plane::Plane};

    use super::*;

    fn light_plane(vop: &VOP) -> Plane {
        Plane {
            geometry: InfinitePlaneShape {
                origin: Point3D::new(0.0, 0.0, 0.0),
                normal: Vector3D::new(0.0, 0.0, 1.0),
            },
            sop: SOP::Light(255, 255, 255),
            vop_above: vop,
            vop_below: vop,
        }
    }

    fn dark_plane(vop: &VOP) -> Plane {
        Plane {
            geometry: InfinitePlaneShape {
                origin: Point3D::new(0.0, 0.0, 0.0),
                normal: Vector3D::new(0.0, 0.0, 1.0),
            },
            sop: SOP::Dark,
            vop_above: vop,
            vop_below: vop,
        }
    }

    fn reflective_plane(vop: &VOP) -> Plane {
        Plane {
            geometry: InfinitePlaneShape {
                origin: Point3D::new(0.0, 0.0, 0.0),
                normal: Vector3D::new(0.0, 0.0, 1.0),
            },
            sop: SOP::Reflect,
            vop_above: vop,
            vop_below: vop,
        }
    }

    #[test]
    fn counted_ray() {
        let air = VOP { ior: 1.0 };
        let mut ray = downwards_ray(&air);
        let plane = light_plane(&air);
        assert_eq!(ray.launch(&[&plane]), BounceResult::Count(255, 255, 255));
    }

    #[test]
    fn killed_ray_at_dark_plane() {
        let air = VOP { ior: 1.0 };
        let mut ray = downwards_ray(&air);
        let plane = dark_plane(&air);
        assert_eq!(ray.launch(&[&plane]), BounceResult::Kill);
    }

    #[test]
    fn killed_ray_no_more_intersections() {
        let air = VOP { ior: 1.0 };
        let mut ray = downwards_ray(&air);
        let plane = reflective_plane(&air);
        assert_eq!(ray.launch(&[&plane]), BounceResult::Kill);
    }
}

#[cfg(test)]
mod rectangle_tests {
    use raytracer::{shape::RectangleShape, surface::rectangle::Rectangle};

    use super::*;

    fn light_rectangle(vop: &VOP) -> Rectangle {
        Rectangle {
            geometry: RectangleShape {
                origin: Point3D::new(0.0, 0.0, 0.0),
                normal: Vector3D::new(0.0, 0.0, 1.0),
                orientation: Vector3D::new(0.0, 1.0, 0.0),
                size: [2.0, 2.0],
            },
            sop: SOP::Light(255, 255, 255),
            vop_above: vop,
            vop_below: vop,
        }
    }

    fn dark_rectangle(vop: &VOP) -> Rectangle {
        Rectangle {
            geometry: RectangleShape {
                origin: Point3D::new(0.0, 0.0, 0.0),
                normal: Vector3D::new(0.0, 0.0, 1.0),
                orientation: Vector3D::new(0.0, 1.0, 0.0),
                size: [2.0, 2.0],
            },
            sop: SOP::Dark,
            vop_above: vop,
            vop_below: vop,
        }
    }

    fn reflective_rectangle(vop: &VOP) -> Rectangle {
        Rectangle {
            geometry: RectangleShape {
                origin: Point3D::new(0.0, 0.0, 0.0),
                normal: Vector3D::new(0.0, 0.0, 1.0),
                orientation: Vector3D::new(0.0, 1.0, 0.0),
                size: [2.0, 2.0],
            },
            sop: SOP::Reflect,
            vop_above: vop,
            vop_below: vop,
        }
    }

    #[test]
    fn counted_ray() {
        let air = VOP { ior: 1.0 };
        let mut ray = downwards_ray(&air);
        let rectangle = light_rectangle(&air);
        assert_eq!(
            ray.launch(&[&rectangle]),
            BounceResult::Count(255, 255, 255)
        );
    }

    #[test]
    fn killed_ray_at_dark_rectangle() {
        let air = VOP { ior: 1.0 };
        let mut ray = downwards_ray(&air);
        let rectangle = dark_rectangle(&air);
        assert_eq!(ray.launch(&[&rectangle]), BounceResult::Kill);
    }

    #[test]
    fn killed_ray_no_more_intersections() {
        let air = VOP { ior: 1.0 };
        let mut ray = downwards_ray(&air);
        let rectangle = reflective_rectangle(&air);
        assert_eq!(ray.launch(&[&rectangle]), BounceResult::Kill);
    }
}
