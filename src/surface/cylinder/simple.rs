use {
    super::{
        super::{Shape, Surface, SurfaceBuilder},
        CylinderShape,
    },
    crate::{Ray, SOP, VOP},
    collections::HashMap,
    nalgebra::{Point3, Unit, Vector3},
    serde::Deserialize,
    std::collections,
    std::sync::Arc,
};

pub struct Cylinder {
    pub geometry: CylinderShape,
    pub sop: SOP,
    pub vop_above: Arc<VOP>,
    pub vop_below: Arc<VOP>,
}

#[derive(Deserialize)]
pub struct CylinderBuilder {
    pub origin: [f64; 3],
    pub height: f64,
    pub radius: f64,
    pub direction: [f64; 3],
    pub sop: SOP,
    pub vop_below: String,
    pub vop_above: String,
}

impl Surface for Cylinder {
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

impl SurfaceBuilder for CylinderBuilder {
    fn build(self, vop_map: &HashMap<String, Arc<VOP>>) -> Arc<dyn Surface + Send + Sync> {
        Arc::new(Cylinder {
            geometry: CylinderShape::new(
                Point3::from_slice(&self.origin),
                Vector3::from_row_slice(&self.direction),
                self.height,
                self.radius,
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
