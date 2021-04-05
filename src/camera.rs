use {
    crate::{ray::BounceResult, Ray, Surface, VOP},
    image::{Rgb, RgbImage},
    nalgebra::{Isometry3, Point3, Unit, Vector3},
    rayon::prelude::*,
    serde::Deserialize,
    std::collections::HashMap,
    std::sync::Arc,
    std::time::Instant,
};
// use indicatif::ProgressBar;

/// Split a rectangle into dividing unit cells, according to a given spacing and return either
/// the corners or the centers of the subdivisions.
fn split_rectangle(
    corner: Point3<f64>,
    size_x: f64,
    size_y: f64,
    num_x: usize,
    num_y: usize,
    corners: bool,
) -> Vec<Point3<f64>> {
    let mut points: Vec<Point3<f64>> = Vec::new();
    let mut v: Vector3<f64>;

    // spacing between adjacent points on the grid
    let dx: f64 = size_x / num_x as f64;
    let dy: f64 = size_y / num_y as f64;

    for i in 0..num_x {
        for j in 0..num_y {
            v = Vector3::new(i as f64 * dx, j as f64 * dy, 0.0);
            // add half-spacing to obtain centers of subgrid
            if !corners {
                v.x += dx / 2.0;
                v.y += dy / 2.0;
            }
            points.push(corner + v);
        }
    }
    points
}

/// Trace a number of rays through the given scene and return their final color values.
pub fn trace_rays(rays: Vec<Ray>, scene: &[Arc<dyn Surface + Send + Sync>]) -> Vec<[u8; 3]> {
    print!("Starting raytrace... ");
    let num_rays: usize = rays.len();
    let t0 = Instant::now();

    let mut result: Vec<(usize, u8, u8, u8)> = rays
        .into_par_iter()
        .enumerate()
        .map(|(i, mut r)| {
            // pbar.inc(1);
            match r.launch(scene) {
                BounceResult::Count(r, g, b) => (i, r, g, b),
                BounceResult::Kill => (i, 0, 0, 0),
                _ => panic!("Something has gone wrong."),
            }
        })
        .collect();

    let seconds = t0.elapsed().as_millis() as f64 / 1000.0;
    // pbar.finish_and_clear();

    // sort result by index
    result.sort_by_key(|(i, _, _, _)| *i);
    let result: Vec<[u8; 3]> = result.into_iter().map(|(_, r, g, b)| [r, g, b]).collect();

    // show total time
    println!(
        "done! Total time: {}s, rays/s: {}.",
        seconds,
        (num_rays as f64 / seconds) as u64
    );
    result
}

/// Average a slice of [u8; 3] arrays.
fn average_array3(arr3: &[[u8; 3]]) -> [u8; 3] {
    let mut result: [usize; 3] = [0; 3];
    let l = arr3.len();
    for n in 0..=2 {
        for elem in arr3 {
            result[n] += elem[n] as usize;
        }
        result[n] /= l;
    }
    [result[0] as u8, result[1] as u8, result[2] as u8]
}

/// Combine rays by their antialiasing chunking. E.g. for AA=3, chunks of 9 will be averaged
/// together.
pub fn combine_rays(results: Vec<[u8; 3]>, antialiasing: usize) -> Vec<[u8; 3]> {
    results
        .chunks(antialiasing.pow(2))
        .map(|chunk| average_array3(chunk))
        .collect()
}

/// Save a vector of [u8; 3] data to an image.
pub fn save_jpg(filepath: &str, data: Vec<[u8; 3]>, num_x: usize, num_y: usize) {
    println!("Saving image...");
    let mut img = RgbImage::new(num_y as u32, num_x as u32);
    for x in 0..num_x {
        for y in 0..num_y {
            img.put_pixel(y as u32, x as u32, Rgb(data[x * num_y + y]));
        }
    }
    let _ = img.save(filepath);
}

#[derive(Debug)]
pub struct Camera {
    origin: Point3<f64>,
    screen_corner: Point3<f64>,
    screen_local_to_world: Isometry3<f64>,
    size_x: f64,
    size_y: f64,
    pub num_x: usize,
    pub num_y: usize,
    pub antialiasing: usize,
    vop: Arc<VOP>,
}

impl Camera {
    fn pixel_corners(&self) -> Vec<Point3<f64>> {
        split_rectangle(
            Point3::origin(),
            self.size_x,
            self.size_y,
            self.num_x,
            self.num_y,
            true,
        )
    }

    fn subpixel_centers(&self) -> Vec<Point3<f64>> {
        let pixel_size_x: f64 = self.size_x / self.num_x as f64;
        let pixel_size_y: f64 = self.size_y / self.num_y as f64;
        self.pixel_corners()
            .into_iter()
            .map(|pxc| {
                split_rectangle(
                    pxc,
                    pixel_size_x,
                    pixel_size_y,
                    self.antialiasing,
                    self.antialiasing,
                    false,
                )
            })
            .flatten()
            .collect()
    }

    pub fn create_rays(&self) -> Vec<Ray> {
        let num_pixels: usize = self.num_x * self.num_y;
        print!(
            "Generating {} x {}^2 = {} rays... ",
            num_pixels,
            self.antialiasing,
            num_pixels * self.antialiasing.pow(2)
        );
        let t0 = Instant::now();
        let rays: Vec<Ray> = self
            .subpixel_centers()
            .into_iter()
            .map(|sbpxc| Ray {
                origin: self.origin,
                direction: self.screen_local_to_world * sbpxc - self.origin,
                vop: self.vop.clone(),
                abs: [0.0; 3],
            })
            .collect();
        println!(
            "done! Total time: {}s.",
            t0.elapsed().as_millis() as f64 / 1000.0
        );
        rays
    }
}

#[derive(Debug, Deserialize)]
pub struct CameraBuilder {
    origin: [f64; 3],
    gaze: [f64; 3],
    up: [f64; 3],
    fov: [f64; 2],
    density: f64,
    #[serde(default = "default_antialiasing")]
    antialiasing: usize,
    vop: String,
}

fn default_antialiasing() -> usize {
    1
}

impl CameraBuilder {
    pub fn build(self, vop_map: &HashMap<String, Arc<VOP>>) -> Camera {
        // allow approximate "up" direction
        let origin = Point3::from_slice(&self.origin);
        let gaze: Unit<Vector3<f64>> = Unit::new_normalize(Vector3::from_row_slice(&self.gaze));
        let mut up: Unit<Vector3<f64>> = Unit::new_normalize(Vector3::from_row_slice(&self.up));
        up = Unit::new_normalize(gaze.cross(&up.cross(&gaze))); // QUESTION: use unchecked here?

        let (size_x, size_y) = self.screen_size();
        let screen_corner: Point3<f64> = origin
            + gaze.into_inner()
            + up.into_inner() * size_x / 2.0
            + up.cross(&gaze) * size_y / 2.0;

        Camera {
            origin,
            screen_corner,
            screen_local_to_world: Isometry3::look_at_lh(
                &screen_corner,
                &(screen_corner - gaze.into_inner()),
                &gaze.cross(&up),
            )
            .inverse(),
            size_x,
            size_y,
            num_x: (self.fov[0] * self.density) as usize,
            num_y: (self.fov[1] * self.density) as usize,
            antialiasing: self.antialiasing,
            vop: vop_map
                .get(&self.vop)
                .expect("No VOP above mapping found.")
                .clone(),
        }
    }

    /// Get the real screen size.
    fn screen_size(&self) -> (f64, f64) {
        (
            2.0 * (self.fov[0] / 2.0).to_radians().tan(),
            2.0 * (self.fov[1] / 2.0).to_radians().tan(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::TOLERANCE;

    fn camera(vop: Arc<VOP>, fov: [f64; 2]) -> Camera {
        let cb = CameraBuilder {
            origin: [0.0, 0.0, 0.0],
            gaze: [0.0, 1.0, 0.0],
            up: [0.0, 0.0, 1.0],
            fov,
            density: 1.0,
            antialiasing: 1,
            vop: "air".to_owned(),
        };
        let mut vop_map: HashMap<String, Arc<VOP>> = HashMap::new();
        vop_map.insert("air".to_owned(), vop);

        cb.build(&vop_map)
    }

    #[test]
    fn screen_size() {
        let air = Arc::new(VOP {
            ior: 1.0,
            abs: [0.0; 3],
        });
        let c = camera(air, [20.0, 30.0]);
        let theo = (0.35265, 0.53590);
        assert!((c.size_x - theo.0).abs() <= 1e-5);
        assert!((c.size_y - theo.1).abs() <= 1e-5);
    }

    #[test]
    fn ray_centering() {
        let air = Arc::new(VOP {
            ior: 1.0,
            abs: [0.0, 0.0, 0.0],
        });
        let c = camera(air, [1.0, 1.0]);
        let centers = c.subpixel_centers();
        assert_eq!(centers.len(), 1);
        assert!(
            (c.screen_local_to_world * centers[0] - Point3::new(0.0, 1.0, 0.0)).norm() < TOLERANCE
        );
    }
}
