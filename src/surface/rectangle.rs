use {
    super::{Surface, SurfaceBuilder},
    crate::{shape::RectangleShape, Point3D, Shape, Vector3D, SOP, VOP},
    collections::HashMap,
    serde::Deserialize,
    std::collections,
    std::sync::Arc,
};

pub struct Rectangle<'a> {
    pub geometry: RectangleShape,
    pub sop: SOP,
    pub vop_above: &'a VOP,
    pub vop_below: &'a VOP,
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

impl<'a> Surface<'a> for Rectangle<'a> {
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

impl SurfaceBuilder for RectangleBuilder {
    fn build<'a>(self, vop_map: &'a HashMap<String, VOP>) -> Arc<dyn Surface + 'a> {
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
                .expect("No VOP above mapping found."),
            vop_below: vop_map
                .get(&self.vop_below)
                .expect("No VOP above mapping found."),
        })
    }
}
