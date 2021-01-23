use crate::{ray::BounceResult, Point3D, Ray, Surface, Vector3D, VOP};
use image::{Rgb, RgbImage};
use indicatif::ProgressBar;
use serde::Deserialize;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Instant;

#[derive(Debug)]
pub struct Camera {
    origin: Point3D,
    gaze: Vector3D,
    up: Vector3D,
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
            origin: Point3D {
                x: self.origin[0],
                y: self.origin[1],
                z: self.origin[2],
            },
            gaze: Vector3D {
                x: self.gaze[0],
                y: self.gaze[1],
                z: self.gaze[2],
            },
            up: Vector3D {
                x: self.up[0],
                y: self.up[1],
                z: self.up[2],
            },
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

    fn screen_upper_left_corner(&self) -> Point3D {
        let (sizex, sizey) = self.screen_size();
        let g = self.gaze.normalized();
        let u = self.up.normalized();
        self.origin + g + u * sizex / 2.0 + u.cross(&g) * sizey / 2.0
    }

    fn pixel_centers(&self) -> Vec<Point3D> {
        // orientation vectors r==right, d==down
        let g = self.gaze.normalized();
        let u = self.up.normalized();
        let r = g.cross(&u);
        let d = -1.0 * u;

        // obtain screen density
        let (sizex, sizey) = self.screen_size();
        let (npx, npy) = self.screen_resolution();
        let pixel_size_x = sizex / npx as f64;
        let pixel_size_y = sizey / npy as f64;

        // starting point is the center of the top-left PIXEL, not the screen itself
        let upper_left_pixel_center =
            self.screen_upper_left_corner() + r * pixel_size_y + d * pixel_size_x;

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
    pub fn look(&self, scene: &[Arc<dyn Surface>]) -> Vec<(u8, u8, u8)> {
        let mut result = Vec::new();

        // instantiate progress bar
        let num_rays: u64 = self.pixel_centers().len() as u64;
        println!("Launching {} rays...", num_rays);
        let pbar = ProgressBar::new(num_rays);
        pbar.set_draw_delta(num_rays / 100);

        // start timer
        let t0 = Instant::now();

        for (_i, pxc) in self.pixel_centers().into_iter().enumerate() {
            // println!("{}", i);
            // launch a ray towards each pixel's center
            let mut ray = Ray {
                origin: self.origin,
                direction: pxc - self.origin,
                vop: self.vop.clone(),
            };
            result.push(match ray.launch(scene) {
                BounceResult::Count(r, g, b) => (r, g, b),
                BounceResult::Kill => (0, 0, 0),
                _ => panic!("Something has gone wrong."),
            });
            pbar.inc(1);
        }
        let seconds = t0.elapsed().as_millis() as f64 / 1000.0;
        pbar.finish_and_clear();

        // show total time
        println!(
            "Raytrace time: {}s, rays/s: {}.",
            seconds,
            num_rays / seconds as u64
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
            origin: Point3D::new(0.0, 0.0, 0.0),
            gaze: Vector3D::new(0.0, 1.0, 0.0),
            up: Vector3D::new(0.0, 0.0, 1.0),
            fov: [20.0, 30.0],
            density: 1.0,
            vop,
        }
    }

    #[test]
    fn screen_resolution() {
        let air = Arc::new(VOP { ior: 1.0 });
        assert_eq!(camera(air).screen_resolution(), (20, 30))
    }

    #[test]
    fn screen_size() {
        let air = Arc::new(VOP { ior: 1.0 });
        let calc = camera(air).screen_size();
        let theo = (0.35265, 0.53590);
        assert!((calc.0 - theo.0).abs() <= 1e-5);
        assert!((calc.1 - theo.1).abs() <= 1e-5);
    }
}
