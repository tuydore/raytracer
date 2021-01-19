pub mod surfaces;

use crate::{Point3D, Shape, VOP};

#[derive(Debug, Clone, Copy)]
pub enum SOP {
    Reflect,
    Refract,
    Light(u8, u8, u8),
    Dark,
}

pub trait Surface<'a> {
    fn geometry(&self) -> &dyn Shape;

    fn vop_above_at(&self, point: &Point3D) -> &'a VOP;

    fn vop_below_at(&self, point: &Point3D) -> &'a VOP;

    fn sop_at(&self, point: &Point3D) -> SOP;
}
