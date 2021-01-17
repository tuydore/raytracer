use raytracer::{
    geometry::{sop, Shape},
    Camera, Plane, Point3D, Ray, Rectangle, Sphere, Vector3D, SOP, VOP,
};

fn checkerboard_xy(side: f64, num: usize) -> Vec<Box<dyn Shape>> {
    let mut result: Vec<Box<dyn Shape>> = Vec::new();
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

    let mut checkerboard = checkerboard_xy(1.0, 20);

    let sphere_center = Point3D::new(10.0, 10.0, 2.0);
    let sphere = Sphere::new(sphere_center, 2.0, SOP::Refract, air, VOP::new(1.5));

    let g = Vector3D::new(0.0, 0.0, -1.0);
    let camera = Camera::new(
        Point3D::new(9.0, 9.0, 30.0),
        g,
        Vector3D::px(), // g.cross(&Vector3D::new(-1.0, 1.0, 0.0)),
        (20.0, 30.0),
        100.0,
        air,
    );
    checkerboard.push(Box::new(sphere));

    println!("{:?}", camera.screen_resolution());
    let result = camera.look(&checkerboard);
    // println!("{:?}", result);
    camera.save_jpg(
        "/home/tuydore/repositories/raytracer/results/glass_sphere_above.tiff",
        result,
    );
}
