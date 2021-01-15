use crate::geometry::{Point3D, Vector3D, VOP};
pub struct Ray {
    pub origin: Point3D,
    pub direction: Vector3D,
    pub vop: VOP,
}

impl Ray {
    pub fn new(origin: Point3D, direction: Vector3D, vop: VOP) -> Self {
        Self {
            origin,
            direction,
            vop,
        }
    }
}
