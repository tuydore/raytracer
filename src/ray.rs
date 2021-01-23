use crate::{Point3D, Surface, Vector3D, SOP, VOP};
use std::error::Error;

#[derive(Debug, Clone)]
pub struct Ray<'a> {
    pub origin: Point3D,
    pub direction: Vector3D,
    pub vop: &'a VOP,
}

impl<'a> Ray<'a> {
    // TODO: avoid repetition by calculating first_intersection twice
    /// Launch a ray through the system and fetch its final return value.
    pub fn launch(&mut self, surfaces: &'a [&dyn Surface]) -> BounceResult {
        loop {
            // get all first intersections with surfaces and distances to them
            let intersections: Vec<Option<(Point3D, f64)>> = surfaces
                .iter()
                .map(|s| s.geometry().intersection(self))
                .map(|p| {
                    if let Some(point) = p {
                        Some((point, point.distance_squared_to(&self.origin)))
                    } else {
                        None
                    }
                })
                .collect();

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
            match self.bounce_unchecked(surfaces[index], &intersections[index].unwrap().0) {
                BounceResult::Continue => continue,
                BounceResult::Error => panic!("Something went wrong!"),
                br => return br,
            }
        }
    }

    /// Analyze a ray incoming on a surface and determine the normal on the side of the incoming ray.
    /// If no errors are found, return intersection point, that normal and the above & below VOPs.
    /// Otherwise return an error.
    fn get_interaction_parameters_unchecked(
        &self,
        surface: &'a dyn Surface,
        point: &Point3D,
    ) -> Result<(Vector3D, &'a VOP, &'a VOP), Box<dyn Error>> {
        // get VOPs above and below
        let vop_above = surface.vop_above_at(&point);
        let vop_below = surface.vop_below_at(&point);

        // get normal at point
        let normal = surface
            .geometry()
            .normal_at(&point)
            .expect("No normal found at point.");

        // ray is inbound from medium into which normal points
        if normal.dot(&self.direction) <= 0.0 {
            // check that ray VOP and above VOP match
            if self.vop != vop_above {
                panic!(
                    "VOP mismatch:\nray: {:#?}\nfrom: {:?}\ninto: {:?}\nintersection: {:?}\nnormal: {:?}",
                    self,
                    vop_below,
                    vop_above,
                    point,
                    normal,
                )
            }
            Ok((normal, vop_above, vop_below))
        // ray is inbound from other side of boundary
        } else {
            if self.vop != vop_below {
                panic!(
                    "VOP mismatch:\nray: {:#?}\nfrom: {:?}\ninto: {:?}\nintersection: {:?}\nnormal: {:?}",
                    self,
                    vop_below,
                    vop_above,
                    point,
                    normal,
                )
            }
            Ok((-1.0 * normal, vop_below, vop_above))
        }
    }

    pub fn bounce_unchecked(&mut self, surface: &'a dyn Surface, point: &Point3D) -> BounceResult {
        let sop = surface.sop_at(point);
        match sop {
            SOP::Reflect => {
                if let Ok((normal, _, _)) =
                    self.get_interaction_parameters_unchecked(surface, point)
                {
                    self.reflect(point, &normal);
                    return BounceResult::Continue;
                }
                BounceResult::Error
            }
            SOP::Refract => {
                if let Ok((normal, vop_above, vop_below)) =
                    self.get_interaction_parameters_unchecked(surface, point)
                {
                    self.refract(point, &normal, &vop_above, &vop_below);
                    return BounceResult::Continue;
                }
                BounceResult::Error
            }
            SOP::Light(r, g, b) => BounceResult::Count(r, g, b),
            SOP::Dark => BounceResult::Kill,
        }
    }

    /// Reflect a ray in a surface.
    fn reflect(&mut self, intersection: &Point3D, normal: &Vector3D) {
        self.origin = *intersection;
        self.direction +=
            2.0 * self.direction.dot(&normal).abs() / normal.length_squared() * *normal;
    }

    /// Refract a ray in a surface.
    /// Reference: https://graphics.stanford.edu/courses/cs148-10-summer/docs/2006--degreve--reflection_refraction.pdf
    fn refract(
        &mut self,
        intersection: &Point3D,
        normal: &Vector3D,
        vop_above: &'a VOP,
        vop_below: &'a VOP,
    ) {
        // ratio of n_above / n_below
        let nanb = vop_above.ior / vop_below.ior;

        // normal to surface at new contact point
        let normal = normal.normalized();
        self.direction = self.direction.normalized();
        let cos_theta_i = normal.dot(&self.direction.reversed());
        let sin_sq_theta_t = nanb.powi(2) * (1.0 - cos_theta_i.powi(2));

        // critical angle
        if sin_sq_theta_t >= 1.0 {
            return self.reflect(intersection, &normal);
        }

        // update ray origin to point of intersection
        self.origin = *intersection;
        // update ray direction
        self.direction =
            self.direction * nanb + normal * (nanb * cos_theta_i - (1.0 - sin_sq_theta_t).sqrt());

        // update ray VOP
        self.vop = vop_below;
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
