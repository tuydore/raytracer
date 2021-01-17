use {
    crate::{
        geometry::{shape::Shape, Point3D, Vector3D, VOP},
        ray::{BounceResult, Ray},
    },
    std::error::Error,
};

#[derive(Debug, Clone, Copy)]
pub enum SOP {
    Reflect,
    Refract,
    Light(u8, u8, u8),
    Dark,
}

impl SOP {
    /// Deals with an incoming ray on a surface depending on the SOP type.
    /// Will update the ray information in-place.
    pub fn bounce(&self, ray: &mut Ray, shape: &impl Shape) -> BounceResult {
        match self {
            Self::Reflect => reflect(ray, shape),
            Self::Refract => refract(ray, shape),
            Self::Light(r, g, b) => BounceResult::Count(*r, *g, *b),
            Self::Dark => BounceResult::Kill,
        }
    }
}

/// Analyze a ray incoming on a surface and determine the normal on the side of the incoming ray.
/// If no errors are found, return intersection point, that normal and the above & below VOPs.
/// Otherwise return an error.
fn check_normal<'a>(
    ray: &Ray,
    shape: &'a impl Shape,
) -> Result<(Point3D, Vector3D, &'a VOP, &'a VOP), Box<dyn Error>> {
    let (intersection, _) = shape
        .intersection(ray)
        .ok_or("No intersection between ray and shape.")?;

    let normal = shape
        .normal_at(&intersection)
        .ok_or("Shape has no normal at point.")?;

    // ray is inbound from medium into which normal points
    if normal.dot(&ray.direction) <= 0.0 {
        // check that ray VOP and above VOP match
        if ray.vop != *shape.vop_above() {
            panic!(
                "VOP mismatch:\nray: {:#?}\nfrom: {:?}\ninto: {:?}\nintersection: {:?}\nnormal: {:?}",
                ray,
                shape.vop_below(),
                shape.vop_above(),
                intersection,
                normal,
            )
        }
        return Ok((intersection, normal, shape.vop_above(), shape.vop_below()));
    // ray is inbound from other side of boundary
    } else {
        if ray.vop != *shape.vop_below() {
            panic!(
                "VOP mismatch:\nray: {:#?}\nfrom: {:?}\ninto: {:?}\nintersection: {:?}\nnormal: {:?}",
                ray,
                shape.vop_below(),
                shape.vop_above(),
                intersection,
                normal,
            )
        }
        return Ok((
            intersection,
            -1.0 * normal,
            shape.vop_below(),
            shape.vop_above(),
        ));
    }
}

/// Reflect a ray in a surface.
fn reflect(ray: &mut Ray, shape: &impl Shape) -> BounceResult {
    if let Ok((intersection, normal, _, _)) = check_normal(ray, shape) {
        ray.origin = intersection;
        ray.direction += 2.0 * ray.direction.dot(&normal).abs() / normal.length_squared() * normal;
        BounceResult::Continue
    } else {
        BounceResult::Error
    }
}

/// Refract a ray in a surface.
/// Reference: https://graphics.stanford.edu/courses/cs148-10-summer/docs/2006--degreve--reflection_refraction.pdf
fn refract(ray: &mut Ray, shape: &impl Shape) -> BounceResult {
    if let Ok((intersection, mut normal, vop_above, vop_below)) = check_normal(ray, shape) {
        // ratio of n_above / n_below
        let nanb = vop_above.index_of_refraction / vop_below.index_of_refraction;

        // normal to surface at new contact point
        normal = normal.normalized();
        ray.direction = ray.direction.normalized();
        let cos_theta_i = normal.dot(&ray.direction.reversed());
        let sin_sq_theta_t = nanb.powi(2) * (1.0 - cos_theta_i.powi(2));

        // critical angle
        if sin_sq_theta_t >= 1.0 {
            return reflect(ray, shape);
        }

        // update ray origin to point of intersection
        ray.origin = intersection;
        // update ray direction
        ray.direction =
            ray.direction * nanb + normal * (nanb * cos_theta_i - (1.0 - sin_sq_theta_t).sqrt());

        // update ray VOP
        ray.vop = *vop_below;
        BounceResult::Continue
    } else {
        BounceResult::Error
    }
}
