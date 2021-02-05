use {
    super::{
        super::{Shape, Surface, SurfaceBuilder},
        PlaneShape,
    },
    crate::{Ray, SOP, VOP},
    collections::HashMap,
    nalgebra::{Point3, Unit, Vector3},
    serde::Deserialize,
    std::collections,
    std::sync::Arc,
};

pub struct Checkerboard {
    geometry: PlaneShape,
    sop: SOP,
    orientation: Unit<Vector3<f64>>,
    tile_size: f64,
    vop_above: Arc<VOP>,
    vop_below: Arc<VOP>,
}

#[derive(Deserialize)]
pub struct CheckerboardBuilder {
    pub origin: [f64; 3],
    pub normal: [f64; 3],
    pub sop: SOP,
    pub orientation: [f64; 3],
    pub tile_size: f64,
    pub vop_below: String,
    pub vop_above: String,
}

impl Surface for Checkerboard {
    fn intersection(&self, ray: &Ray) -> Option<Point3<f64>> {
        self.geometry.intersection(ray)
    }
    fn unchecked_normal_at(&self, point: &Point3<f64>) -> Unit<Vector3<f64>> {
        self.geometry.unchecked_normal_at(point)
    }
    fn unchecked_vop_above_at(&self, _: &Point3<f64>) -> Arc<VOP> {
        self.vop_above.clone()
    }
    fn unchecked_vop_below_at(&self, _: &Point3<f64>) -> Arc<VOP> {
        self.vop_below.clone()
    }
    fn unchecked_sop_at(&self, point: &Point3<f64>) -> SOP {
        let y = self
            .unchecked_normal_at(point)
            .cross(&self.orientation)
            .normalize();
        let x = self.orientation;
        let from_origin = *point - self.geometry.origin;

        let size_x = from_origin.dot(&x) / self.tile_size;
        let size_y = from_origin.dot(&y) / self.tile_size;
        if (size_x.floor() as i64 + size_y.floor() as i64) % 2 == 0 {
            self.sop
        } else {
            SOP::Dark
        }
    }
}

impl SurfaceBuilder for CheckerboardBuilder {
    fn build(self, vop_map: &HashMap<String, Arc<VOP>>) -> Arc<dyn Surface + Send + Sync> {
        Arc::new(Checkerboard {
            geometry: PlaneShape::new(
                Point3::from_slice(&self.origin),
                Vector3::from_row_slice(&self.normal),
                Some(Vector3::from_row_slice(&self.orientation)),
            ),
            orientation: Unit::new_normalize(Vector3::from_row_slice(&self.orientation)),
            sop: self.sop,
            tile_size: self.tile_size,
            vop_above: vop_map
                .get(&self.vop_above)
                .expect("No VOP above mapping found.")
                .clone(),
            vop_below: vop_map
                .get(&self.vop_below)
                .expect("No VOP above mapping found.")
                .clone(),
        })
    }
}
