use raytracer::{ray::BounceResult, Point3D, Ray, Vector3D, SOP, VOP};
use std::sync::Arc;

fn downwards_ray(vop: Arc<VOP>) -> Ray {
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

    fn light_sphere(vop: Arc<VOP>) -> Sphere {
        Sphere {
            geometry: SphereShape {
                center: Point3D::new(0.0, 0.0, 0.0),
                radius: 1.0,
            },
            sop: SOP::Light(255, 255, 255),
            vop_above: vop.clone(),
            vop_below: vop,
        }
    }

    fn dark_sphere(vop: Arc<VOP>) -> Sphere {
        Sphere {
            geometry: SphereShape {
                center: Point3D::new(0.0, 0.0, 0.0),
                radius: 1.0,
            },
            sop: SOP::Dark,
            vop_above: vop.clone(),
            vop_below: vop,
        }
    }

    fn reflective_sphere(vop: Arc<VOP>) -> Sphere {
        Sphere {
            geometry: SphereShape {
                center: Point3D::new(0.0, 0.0, 0.0),
                radius: 1.0,
            },
            sop: SOP::Reflect,
            vop_above: vop.clone(),
            vop_below: vop,
        }
    }

    #[test]
    fn counted_ray() {
        let air = Arc::new(VOP { ior: 1.0 });
        let sphere = light_sphere(air.clone());
        let mut ray = downwards_ray(air);
        assert_eq!(
            ray.launch(&[Arc::new(sphere)]),
            BounceResult::Count(255, 255, 255)
        );
    }

    #[test]
    fn killed_ray_at_dark_plane() {
        let air = Arc::new(VOP { ior: 1.0 });
        let mut ray = downwards_ray(air.clone());
        let sphere = dark_sphere(air);
        assert_eq!(ray.launch(&[Arc::new(sphere)]), BounceResult::Kill);
    }

    #[test]
    fn killed_ray_no_more_intersections() {
        let air = Arc::new(VOP { ior: 1.0 });
        let mut ray = downwards_ray(air.clone());
        let sphere = reflective_sphere(air);
        assert_eq!(ray.launch(&[Arc::new(sphere)]), BounceResult::Kill);
    }
}

#[cfg(test)]
mod plane_tests {
    use raytracer::{shape::InfinitePlaneShape, surface::plane::Plane};

    use super::*;

    fn light_plane(vop: Arc<VOP>) -> Plane {
        Plane {
            geometry: InfinitePlaneShape {
                origin: Point3D::new(0.0, 0.0, 0.0),
                normal: Vector3D::new(0.0, 0.0, 1.0),
            },
            sop: SOP::Light(255, 255, 255),
            vop_above: vop.clone(),
            vop_below: vop,
        }
    }

    fn dark_plane(vop: Arc<VOP>) -> Plane {
        Plane {
            geometry: InfinitePlaneShape {
                origin: Point3D::new(0.0, 0.0, 0.0),
                normal: Vector3D::new(0.0, 0.0, 1.0),
            },
            sop: SOP::Dark,
            vop_above: vop.clone(),
            vop_below: vop,
        }
    }

    fn reflective_plane(vop: Arc<VOP>) -> Plane {
        Plane {
            geometry: InfinitePlaneShape {
                origin: Point3D::new(0.0, 0.0, 0.0),
                normal: Vector3D::new(0.0, 0.0, 1.0),
            },
            sop: SOP::Reflect,
            vop_above: vop.clone(),
            vop_below: vop,
        }
    }

    #[test]
    fn counted_ray() {
        let air = Arc::new(VOP { ior: 1.0 });
        let mut ray = downwards_ray(air.clone());
        let plane = light_plane(air);
        assert_eq!(
            ray.launch(&[Arc::new(plane)]),
            BounceResult::Count(255, 255, 255)
        );
    }

    #[test]
    fn killed_ray_at_dark_plane() {
        let air = Arc::new(VOP { ior: 1.0 });
        let mut ray = downwards_ray(air.clone());
        let plane = dark_plane(air);
        assert_eq!(ray.launch(&[Arc::new(plane)]), BounceResult::Kill);
    }

    #[test]
    fn killed_ray_no_more_intersections() {
        let air = Arc::new(VOP { ior: 1.0 });
        let mut ray = downwards_ray(air.clone());
        let plane = reflective_plane(air);
        assert_eq!(ray.launch(&[Arc::new(plane)]), BounceResult::Kill);
    }
}

#[cfg(test)]
mod rectangle_tests {
    use raytracer::{shape::RectangleShape, surface::rectangle::Rectangle};

    use super::*;

    fn light_rectangle(vop: Arc<VOP>) -> Rectangle {
        Rectangle {
            geometry: RectangleShape {
                origin: Point3D::new(0.0, 0.0, 0.0),
                normal: Vector3D::new(0.0, 0.0, 1.0),
                orientation: Vector3D::new(0.0, 1.0, 0.0),
                size: [2.0, 2.0],
            },
            sop: SOP::Light(255, 255, 255),
            vop_above: vop.clone(),
            vop_below: vop,
        }
    }

    fn dark_rectangle(vop: Arc<VOP>) -> Rectangle {
        Rectangle {
            geometry: RectangleShape {
                origin: Point3D::new(0.0, 0.0, 0.0),
                normal: Vector3D::new(0.0, 0.0, 1.0),
                orientation: Vector3D::new(0.0, 1.0, 0.0),
                size: [2.0, 2.0],
            },
            sop: SOP::Dark,
            vop_above: vop.clone(),
            vop_below: vop,
        }
    }

    fn reflective_rectangle(vop: Arc<VOP>) -> Rectangle {
        Rectangle {
            geometry: RectangleShape {
                origin: Point3D::new(0.0, 0.0, 0.0),
                normal: Vector3D::new(0.0, 0.0, 1.0),
                orientation: Vector3D::new(0.0, 1.0, 0.0),
                size: [2.0, 2.0],
            },
            sop: SOP::Reflect,
            vop_above: vop.clone(),
            vop_below: vop,
        }
    }

    #[test]
    fn counted_ray() {
        let air = Arc::new(VOP { ior: 1.0 });
        let mut ray = downwards_ray(air.clone());
        let rectangle = light_rectangle(air);
        assert_eq!(
            ray.launch(&[Arc::new(rectangle)]),
            BounceResult::Count(255, 255, 255)
        );
    }

    #[test]
    fn killed_ray_at_dark_rectangle() {
        let air = Arc::new(VOP { ior: 1.0 });
        let mut ray = downwards_ray(air.clone());
        let rectangle = dark_rectangle(air);
        assert_eq!(ray.launch(&[Arc::new(rectangle)]), BounceResult::Kill);
    }

    #[test]
    fn killed_ray_no_more_intersections() {
        let air = Arc::new(VOP { ior: 1.0 });
        let mut ray = downwards_ray(air.clone());
        let rectangle = reflective_rectangle(air);
        assert_eq!(ray.launch(&[Arc::new(rectangle)]), BounceResult::Kill);
    }
}
