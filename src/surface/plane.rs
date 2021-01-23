use {
    super::{Surface, SurfaceBuilder},
    crate::{shape::InfinitePlaneShape, Point3D, Shape, Vector3D, SOP, VOP},
    collections::HashMap,
    serde::Deserialize,
    std::collections,
    std::sync::Arc,
};

pub struct Plane {
    pub geometry: InfinitePlaneShape,
    pub sop: SOP,
    pub vop_above: Arc<VOP>,
    pub vop_below: Arc<VOP>,
}

#[derive(Deserialize)]
pub struct PlaneBuilder {
    pub origin: [f64; 3],
    pub normal: [f64; 3],
    pub sop: SOP,
    pub vop_below: String,
    pub vop_above: String,
}

impl SurfaceBuilder for PlaneBuilder {
    fn build(self, vop_map: &HashMap<String, Arc<VOP>>) -> Arc<dyn Surface> {
        Arc::new(Plane {
            geometry: InfinitePlaneShape {
                origin: Point3D {
                    x: self.origin[0],
                    y: self.origin[1],
                    z: self.origin[2],
                },
                normal: Vector3D {
                    x: self.normal[0],
                    y: self.normal[1],
                    z: self.normal[2],
                },
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

impl Surface for Plane {
    fn geometry(&self) -> &dyn Shape {
        &self.geometry
    }
    fn vop_above_at(&self, _: &Point3D) -> Arc<VOP> {
        self.vop_above.clone()
    }
    fn vop_below_at(&self, _: &Point3D) -> Arc<VOP> {
        self.vop_below.clone()
    }
    fn sop_at(&self, _: &Point3D) -> SOP {
        self.sop
    }
}
