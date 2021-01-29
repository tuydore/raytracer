use {
    super::{Surface, SurfaceBuilder},
    crate::{shape::SphereShape, Point3D, Shape, Vector3D, SOP, VOP},
    collections::HashMap,
    num_complex::Complex,
    serde::Deserialize,
    std::collections,
    std::f64::consts::PI,
    std::sync::Arc,
};

pub struct MandelbrotSphere {
    pub geometry: SphereShape,
    pub vop_above: Arc<VOP>,
    pub vop_below: Arc<VOP>,
    pub vertical: Vector3D,
    pub to_gmt_equator: Vector3D,
    pub mandelbrot_origin: [f64; 2],
    pub mandelbrot_scale: f64,
    pub mandelbrot_maxiter: usize,
}

#[derive(Deserialize)]
pub struct MandelbrotSphereBuilder {
    pub center: [f64; 3],
    pub radius: f64,
    pub vop_below: String,
    pub vop_above: String,
    pub vertical: [f64; 3],
    pub to_gmt_equator: [f64; 3],
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

fn mandelbrot(x: f64, y: f64, max_iter: usize) -> usize {
    let coord = Complex::new(x, y);
    let mut z = Complex::new(0.0, 0.0);
    let mut n = 0;
    while z.norm() <= 2.0 && n < max_iter {
        z = z.powu(2) + coord;
        n += 1;
    }
    n
}

impl Surface for MandelbrotSphere {
    fn geometry(&self) -> &dyn Shape {
        // impl here instead?
        &self.geometry
    }
    fn vop_above_at(&self, _point: &Point3D) -> Arc<VOP> {
        self.vop_above.clone()
    }
    fn vop_below_at(&self, _point: &Point3D) -> Arc<VOP> {
        self.vop_below.clone()
    }
    fn sop_at(&self, point: &Point3D) -> SOP {
        let (lon, lat) = Self::unchecked_longitude_latitude(&self, point);
        let (x, y) = self.mercator_projection(lon, lat);
        let val = self.mandelbrot_scaled(x, y, self.mandelbrot_maxiter);
        let color = 255 - (val * 255 / self.mandelbrot_maxiter) as u8;
        SOP::Light(color, color, color)
    }
}

impl MandelbrotSphere {
    /// Find the longitude and latitude of a given point, assuming it is on the sphere.
    fn unchecked_longitude_latitude(&self, point: &Point3D) -> (f64, f64) {
        // vector from origin to surface
        let v = (*point - self.geometry.center).normalized();
        // find projection along vertical axis
        let cos_theta = v.dot(&self.vertical.normalized());
        let v_vertical: Vector3D = self.vertical * cos_theta;
        // find length of projection on equator slice
        let v_equator = v - v_vertical;
        let cos_phi = v_equator.dot(&self.to_gmt_equator);
        (PI / 2.0 - cos_theta.acos(), cos_phi.acos())
    }
    /// Convert a set of longitude and latitude to a Mercator projection
    fn mercator_projection(&self, longitude: f64, latitude: f64) -> (f64, f64) {
        let r = self.geometry.radius;
        let x = r * longitude;
        let y = r * (PI / 4.0 + latitude / 2.0).tan().ln();
        (x, y)
    }
    /// Get value of Mandelbrot fractal at given coordinate, as function of number of iterations.
    fn mandelbrot_scaled(&self, x: f64, y: f64, max_iter: usize) -> usize {
        mandelbrot(
            (x - self.mandelbrot_origin[0]) / self.mandelbrot_scale,
            (y - self.mandelbrot_origin[1]) / self.mandelbrot_scale,
            max_iter,
        )
    }
}

impl SurfaceBuilder for MandelbrotSphereBuilder {
    fn build(self, vop_map: &HashMap<String, Arc<VOP>>) -> Arc<dyn Surface + Send + Sync> {
        Arc::new(MandelbrotSphere {
            geometry: SphereShape {
                center: Point3D {
                    x: self.center[0],
                    y: self.center[1],
                    z: self.center[2],
                },
                radius: self.radius,
            },
            vop_above: vop_map
                .get(&self.vop_above)
                .expect("No VOP above mapping found.")
                .clone(),
            vop_below: vop_map
                .get(&self.vop_below)
                .expect("No VOP above mapping found.")
                .clone(),
            vertical: Vector3D::new(self.vertical[0], self.vertical[1], self.vertical[2]),
            to_gmt_equator: Vector3D::new(
                self.to_gmt_equator[0],
                self.to_gmt_equator[1],
                self.to_gmt_equator[2],
            ),
            mandelbrot_origin: self.mandelbrot_origin,
            mandelbrot_scale: self.mandelbrot_scale,
            mandelbrot_maxiter: self.mandelbrot_maxiter,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::EPSILON;

    fn mandelbrot_sphere(vop: Arc<VOP>) -> MandelbrotSphere {
        MandelbrotSphere {
            geometry: SphereShape {
                center: Point3D::new(0.0, 0.0, 0.0),
                radius: 2.0,
            },
            vop_above: vop.clone(),
            vop_below: vop,
            vertical: Vector3D::pz(),
            to_gmt_equator: Vector3D::px(),
            mandelbrot_origin: [0.0, 0.0],
            mandelbrot_scale: 1.0,
            mandelbrot_maxiter: 1,
        }
    }

    #[test]
    fn longitude_latitude() {
        let air = Arc::new(VOP {
            ior: 1.0,
            abs: [0.0, 0.0, 0.0],
        });
        let mbs = mandelbrot_sphere(air);
        assert_eq!(
            mbs.unchecked_longitude_latitude(&Point3D::new(2.0, 0.0, 0.0)),
            (0.0, 0.0)
        );
        assert!(
            mbs.unchecked_longitude_latitude(&Point3D::new(0.0, 0.0, 2.0))
                .0
                - PI / 2.0
                <= EPSILON
        );
        assert_eq!(
            mbs.unchecked_longitude_latitude(&Point3D::new(0.0, 2.0, 0.0)),
            (0.0, PI / 2.0)
        );
    }

    #[test]
    fn test_mandelbrot() {
        assert_eq!(mandelbrot(0.0, 0.0, 100), 100);
        assert_eq!(mandelbrot(-1.0, 0.0, 100), 100);
        assert_eq!(mandelbrot(-2.0, 1.0, 100), 1);
    }
}
