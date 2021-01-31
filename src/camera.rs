use {
    crate::{ray::BounceResult, Ray, Surface, VOP},
    image::{Rgb, RgbImage},
    nalgebra::{Point3, Vector3},
    rayon::prelude::*,
    serde::Deserialize,
    std::collections::HashMap,
    std::sync::Arc,
    std::time::Instant,
};
// use indicatif::ProgressBar;

#[derive(Debug)]
pub struct Camera {
    origin: Point3<f64>,
    gaze: Vector3<f64>,
    up: Vector3<f64>,
    fov: [f64; 2],
    density: f64,
    vop: Arc<VOP>,
}

#[derive(Debug, Deserialize)]
pub struct CameraBuilder {
    origin: [f64; 3],
    gaze: [f64; 3],
    up: [f64; 3],
    fov: [f64; 2],
    density: f64,
    vop: String,
}

impl CameraBuilder {
    pub fn build(self, vop_map: &HashMap<String, Arc<VOP>>) -> Camera {
        Camera {
            origin: Point3::from_slice(&self.origin),
            gaze: Vector3::from_row_slice(&self.gaze),
            up: Vector3::from_row_slice(&self.up),
            fov: self.fov,
            density: self.density,
            vop: vop_map
                .get(&self.vop)
                .expect("No VOP above mapping found.")
                .clone(),
        }
    }
}

// TODO: check gaze and up are perpendicular
impl Camera {
    /// Assume a screen filling the entire FOV will be placed at a distance of 1 in front of the camera.
    /// With the density parameter, we will approximate the number of bins in each dimension.
    pub fn screen_resolution(&self) -> (usize, usize) {
        (
            (self.fov[0] * self.density).round() as usize,
            (self.fov[1] * self.density).round() as usize,
        )
    }

    /// Get the real screen size.
    fn screen_size(&self) -> (f64, f64) {
        (
            2.0 * (self.fov[0] / 2.0).to_radians().tan(),
            2.0 * (self.fov[1] / 2.0).to_radians().tan(),
        )
    }

    fn screen_upper_left_corner(&self) -> Point3<f64> {
        let (sizex, sizey) = self.screen_size();
        let g = self.gaze.normalize();
        let u = self.up.normalize();
        self.origin + g + u * sizex / 2.0 + u.cross(&g) * sizey / 2.0
    }

    fn pixel_centers(&self) -> Vec<Point3<f64>> {
        // orientation vectors r==right, d==down
        let g = self.gaze.normalize();
        let u = self.up.normalize();
        let r = g.cross(&u);
        let d = -1.0 * u;

        // obtain screen density
        let (sizex, sizey) = self.screen_size();
        let (npx, npy) = self.screen_resolution();
        let pixel_size_x = sizex / npx as f64;
        let pixel_size_y = sizey / npy as f64;

        // starting point is the center of the top-left PIXEL, not the screen itself
        let upper_left_pixel_center =
            self.screen_upper_left_corner() + r * pixel_size_y / 2.0 + d * pixel_size_x / 2.0;

        let mut centers = Vec::new();
        for i in 0..npx {
            for j in 0..npy {
                centers.push(
                    upper_left_pixel_center
                        + r * pixel_size_y * j as f64
                        + d * pixel_size_x * i as f64,
                );
            }
        }
        centers
    }

    /// Capture the scene before the camera's eyes.
    pub fn look(&self, scene: &[Arc<dyn Surface + Send + Sync>]) -> Vec<(u8, u8, u8)> {
        // show resolution info
        let (npx, npy) = self.screen_resolution();
        println!("Screen resolution: {} x {}", npy, npx);

        // instantiate progress bar
        let num_rays: u64 = self.pixel_centers().len() as u64;

        // TODO: why is this so slow?
        // let pbar = ProgressBar::new(num_rays);
        // pbar.set_draw_delta(num_rays / 100);
        // pbar.enable_steady_tick(1000);

        // create all rays
        print!("Pre-generating {} rays... ", num_rays);
        let t0 = Instant::now();
        let rays: Vec<Ray> = self
            .pixel_centers()
            .into_iter()
            .map(|pxc| Ray {
                origin: self.origin,
                direction: pxc - self.origin,
                vop: self.vop.clone(),
                abs: [0.0; 3],
            })
            .collect();
        println!("done ({}s)", t0.elapsed().as_millis() as f64 / 1000.0);

        // start timer
        print!("Starting raytrace...");
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
        let result = result.into_iter().map(|(_, r, g, b)| (r, g, b)).collect();

        // show total time
        println!(
            "done (total time: {}s, rays/s: {})",
            seconds,
            (num_rays as f64 / seconds) as u64
        );
        result
    }

    pub fn save_jpg(&self, filepath: &str, data: Vec<(u8, u8, u8)>) {
        println!("Saving image...");
        let (rx, ry) = self.screen_resolution();
        let mut img = RgbImage::new(ry as u32, rx as u32);
        for x in 0..rx {
            for y in 0..ry {
                let (r, g, b) = data[x * ry + y];
                img.put_pixel(y as u32, x as u32, Rgb([r, g, b]));
            }
        }
        let _ = img.save(filepath);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn camera(vop: Arc<VOP>) -> Camera {
        Camera {
            origin: Point3::new(0.0, 0.0, 0.0),
            gaze: Vector3::new(0.0, 1.0, 0.0),
            up: Vector3::new(0.0, 0.0, 1.0),
            fov: [20.0, 30.0],
            density: 1.0,
            vop,
        }
    }

    #[test]
    fn screen_resolution() {
        let air = Arc::new(VOP {
            ior: 1.0,
            abs: [0.0; 3],
        });
        assert_eq!(camera(air).screen_resolution(), (20, 30))
    }

    #[test]
    fn screen_size() {
        let air = Arc::new(VOP {
            ior: 1.0,
            abs: [0.0; 3],
        });
        let calc = camera(air).screen_size();
        let theo = (0.35265, 0.53590);
        assert!((calc.0 - theo.0).abs() <= 1e-5);
        assert!((calc.1 - theo.1).abs() <= 1e-5);
    }

    #[test]
    fn ray_centering() {
        let air = VOP {
            ior: 1.0,
            abs: [0.0, 0.0, 0.0],
        };
        let camera = Camera {
            origin: Point3::new(0.0, 0.0, 0.0),
            gaze: Vector3::y(),
            up: Vector3::z(),
            fov: [1.0, 1.0],
            density: 1.0,
            vop: Arc::new(air),
        };
        let centers = camera.pixel_centers();
        assert_eq!(centers.len(), 1);
        assert_eq!(centers[0], Point3::new(0.0, 1.0, 0.0));
    }
}
