use {
    super::{Surface, SurfaceBuilder},
    crate::{shape::SphereShape, Point3D, Shape, SOP, VOP},
    collections::HashMap,
    serde::Deserialize,
    std::collections,
};

pub struct Sphere<'a> {
    pub geometry: SphereShape,
    pub sop: SOP,
    pub vop_above: &'a VOP,
    pub vop_below: &'a VOP,
}

#[derive(Deserialize)]
pub struct SphereBuilder {
    pub center: [f64; 3],
    pub radius: f64,
    pub sop: SOP,
    pub vop_below: String,
    pub vop_above: String,
}

impl<'a> Surface<'a> for Sphere<'a> {
    fn geometry(&self) -> &dyn Shape {
        &self.geometry
    }
    fn vop_above_at(&self, _point: &Point3D) -> &'a VOP {
        &self.vop_above
    }
    fn vop_below_at(&self, _point: &Point3D) -> &'a VOP {
        &self.vop_below
    }
    fn sop_at(&self, _point: &Point3D) -> SOP {
        self.sop
    }
}

impl<'a> SurfaceBuilder<'a> for SphereBuilder {
    fn build(self, vop_map: &'a HashMap<String, VOP>) -> Box<dyn Surface + 'a> {
        Box::new(Sphere {
            geometry: SphereShape {
                center: Point3D {
                    x: self.center[0],
                    y: self.center[1],
                    z: self.center[2],
                },
                radius: self.radius,
            },
            sop: self.sop,
            vop_above: vop_map
                .get(&self.vop_above)
                .expect("No VOP above mapping found."),
            vop_below: vop_map
                .get(&self.vop_below)
                .expect("No VOP above mapping found."),
        })
    }
}
