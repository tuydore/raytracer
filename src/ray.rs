use crate::geometry::{Point3D, Shape, Vector3D, VOP};
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

    // TODO: avoid repetition by calculating first_intersection twice
    /// Launch a ray through the system and fetch its final return value.
    pub fn launch(&mut self, shapes: &[Box<dyn Shape>]) -> BounceResult {
        loop {
            // get all first intersections with surfaces and distances to them
            let intersections: Vec<Option<(Point3D, f64)>> =
                shapes.iter().map(|s| s.intersection(self)).collect();

            // if no more intersections, return as Dark
            if intersections.iter().all(|x| x.is_none()) {
                return BounceResult::Kill;
            }

            // pick closest shape
            let mut closest = f64::INFINITY;
            let mut index = 0;
            for (i, opt) in intersections.iter().enumerate() {
                if opt.is_some() && opt.unwrap().1 <= closest {
                    closest = opt.unwrap().1;
                    index = i;
                }
            }

            // bounce ray off closest shape
            match shapes[index].bounce(self) {
                BounceResult::Continue => continue,
                BounceResult::Error => panic!("Something went wrong!"),
                br => return br,
            }
        }
    }
}

/// Result returned by ray bounce operation. This can beone of the following:
/// * `Count` - the ray has reached a light source and therefore must be counted.
/// * `Kill` - the ray has reached a determined "dark" spot (either due to being out-of bounds or
///     a perfectly absorbant material) and is to be gracefully terminated.
/// * `Continue` - the ray has interacted normally and can continue along its merry way.
/// * `Error` - the ray has encountered an error (for example a ray with VOP of RI=1.0 has been
///     registered as hitting a surface at VOP with RI=1.5), with custom implementation of what
///     happens in this case.
#[derive(Debug, PartialEq)]
pub enum BounceResult {
    Count(u8, u8, u8),
    Kill,
    Continue,
    Error,
}
