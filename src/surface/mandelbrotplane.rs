use {
    super::{Surface, SurfaceBuilder},
    crate::{shape::InfinitePlaneShape, Shape, SOP, VOP},
    collections::HashMap,
    nalgebra::{Point3, Unit, Vector3},
    num_complex::Complex,
    scarlet::colormap::ListedColorMap,
    serde::Deserialize,
    std::collections,
    std::sync::Arc,
};

pub struct MandelbrotPlane {
    pub geometry: InfinitePlaneShape,
    pub orientation: Unit<Vector3<f64>>,
    pub vop_above: Arc<VOP>,
    pub vop_below: Arc<VOP>,
    pub colormap: Vec<[f64; 3]>,
    pub mandelbrot_scale: f64,
    pub mandelbrot_maxiter: usize,
    pub mandelbrot_origin: [f64; 2],
}

#[derive(Deserialize)]
pub struct MandelbrotPlaneBuilder {
    pub origin: [f64; 3],
    pub normal: [f64; 3],
    pub orientation: [f64; 3],
    pub colormap: String,
    pub vop_below: String,
    pub vop_above: String,
    #[serde(default = "default_mandelbrot_origin")]
    pub mandelbrot_origin: [f64; 2],
    #[serde(default = "default_mandelbrot_scale")]
    pub mandelbrot_scale: f64,
    #[serde(default = "default_mandelbrot_maxiter")]
    pub mandelbrot_maxiter: usize,
}

fn default_mandelbrot_origin() -> [f64; 2] {
    [0.0, 0.0]
}

fn default_mandelbrot_scale() -> f64 {
    1.0
}

fn default_mandelbrot_maxiter() -> usize {
    50
}

pub fn mandelbrot(x: f64, y: f64, max_iter: usize) -> usize {
    let coord = Complex::new(x, y);
    let mut z = Complex::new(0.0, 0.0);
    let mut n = 0;
    while z.norm() <= 2.0 && n < max_iter {
        z = z.powu(2) + coord;
        n += 1;
    }
    n
}

impl SurfaceBuilder for MandelbrotPlaneBuilder {
    fn build(self, vop_map: &HashMap<String, Arc<VOP>>) -> Arc<dyn Surface + Send + Sync> {
        Arc::new(MandelbrotPlane {
            geometry: InfinitePlaneShape {
                origin: Point3::from_slice(&self.origin),
                normal: Unit::new_normalize(Vector3::from_row_slice(&self.normal)),
            },
            orientation: Unit::new_normalize(Vector3::from_row_slice(&self.orientation)),
            colormap: match self.colormap.as_str() {
                "viridis" => ListedColorMap::viridis().vals,
                "magma" => ListedColorMap::magma().vals,
                "inferno" => ListedColorMap::inferno().vals,
                "plasma" => ListedColorMap::plasma().vals,
                s => panic!("Unknown color map {}", s),
            },
            vop_above: vop_map
                .get(&self.vop_above)
                .expect("No VOP above mapping found.")
                .clone(),
            vop_below: vop_map
                .get(&self.vop_below)
                .expect("No VOP above mapping found.")
                .clone(),
            mandelbrot_origin: self.mandelbrot_origin,
            mandelbrot_maxiter: self.mandelbrot_maxiter,
            mandelbrot_scale: self.mandelbrot_scale,
        })
    }
}

impl Surface for MandelbrotPlane {
    fn geometry(&self) -> &dyn Shape {
        &self.geometry
    }
    fn vop_above_at(&self, _: &Point3<f64>) -> Arc<VOP> {
        self.vop_above.clone()
    }
    fn vop_below_at(&self, _: &Point3<f64>) -> Arc<VOP> {
        self.vop_below.clone()
    }
    fn sop_at(&self, point: &Point3<f64>) -> SOP {
        // intersection with plane
        let y: Vector3<f64> = self
            .geometry
            .normal_at(point)
            .unwrap()
            .cross(&self.orientation)
            .normalize();
        let x: Vector3<f64> = self.orientation.normalize(); // TODO: should be by default
        let from_origin: Vector3<f64> = *point - self.geometry().origin();

        let xproj = from_origin.dot(&x);
        let yproj = from_origin.dot(&y);
        let x = mandelbrot(
            xproj / self.mandelbrot_scale - self.mandelbrot_origin[0],
            yproj / self.mandelbrot_scale - self.mandelbrot_origin[1],
            // (xproj - self.mandelbrot_origin[0]) / self.mandelbrot_scale,
            // (yproj - self.mandelbrot_origin[1]) / self.mandelbrot_scale,
            self.mandelbrot_maxiter,
        );
        if x == self.mandelbrot_maxiter {
            SOP::Dark
        } else {
            let rgb_float: [f64; 3] =
                self.colormap[self.colormap.len() * x / self.mandelbrot_maxiter];
            SOP::Light(
                (rgb_float[0] * 255.0) as u8,
                (rgb_float[1] * 255.0) as u8,
                (rgb_float[2] * 255.0) as u8,
            )
        }
    }
}
