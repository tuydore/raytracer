use raytracer::{Camera, Plane, Point3D, Sphere, Vector3D, SOP, VOP};

fn main() {
    let _plane = Plane::new(
        Point3D::new(0.0, 0.0, 0.0),
        Vector3D::new(0.0, 0.0, 1.0),
        SOP::Reflect,
        VOP::new(1.0),
        VOP::new(1.0),
    );

    let sphere = Sphere::new(
        Point3D::new(0.0, 10.0, 0.0),
        1.0,
        SOP::Light(255, 255, 255),
        VOP::new(1.0),
        VOP::new(1.0),
    );

    let camera = Camera::new(
        Point3D::new(0.0, 0.0, 0.0),
        Vector3D::new(0.0, 1.0, 0.0),
        Vector3D::new(0.0, 0.0, 1.0),
        (20.0, 20.0),
        0.05,
        VOP::new(1.0),
    );

    let result = camera.look(&[Box::new(sphere)]);
    println!("{:?}", result);
    camera.save_jpg(
        "/home/tuydore/repositories/raytracer/results/myfirstraytrace.jpg",
        result,
    );
}
