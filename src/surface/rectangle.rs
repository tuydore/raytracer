use {
    super::{Surface, SurfaceBuilder},
    crate::{shape::RectangleShape, Shape, SOP, VOP},
    collections::HashMap,
    nalgebra::{Point3, Unit, Vector3},
    serde::Deserialize,
    std::collections,
    std::sync::Arc,
};

pub struct Rectangle {
    pub geometry: RectangleShape,
    pub sop: SOP,
    pub vop_above: Arc<VOP>,
    pub vop_below: Arc<VOP>,
}

#[derive(Deserialize)]
pub struct RectangleBuilder {
    pub origin: [f64; 3],
    pub normal: [f64; 3],
    pub sop: SOP,
    pub size: [f64; 2],
    pub orientation: [f64; 3],
    pub vop_below: String,
    pub vop_above: String,
}

impl Surface for Rectangle {
    fn geometry(&self) -> &dyn Shape {
        &self.geometry
    }
    fn vop_above_at(&self, _point: &Point3<f64>) -> Arc<VOP> {
        self.vop_above.clone()
    }
    fn vop_below_at(&self, _point: &Point3<f64>) -> Arc<VOP> {
        self.vop_below.clone()
    }
    fn sop_at(&self, _point: &Point3<f64>) -> SOP {
        self.sop
    }
}

impl SurfaceBuilder for RectangleBuilder {
    fn build(self, vop_map: &HashMap<String, Arc<VOP>>) -> Arc<dyn Surface + Send + Sync> {
        Arc::new(Rectangle {
            geometry: RectangleShape {
                origin: Point3::from_slice(&self.origin),
                normal: Unit::new_normalize(Vector3::from_row_slice(&self.normal)),
                orientation: Unit::new_normalize(Vector3::from_row_slice(&self.orientation)),
                size: self.size,
            },
            sop: self.sop,
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
