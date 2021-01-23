use {
    super::{Surface, SurfaceBuilder},
    crate::{shape::InfinitePlaneShape, Point3D, Shape, Vector3D, SOP, VOP},
    collections::HashMap,
    serde::Deserialize,
    std::collections,
    std::sync::Arc,
};

pub struct Checkerboard<'a> {
    geometry: InfinitePlaneShape,
    sop: SOP,
    orientation: Vector3D,
    tile_size: f64,
    vop_above: &'a VOP,
    vop_below: &'a VOP,
}

#[derive(Deserialize)]
pub struct CheckerboardBuilder {
    pub origin: [f64; 3],
    pub normal: [f64; 3],
    pub sop: SOP,
    pub orientation: [f64; 3],
    pub tile_size: f64,
    pub vop_below: String,
    pub vop_above: String,
}

impl<'a> Surface<'a> for Checkerboard<'a> {
    fn geometry(&self) -> &dyn Shape {
        &self.geometry
    }
    fn vop_above_at(&self, _: &Point3D) -> &'a VOP {
        &self.vop_above
    }
    fn vop_below_at(&self, _: &Point3D) -> &'a VOP {
        &self.vop_below
    }
    fn sop_at(&self, point: &Point3D) -> SOP {
        let y = self
            .geometry
            .normal_at(point)
            .unwrap()
            .cross(&self.orientation)
            .normalized();
        let x = self.orientation.normalized();
        let from_origin = *point - self.geometry().origin();

        let size_x = from_origin.dot(&x) / self.tile_size;
        let size_y = from_origin.dot(&y) / self.tile_size;
        if (size_x.floor() as i64 + size_y.floor() as i64) % 2 == 0 {
            self.sop
        } else {
            SOP::Dark
        }
    }
}

impl SurfaceBuilder for CheckerboardBuilder {
    fn build<'a>(self, vop_map: &'a HashMap<String, VOP>) -> Arc<dyn Surface + 'a> {
        Arc::new(Checkerboard {
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
            orientation: Vector3D {
                x: self.orientation[0],
                y: self.orientation[1],
                z: self.orientation[2],
            },
            sop: self.sop,
            tile_size: self.tile_size,
            vop_above: vop_map
                .get(&self.vop_above)
                .expect("No VOP above mapping found."),
            vop_below: vop_map
                .get(&self.vop_below)
                .expect("No VOP above mapping found."),
        })
    }
}
