use {
    super::{Surface, SurfaceBuilder},
    crate::{shape::SphereShape, Shape, SOP, VOP},
    collections::HashMap,
    nalgebra::Point3,
    serde::Deserialize,
    std::collections,
    std::sync::Arc,
};

pub struct Sphere {
    pub geometry: SphereShape,
    pub sop: SOP,
    pub vop_above: Arc<VOP>,
    pub vop_below: Arc<VOP>,
}

#[derive(Deserialize)]
pub struct SphereBuilder {
    pub center: [f64; 3],
    pub radius: f64,
    pub sop: SOP,
    pub vop_below: String,
    pub vop_above: String,
}

impl Surface for Sphere {
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

impl SurfaceBuilder for SphereBuilder {
    fn build(self, vop_map: &HashMap<String, Arc<VOP>>) -> Arc<dyn Surface + Send + Sync> {
        Arc::new(Sphere {
            geometry: SphereShape {
                center: Point3::from_slice(&self.center),
                radius: self.radius,
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
