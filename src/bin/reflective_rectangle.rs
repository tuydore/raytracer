use raytracer::{Camera, Plane, Point3D, Ray, Rectangle, Sphere, Vector3D, SOP, VOP};

fn main() {
    let r1 = Rectangle::new(
        Point3D::new(0.0, 2.0, 0.0),
        Vector3D::my(),
        Vector3D::pz(),
        (2.0, 2.0),
        SOP::Reflect,
        VOP::new(1.0),
        VOP::new(1.0),
    );

    let r2 = Rectangle::new(
        Point3D::new(0.0, -2.0, 0.0),
        Vector3D::py(),
        Vector3D::pz(),
        (2.0, 2.0),
        SOP::Light(255, 255, 255),
        VOP::new(1.0),
        VOP::new(1.0),
    );

    let sphere = Sphere::new(
        Point3D::new(0.0, 3.0, 0.0),
        1.0,
        SOP::Reflect,
        VOP::new(1.0),
        VOP::new(1.0),
    );

    let camera = Camera::new(
        Point3D::origin(),
        Vector3D::py(),
        Vector3D::pz(),
        (3.0, 3.0),
        1.0,
        VOP::new(1.0),
    );

    let result = camera.look(&[
        Box::new(sphere),
        // Box::new(r1),
        Box::new(r2),
    ]);
    println!("{:?}", result);
    // camera.save_jpg(
    //     "/home/tuydore/repositories/raytracer/results/simple_reflective.jpg",
    //     result,
    // );
}
