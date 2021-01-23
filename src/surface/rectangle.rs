use {
    super::{Surface, SurfaceBuilder},
    crate::{shape::RectangleShape, Point3D, Shape, Vector3D, SOP, VOP},
    collections::HashMap,
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
    fn vop_above_at(&self, _point: &Point3D) -> Arc<VOP> {
        self.vop_above.clone()
    }
    fn vop_below_at(&self, _point: &Point3D) -> Arc<VOP> {
        self.vop_below.clone()
    }
    fn sop_at(&self, _point: &Point3D) -> SOP {
        self.sop
    }
}

impl SurfaceBuilder for RectangleBuilder {
    fn build(self, vop_map: &HashMap<String, Arc<VOP>>) -> Arc<dyn Surface + Send + Sync> {
        Arc::new(Rectangle {
            geometry: RectangleShape {
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
                orientation: Vector3D {
                    x: self.orientation[0],
                    y: self.orientation[1],
                    z: self.orientation[2],
                },
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
