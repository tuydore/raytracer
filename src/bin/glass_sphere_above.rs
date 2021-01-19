use raytracer::{Camera, Checkerboard, Point3D, Sphere, Surface, Vector3D, SOP, VOP};

fn main() {
    let air = VOP::new(1.0);
    let glass = VOP::new(1.5);

    let checkerboard = Checkerboard::new(
        Point3D::new(0.0, 0.0, 0.0),
        Vector3D::pz(),
        Vector3D::py(),
        (255, 255, 255),
        2.0,
        &air,
        &air,
    );
    // let mut checkerboard = checkerboard_xy(1.0, 20);

    let mut spheres = Vec::new();

    spheres.push(Sphere::new(
        Point3D::new(6.0, 6.0, 2.0),
        2.0,
        SOP::Refract,
        &air,
        &glass,
    ));

    spheres.push(Sphere::new(
        Point3D::new(0.0, 0.0, 2.0),
        2.0,
        SOP::Light(94, 39, 80),
        &air,
        &glass,
    ));

    for (x, y) in [(0.0, 6.0), (6.0, 0.0)].iter() {
        spheres.push(Sphere::new(
            Point3D::new(*x, *y, 2.0),
            2.0,
            SOP::Reflect,
            &air,
            &glass,
        ))
    }

    let g = Vector3D::new(-1.0, -1.0, -0.5);
    let camera = Camera::new(
        Point3D::new(30.0, 30.0, 15.0),
        g,
        g.cross(&Vector3D::new(1.0, -1.0, 0.0)),
        (18.0, 32.0),
        200.0,
        &air,
    );

    let mut scene: Vec<&dyn Surface> = vec![&checkerboard];
    for sph in spheres.iter() {
        scene.push(sph);
    }

    println!("{:?}", camera.screen_resolution());
    let result = camera.look(&scene);
    // println!("{:?}", result);
    camera.save_jpg(
        "/home/tuydore/repositories/raytracer/results/glass_spheres_mix.jpg",
        result,
    );
}
