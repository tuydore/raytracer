use {
    super::{Surface, SurfaceBuilder},
    crate::{shape::InfinitePlaneShape, Shape, SOP, VOP},
    collections::HashMap,
    nalgebra::{Point3, Vector3},
    serde::Deserialize,
    std::collections,
    std::sync::Arc,
};

pub struct Checkerboard {
    geometry: InfinitePlaneShape,
    sop: SOP,
    orientation: Vector3<f64>,
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
        let y = self
            .geometry
            .normal_at(point)
            .unwrap()
            .cross(&self.orientation)
            .normalize();
        let x = self.orientation.normalize(); // TODO: should be by default
        let from_origin = *point - self.geometry().origin();

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
            geometry: InfinitePlaneShape {
                origin: Point3::from_slice(&self.origin),
                normal: Vector3::from_row_slice(&self.normal),
            },
            orientation: Vector3::from_row_slice(&self.orientation),
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
