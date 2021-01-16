use crate::{geometry::shape::Shape, light::Ray};

pub enum SOP {
    Reflect,
    Refract,
}

impl SOP {
    /// Deals with an incoming ray on a surface depending on the SOP type.
    /// Will update the ray information in-place.
    pub fn bounce(&self, ray: &mut Ray, shape: &impl Shape) {
        match self {
            Self::Reflect => Self::reflect(ray, shape),
            Self::Refract => Self::refract(ray, shape),
        }
    }

    /// Reflect a ray in a surface.
    fn reflect(ray: &mut Ray, shape: &impl Shape) {
        ray.origin = shape.first_intersection(ray).unwrap();
        let normal = shape.normal_at(&ray.origin).unwrap();
        ray.direction += 2.0 * ray.direction.dot(&normal).abs() / normal.length_squared() * normal;
    }

    /// Refract a ray in a surface.
    fn refract(ray: &mut Ray, shape: &impl Shape) {
        // update ray origin to point of intersection
        ray.origin = shape.first_intersection(ray).unwrap();

        // ratio of n_above / n_below
        let nanb = shape.vop_above().index_of_refraction / shape.vop_below().index_of_refraction;

        // normal to surface at new contact point
        let n = shape.normal_at(&ray.origin).unwrap().normalized();
        ray.direction = ray.direction.normalized();

        // update ray direction
        ray.direction = nanb * ray.direction
            + (nanb * ray.direction.dot(&n)
                - (1.0 - nanb.powi(2) * (1.0 - ray.direction.dot(&n).powi(2))).sqrt())
                * n;

        // TODO: changing ray VOP
    }
}
