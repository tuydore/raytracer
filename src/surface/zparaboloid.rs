use {
    super::{Surface, SurfaceBuilder},
    crate::{shape::ParaboloidShape, Point3D, Shape, SOP, VOP},
    collections::HashMap,
    serde::Deserialize,
    std::collections,
    std::sync::Arc,
};

pub struct ZParaboloid<'a> {
    geometry: ParaboloidShape,
    sop: SOP,
    vop_above: &'a VOP,
    vop_below: &'a VOP,
}

#[derive(Deserialize)]
pub struct ZParaboloidBuilder {
    pub origin: [f64; 3],
    pub a: f64,
    pub b: f64,
    pub sop: SOP,
    pub vop_above: String,
    pub vop_below: String,
}

impl<'a> Surface<'a> for ZParaboloid<'a> {
    fn geometry(&self) -> &dyn Shape {
        &self.geometry
    }

    fn vop_above_at(&self, _: &Point3D) -> &'a VOP {
        &self.vop_above
    }

    fn vop_below_at(&self, _: &Point3D) -> &'a VOP {
        &self.vop_below
    }

    fn sop_at(&self, _: &Point3D) -> SOP {
        self.sop
    }
}

impl SurfaceBuilder for ZParaboloidBuilder {
    fn build<'a>(self, vop_map: &'a HashMap<String, VOP>) -> Arc<dyn Surface + 'a> {
        Arc::new(ZParaboloid {
            geometry: ParaboloidShape {
                x0: self.origin[0],
                y0: self.origin[1],
                z0: self.origin[2],
                asq: self.a.powi(2),
                bsq: self.b.powi(2),
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
