use raytracer::{Camera, Plane, Point3D, Ray, Rectangle, Sphere, Vector3D, SOP, VOP};

fn main() {
    let air = VOP::new(1.0);

    let r1 = Rectangle::new(
        Point3D::new(-1.0, 11.0, 0.0),
        Vector3D::new(0.0, 0.0, 1.0),
        Vector3D::new(0.0, 1.0, 0.0),
        (2.0, 2.0),
        SOP::Light(255, 0, 0),
        air,
        air,
    );
    let r2 = Rectangle::new(
        Point3D::new(1.0, 11.0, 0.0),
        Vector3D::new(0.0, 0.0, 1.0),
        Vector3D::new(0.0, 1.0, 0.0),
        (2.0, 2.0),
        SOP::Dark,
        air,
        air,
    );
    let r3 = Rectangle::new(
        Point3D::new(-1.0, 13.0, 0.0),
        Vector3D::new(0.0, 0.0, 1.0),
        Vector3D::new(0.0, 1.0, 0.0),
        (2.0, 2.0),
        SOP::Dark,
        air,
        air,
    );
    let r4 = Rectangle::new(
        Point3D::new(1.0, 13.0, 0.0),
        Vector3D::new(0.0, 0.0, 1.0),
        Vector3D::new(0.0, 1.0, 0.0),
        (2.0, 2.0),
        SOP::Light(0, 0, 255),
        air,
        air,
    );

    let sphere_center = Point3D::new(0.0, 12.5, 1.0);
    let sphere = Sphere::new(sphere_center, 1.0, SOP::Reflect, air, VOP::new(1.0));

    let r_right = Rectangle::new(
        Point3D::new(2.0, 12.0, 2.0),
        Vector3D::mx(),
        Vector3D::pz(),
        (4.0, 4.0),
        SOP::Light(255, 255, 255),
        air,
        air,
    );

    let r_left = Rectangle::new(
        Point3D::new(-2.0, 12.0, 2.0),
        Vector3D::mx(),
        Vector3D::pz(),
        (4.0, 4.0),
        SOP::Light(255, 255, 255),
        air,
        air,
    );

    let r_back = Rectangle::new(
        Point3D::new(0.0, 14.0, 2.0),
        Vector3D::my(),
        Vector3D::pz(),
        (4.0, 4.0),
        SOP::Reflect,
        air,
        air,
    );

    let r_top = Rectangle::new(
        Point3D::new(0.0, 12.0, 4.0),
        Vector3D::mz(),
        Vector3D::py(),
        (4.0, 4.0),
        SOP::Dark,
        air,
        air,
    );

    let g = Vector3D::new(0.0, 1.5, -0.5);
    let camera = Camera::new(
        Point3D::new(0.0, 0.0, 5.0),
        g,
        Vector3D::new(1.0, 0.0, 0.0).cross(&g),
        (20.0, 30.0),
        0.01,
        air,
    );

    println!("{:?}", camera.screen_resolution());
    let result = camera.look(&[
        Box::new(sphere),
        Box::new(r1),
        Box::new(r2),
        Box::new(r3),
        Box::new(r4),
        Box::new(r_right),
        Box::new(r_left),
        Box::new(r_back),
        Box::new(r_top),
    ]);
    // println!("{:?}", result);
    camera.save_jpg(
        "/home/tuydore/repositories/raytracer/results/checkerboard.tiff",
        result,
    );
}
