use {
    super::{
        super::{Shape, Surface, SurfaceBuilder},
        RectangleShape,
    },
    crate::{Ray, SOP, VOP},
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
    fn intersection(&self, ray: &Ray) -> Option<Point3<f64>> {
        self.geometry.intersection(ray)
    }
    fn unchecked_normal_at(&self, point: &Point3<f64>) -> Unit<Vector3<f64>> {
        self.geometry.unchecked_normal_at(point)
    }
    fn unchecked_vop_above_at(&self, _point: &Point3<f64>) -> Arc<VOP> {
        self.vop_above.clone()
    }
    fn unchecked_vop_below_at(&self, _point: &Point3<f64>) -> Arc<VOP> {
        self.vop_below.clone()
    }
    fn unchecked_sop_at(&self, _point: &Point3<f64>) -> SOP {
        self.sop
    }
}

impl SurfaceBuilder for RectangleBuilder {
    fn build(self, vop_map: &HashMap<String, Arc<VOP>>) -> Arc<dyn Surface + Send + Sync> {
        Arc::new(Rectangle {
            geometry: RectangleShape::new(
                Point3::from_slice(&self.origin),
                Vector3::from_row_slice(&self.normal),
                Vector3::from_row_slice(&self.orientation),
                self.size,
            ),
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
