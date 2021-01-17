pub mod surfaces;

use crate::{BounceResult, Point3D, Ray, Shape, Vector3D, VOP};
use std::error::Error;

#[derive(Debug, Clone, Copy)]
pub enum SOP {
    Reflect,
    Refract,
    Light(u8, u8, u8),
    Dark,
}

pub trait Surface {
    fn geometry(&self) -> &dyn Shape;

    fn vop_above_at(&self, point: &Point3D) -> &VOP;

    fn vop_below_at(&self, point: &Point3D) -> &VOP;

    fn sop_at(&self, point: &Point3D) -> &SOP;

    fn bounce(&self, ray: &mut Ray) -> BounceResult {
        // TODO: clean this up
        if let Some(p) = self.geometry().intersection(ray) {
            self.bounce_at(&p, ray)
        } else {
            BounceResult::Kill
        }
    }

    fn bounce_at(&self, point: &Point3D, ray: &mut Ray) -> BounceResult {
        let sop = self.sop_at(point);
        match sop {
            SOP::Reflect => {
                if let Ok((intersection, normal, _, _)) = self.get_interaction_parameters(ray) {
                    reflect(ray, &intersection, &normal);
                    return BounceResult::Continue;
                }
                BounceResult::Error
            }
            SOP::Refract => {
                if let Ok((intersection, normal, vop_above, vop_below)) =
                    self.get_interaction_parameters(ray)
                {
                    refract(ray, &intersection, &normal, vop_above, vop_below);
                    return BounceResult::Continue;
                }
                BounceResult::Error
            }
            SOP::Light(r, g, b) => BounceResult::Count(*r, *g, *b),
            SOP::Dark => BounceResult::Kill,
        }
    }

    /// Analyze a ray incoming on a surface and determine the normal on the side of the incoming ray.
    /// If no errors are found, return intersection point, that normal and the above & below VOPs.
    /// Otherwise return an error.
    fn get_interaction_parameters(
        &self,
        ray: &Ray,
    ) -> Result<(Point3D, Vector3D, &VOP, &VOP), Box<dyn Error>> {
        let intersection = self
            .geometry()
            .intersection(ray)
            .ok_or("No intersection between ray and shape.")?;

        let normal = self
            .geometry()
            .normal_at(&intersection)
            .ok_or("Shape has no normal at point.")?;

        let vop_above = self.vop_above_at(&intersection);
        let vop_below = self.vop_below_at(&intersection);

        // ray is inbound from medium into which normal points
        if normal.dot(&ray.direction) <= 0.0 {
            // check that ray VOP and above VOP match
            if ray.vop != *vop_above {
                panic!(
                    "VOP mismatch:\nray: {:#?}\nfrom: {:?}\ninto: {:?}\nintersection: {:?}\nnormal: {:?}",
                    ray,
                    vop_below,
                    vop_above,
                    intersection,
                    normal,
                )
            }
            Ok((intersection, normal, vop_above, vop_below))
        // ray is inbound from other side of boundary
        } else {
            if ray.vop != *vop_below {
                panic!(
                    "VOP mismatch:\nray: {:#?}\nfrom: {:?}\ninto: {:?}\nintersection: {:?}\nnormal: {:?}",
                    ray,
                    vop_below,
                    vop_above,
                    intersection,
                    normal,
                )
            }
            Ok((intersection, -1.0 * normal, vop_below, vop_above))
        }
    }
}

/// Reflect a ray in a surface.
fn reflect(ray: &mut Ray, intersection: &Point3D, normal: &Vector3D) {
    ray.origin = *intersection;
    ray.direction += 2.0 * ray.direction.dot(&normal).abs() / normal.length_squared() * *normal;
}

/// Refract a ray in a surface.
/// Reference: https://graphics.stanford.edu/courses/cs148-10-summer/docs/2006--degreve--reflection_refraction.pdf
fn refract(
    ray: &mut Ray,
    intersection: &Point3D,
    normal: &Vector3D,
    vop_above: &VOP,
    vop_below: &VOP,
) {
    // ratio of n_above / n_below
    let nanb = vop_above.index_of_refraction / vop_below.index_of_refraction;

    // normal to surface at new contact point
    let normal = normal.normalized();
    ray.direction = ray.direction.normalized();
    let cos_theta_i = normal.dot(&ray.direction.reversed());
    let sin_sq_theta_t = nanb.powi(2) * (1.0 - cos_theta_i.powi(2));

    // critical angle
    if sin_sq_theta_t >= 1.0 {
        return reflect(ray, intersection, &normal);
    }

    // update ray origin to point of intersection
    ray.origin = *intersection;
    // update ray direction
    ray.direction =
        ray.direction * nanb + normal * (nanb * cos_theta_i - (1.0 - sin_sq_theta_t).sqrt());

    // update ray VOP
    ray.vop = *vop_below;
}
