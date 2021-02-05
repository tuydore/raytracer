use {
    crate::{Surface, SOP, VOP},
    nalgebra::{Point3, Unit, Vector3},
    std::{error::Error, sync::Arc},
};

#[derive(Debug, Clone)]
pub struct Ray {
    pub origin: Point3<f64>,
    pub direction: Vector3<f64>,
    pub vop: Arc<VOP>,
    pub abs: [f64; 3], // TODO: ray absorption when ray has no more intersections?
}

impl Ray {
    // TODO: avoid repetition by calculating first_intersection twice
    /// Launch a ray through the system and fetch its final return value.
    pub fn launch(&mut self, surfaces: &[Arc<dyn Surface + Send + Sync>]) -> BounceResult {
        loop {
            // get all first intersections with surfaces and distances to them
            let intersections: Vec<Option<(Point3<f64>, f64)>> = surfaces
                .iter()
                .map(|s| s.intersection(self))
                .map(|p| {
                    if let Some(point) = p {
                        Some((point, (point - self.origin).norm_squared()))
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
            match self.bounce_unchecked(surfaces[index].as_ref(), &intersections[index].unwrap().0)
            {
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
        surface: &dyn Surface,
        point: &Point3<f64>,
    ) -> Result<(Unit<Vector3<f64>>, Arc<VOP>, Arc<VOP>), Box<dyn Error>> {
        // get VOPs above and below
        let vop_above = surface.unchecked_vop_above_at(&point);
        let vop_below = surface.unchecked_vop_below_at(&point);

        // get normal at point
        let normal = surface.unchecked_normal_at(&point);

        // ray is inbound from medium into which normal points
        if normal.dot(&self.direction) <= 0.0 {
            // check that ray VOP and above VOP match
            if self.vop != vop_above {
                panic!(
                    "VOP mismatch:\nray: {:#?}\nfrom: {:?}\ninto: {:?}\nintersection: {:?}\nnormal: {:?}",
                    self,
                    vop_above,
                    vop_below,
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
            Ok((
                Unit::new_normalize(-1.0 * normal.into_inner()),
                vop_below,
                vop_above,
            ))
        }
    }

    pub fn bounce_unchecked(&mut self, surface: &dyn Surface, point: &Point3<f64>) -> BounceResult {
        // update ray's own absorption factor by the distance traveled in the current VOP
        let distance = (self.origin - *point).norm();
        for i in 0..=2 {
            self.abs[i] += self.vop.abs[i] * distance;
        }

        let sop = surface.unchecked_sop_at(point);
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
                    self.refract(point, &normal, vop_above, vop_below);
                    return BounceResult::Continue;
                }
                BounceResult::Error
            }
            SOP::Light(r, g, b) => {
                let actual_rgb: Vec<u8> = self
                    .abs
                    .iter()
                    .zip([r, g, b].iter())
                    .map(|(absorption, value)| (*value as f64 * (-absorption).exp()) as u8)
                    .collect();
                BounceResult::Count(actual_rgb[0], actual_rgb[1], actual_rgb[2])
            }
            SOP::Dark => BounceResult::Kill,
        }
    }

    /// Reflect a ray in a surface.
    fn reflect(&mut self, intersection: &Point3<f64>, normal: &Vector3<f64>) {
        self.origin = *intersection;
        self.direction += 2.0 * self.direction.dot(&normal).abs() / normal.norm_squared() * *normal;
    }

    /// Refract a ray in a surface.
    /// Reference: https://graphics.stanford.edu/courses/cs148-10-summer/docs/2006--degreve--reflection_refraction.pdf
    fn refract(
        &mut self,
        intersection: &Point3<f64>,
        normal: &Vector3<f64>,
        vop_above: Arc<VOP>,
        vop_below: Arc<VOP>,
    ) {
        // ratio of n_above / n_below
        let nanb = vop_above.ior / vop_below.ior;

        // normal to surface at new contact point
        let normal = normal.normalize();
        self.direction = self.direction.normalize();
        let cos_theta_i = -normal.dot(&self.direction);
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

/// Result returned by ray bounce operation. This can be one of the following:
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

#[cfg(test)]
mod tests {
    use {
        super::*,
        crate::{
            surface::plane::{simple::Plane, PlaneShape},
            TOLERANCE,
        },
    };

    fn air() -> Arc<VOP> {
        Arc::new(VOP {
            ior: 1.0,
            abs: [0.0, 0.0, 0.0],
        })
    }

    fn glass() -> Arc<VOP> {
        Arc::new(VOP {
            ior: 1.5,
            abs: [0.0, 0.0, 0.0],
        })
    }

    fn reflective_plane(air: Arc<VOP>) -> Plane {
        Plane {
            geometry: PlaneShape::new(Point3::origin(), Vector3::z(), None),
            sop: SOP::Reflect,
            vop_above: air.clone(),
            vop_below: air,
        }
    }

    #[cfg(test)]
    mod direction {
        //! Test that the direction of a ray is correctly updated during reflection and refraction
        //! operations. Because this is solely dependent on the correct implementation of the
        //! `Surface` trait for each object, it only makes sense to test for a single one and test
        //! `Surface` implementation separately, per surface type.
        use super::*;

        fn refractive_plane(air: Arc<VOP>, glass: Arc<VOP>) -> Plane {
            Plane {
                geometry: PlaneShape::new(Point3::origin(), Vector3::z(), None),
                sop: SOP::Refract,
                vop_above: air,
                vop_below: glass,
            }
        }

        #[test]
        fn refraction_orthogonal() {
            let air = air();
            let glass = glass();
            let plane = refractive_plane(air.clone(), glass);
            let mut downward_ray = Ray {
                origin: Point3::origin(),
                direction: -Vector3::z(),
                vop: air,
                abs: [0.0; 3],
            };
            downward_ray.bounce_unchecked(&plane, &Point3::origin());
            assert_eq!(downward_ray.direction.normalize(), -Vector3::z());
            assert_eq!(downward_ray.origin, Point3::origin());
        }

        #[test]
        fn snells_law_from_above() {
            let air = air();
            let glass = glass();
            let plane = refractive_plane(air.clone(), glass);
            let original_ray = Ray {
                origin: Point3::new(1.0, 0.0, 1.0),
                direction: Vector3::new(-1.0, 0.0, -1.0),
                vop: air,
                abs: [0.0; 3],
            };
            let mut ray = original_ray.clone();
            let intersection = plane.intersection(&original_ray).unwrap();
            ray.bounce_unchecked(&plane, &Point3::origin());

            // calculate via snell's law
            let normal = plane.unchecked_normal_at(&ray.origin);
            let theta_i = normal
                .dot(&(-1.0 * original_ray.direction).normalize())
                .acos();
            let theta_t = (-1.0 * normal.into_inner())
                .dot(&ray.direction.normalize())
                .acos();
            assert!(
                plane.unchecked_vop_above_at(&intersection).ior * theta_i.sin()
                    - plane.unchecked_vop_below_at(&intersection).ior * theta_t.sin()
                    <= f64::EPSILON
            );
        }

        #[test]
        fn snells_law_from_below() {
            let air = air();
            let glass = glass();
            let plane = refractive_plane(air, glass.clone());
            let original_ray = Ray {
                origin: Point3::new(0.2, 0.0, -1.0),
                direction: Vector3::new(-0.2, 0.0, 1.0),
                vop: glass,
                abs: [0.0; 3],
            };
            let intersection = plane.intersection(&original_ray).unwrap();
            let mut ray = original_ray.clone();
            ray.bounce_unchecked(&plane, &Point3::origin());

            // calculate via snell's law
            let normal = plane.unchecked_normal_at(&ray.origin);
            let theta_i = normal.dot(&(original_ray.direction).normalize()).acos();
            let theta_t = normal.dot(&ray.direction.normalize());
            assert!(
                plane.unchecked_vop_below_at(&intersection).ior * theta_i.sin()
                    - plane.unchecked_vop_above_at(&intersection).ior * theta_t.sin()
                    <= f64::EPSILON
            );
        }

        #[test]
        fn reflection_air() {
            let air = air();
            let sphere = reflective_plane(air.clone());
            let mut ray = Ray {
                origin: Point3::new(1.0, 0.0, 1.0),
                direction: Vector3::new(-1.0, 0.0, -1.0),
                vop: air,
                abs: [0.0; 3],
            };
            ray.bounce_unchecked(&sphere, &Point3::origin());
            assert_eq!(ray.origin, Point3::origin());
            assert!(
                (ray.direction.normalize() - Vector3::new(-1.0, 0.0, 1.0).normalize())
                    .norm_squared()
                    <= TOLERANCE
            );
        }

        // TODO: test TIR
    }

    #[cfg(test)]
    mod lifetime {
        //! Test that rays are appropriately killed when reaching Dark areas or when no further
        //! intersections are discovered.
        use super::*;

        fn downwards_ray(vop: Arc<VOP>) -> Ray {
            Ray {
                origin: Point3::new(0.0, 0.0, 10.0),
                direction: Vector3::new(0.0, 0.0, -1.0),
                vop,
                abs: [0.0; 3],
            }
        }

        fn light_plane(vop: Arc<VOP>) -> Plane {
            Plane {
                geometry: PlaneShape::new(Point3::origin(), Vector3::z(), None),
                sop: SOP::Light(255, 255, 255),
                vop_above: vop.clone(),
                vop_below: vop,
            }
        }

        fn dark_plane(vop: Arc<VOP>) -> Plane {
            Plane {
                geometry: PlaneShape::new(Point3::origin(), Vector3::z(), None),
                sop: SOP::Dark,
                vop_above: vop.clone(),
                vop_below: vop,
            }
        }

        #[test]
        fn counted_ray() {
            let air = air();
            let mut ray = downwards_ray(air.clone());
            let plane = light_plane(air);
            assert_eq!(
                ray.launch(&[Arc::new(plane)]),
                BounceResult::Count(255, 255, 255)
            );
        }

        #[test]
        fn ray_killed_at_dark_plane() {
            let air = air();
            let mut ray = downwards_ray(air.clone());
            let plane = dark_plane(air);
            assert_eq!(ray.launch(&[Arc::new(plane)]), BounceResult::Kill);
        }

        #[test]
        fn ray_killed_no_more_intersections() {
            let air = air();
            let mut ray = downwards_ray(air.clone());
            let plane = reflective_plane(air);
            assert_eq!(ray.launch(&[Arc::new(plane)]), BounceResult::Kill);
        }
    }
}
