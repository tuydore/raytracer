use {
    super::{
        super::{Shape, Surface, SurfaceBuilder},
        ParaboloidShape,
    },
    crate::{Ray, SOP, VOP},
    collections::HashMap,
    nalgebra::{Point3, Unit, Vector3},
    serde::Deserialize,
    std::collections,
    std::sync::Arc,
};

pub struct Paraboloid {
    geometry: ParaboloidShape,
    sop: SOP,
    vop_above: Arc<VOP>,
    vop_below: Arc<VOP>,
}

#[derive(Deserialize)]
pub struct ParaboloidBuilder {
    pub origin: [f64; 3],
    pub normal: [f64; 3],
    pub orientation: [f64; 3],
    pub asq: f64,
    pub bsq: f64,
    pub sop: SOP,
    pub vop_above: String,
    pub vop_below: String,
}

impl Surface for Paraboloid {
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
    fn unchecked_sop_at(&self, _: &Point3<f64>) -> SOP {
        self.sop
    }
}

impl SurfaceBuilder for ParaboloidBuilder {
    fn build(self, vop_map: &HashMap<String, Arc<VOP>>) -> Arc<dyn Surface + Send + Sync> {
        Arc::new(Paraboloid {
            geometry: ParaboloidShape::new(
                Point3::from_slice(&self.origin),
                Vector3::from_row_slice(&self.normal),
                Vector3::from_row_slice(&self.orientation),
                self.asq,
                self.bsq,
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
