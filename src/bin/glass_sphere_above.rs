use raytracer::{Camera, Checkerboard, Point3D, Rectangle, Sphere, Surface, Vector3D, SOP, VOP};

fn checkerboard_xy(side: f64, num: usize) -> Vec<Box<dyn Surface>> {
    let mut result: Vec<Box<dyn Surface>> = Vec::new();
    for i in 0..=num {
        for j in 0..=num {
            let surface = if (i + j) % 2 == 0 {
                SOP::Light(255, 255, 255)
            } else {
                SOP::Dark
            };
            let rectangle = Rectangle::new(
                Point3D::new(side * (0.5 + i as f64), side * (0.5 + j as f64), 0.0),
                Vector3D::pz(),
                Vector3D::px(),
                (side, side),
                surface,
                VOP::new(1.0),
                VOP::new(1.0),
            );
            result.push(Box::new(rectangle));
        }
    }
    result
}

fn main() {
    let air = VOP::new(1.0);

    let checkerboard = Checkerboard::new(
        Point3D::new(0.0, 0.0, 0.0),
        Vector3D::pz(),
        Vector3D::py(),
        (255, 255, 255),
        2.0,
        air,
        air,
    );
    // let mut checkerboard = checkerboard_xy(1.0, 20);

    let mut spheres = Vec::new();

    spheres.push(Sphere::new(
        Point3D::new(6.0, 6.0, 2.0),
        2.0,
        SOP::Refract,
        air,
        VOP::new(1.5),
    ));

    spheres.push(Sphere::new(
        Point3D::new(0.0, 0.0, 2.0),
        2.0,
        SOP::Light(94, 39, 80),
        air,
        VOP::new(1.5),
    ));

    for (x, y) in [(0.0, 6.0), (6.0, 0.0)].iter() {
        spheres.push(Sphere::new(
            Point3D::new(*x, *y, 2.0),
            2.0,
            SOP::Reflect,
            air,
            VOP::new(1.0),
        ))
    }

    let g = Vector3D::new(-1.0, -1.0, -0.5);
    let camera = Camera::new(
        Point3D::new(30.0, 30.0, 15.0),
        g,
        g.cross(&Vector3D::new(1.0, -1.0, 0.0)),
        (18.0, 32.0),
        200.0,
        air,
    );

    let mut scene: Vec<Box<dyn Surface>> = vec![Box::new(checkerboard)];
    for sph in spheres {
        scene.push(Box::new(sph));
    }

    println!("{:?}", camera.screen_resolution());
    let result = camera.look(&scene);
    // println!("{:?}", result);
    camera.save_jpg(
        "/home/tuydore/repositories/raytracer/results/glass_spheres_mix.jpg",
        result,
    );
}
