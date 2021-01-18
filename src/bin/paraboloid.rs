use raytracer::{
    Camera, Checkerboard, Plane, Point3D, Sphere, Surface, Vector3D, ZParaboloid, SOP, VOP,
};

fn main() {
    let air = VOP::new(1.0);
    let _glass = VOP::new(1.5);

    // TODO: refraction
    let paraboloid = ZParaboloid::new(0.0, 0.0, 0.0, -1.0, -1.0, SOP::Reflect, air, air);
    let sphere = Sphere::new(Point3D::new(0.0, 0.0, 2.0), 1.0, SOP::Reflect, air, air);

    // walls
    let py = Checkerboard::new(
        Point3D::new(0.0, 10.0, 0.0),
        Vector3D::my(),
        Vector3D::pz(),
        (255, 0, 0),
        5.0,
        air,
        air,
    );
    let my = Checkerboard::new(
        Point3D::new(0.0, -10.0, 0.0),
        Vector3D::py(),
        Vector3D::pz(),
        (0, 0, 255),
        5.0,
        air,
        air,
    );
    let mx = Plane::new(
        Point3D::new(-200.0, 0.0, 0.0),
        Vector3D::px(),
        SOP::Dark,
        air,
        air,
    );
    let px = Plane::new(
        Point3D::new(200.0, 0.0, 0.0),
        Vector3D::mx(),
        SOP::Reflect,
        air,
        air,
    );
    let scene: Vec<Box<dyn Surface>> = vec![
        Box::new(paraboloid),
        Box::new(sphere),
        Box::new(py),
        Box::new(my),
        Box::new(px),
        Box::new(mx),
    ];

    let gaze = Vector3D::px();
    let camera = Camera::new(
        Point3D::new(-10.0, 0.0, 0.5),
        gaze,
        Vector3D::pz(), // gaze.cross(&Vector3D::new(-1.0, 1.0, 0.0)),
        (20.0, 30.0),
        200.0,
        air,
    );

    println!("{:?}", camera.screen_resolution());
    let result = camera.look(&scene);
    camera.save_jpg(
        "/home/tuydore/repositories/raytracer/results/paraboloid.jpg",
        result,
    );
}
