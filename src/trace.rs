use crate::{Ray, Surface, SOP};
use nalgebra::Point3;
use rayon::prelude::*;
use std::sync::Arc;

type Surf = Arc<dyn Surface + Send + Sync>;
type Interaction = Option<(Point3<f64>, f64)>;
type IndexedInteraction = Option<(Point3<f64>, f64, usize)>;

/// Return the point of intersection and distance squared to it (if any exist)
/// between a surface and a vector of rays.
fn one_surface_many_rays(surface: &Surf, rays: &[Ray]) -> Vec<Interaction> {
    rays.par_iter()
        .map(|r| {
            surface
                .intersection(r) // TODO: intersect method for multiple rays (on GPU)
                .map(|p| (p, (p - r.origin).norm_squared()))
        })
        .collect()
}

/// Calculate interactions between all surfaces and all rays and determine the closest point of
/// interaction for each ray, the squared distance to it and the surface index it corresponds to.
fn many_surfaces_many_rays(surfaces: &[Surf], rays: &[Ray]) -> Vec<IndexedInteraction> {
    // all surfaces x rays interactions
    let interactions: Vec<Vec<Interaction>> = surfaces
        .iter()
        .map(|s| one_surface_many_rays(s, rays))
        .collect();

    // contains the nearest interaction and the index of the surface it involves, if any
    let mut positional_interactions: Vec<IndexedInteraction> = vec![None; rays.len()];

    let mut closest_distance_squared: &f64;
    // TODO: multithread this
    for ri in 0..rays.len() {
        closest_distance_squared = &f64::MAX;
        for si in 0..surfaces.len() {
            // TODO: turn to unchecked
            if let Some(x) = &interactions[si][ri] {
                if x.1 < *closest_distance_squared {
                    positional_interactions[ri] = Some((x.0, x.1, si));
                    closest_distance_squared = &positional_interactions[ri].as_ref().unwrap().1;
                }
            }
        }
    }
    positional_interactions
}

/// Processes a ray's journey through the optical system and
/// returns whether that journey has been completed or not.
fn one_ray_interaction(ray: &mut Ray, point: Point3<f64>, dsq: f64, surface: &Surf) -> bool {
    // add absorption
    let distance: f64 = dsq.sqrt();
    for i in 0..=2 {
        ray.abs[i] += ray.vop.abs[i] * distance;
    }

    let sop = surface.unchecked_sop_at(&point);
    match sop {
        SOP::Reflect => {
            let (normal, _, _) = ray.get_interaction_parameters_unchecked(surface.as_ref(), &point);
            ray.reflect(&point, &normal);
            false
        }
        SOP::Refract => {
            let (normal, vop_above, vop_below) =
                ray.get_interaction_parameters_unchecked(surface.as_ref(), &point);
            ray.refract(&point, &normal, vop_above, vop_below);
            false
        }
        SOP::Light(r, g, b) => {
            let actual_rgb: Vec<usize> = ray
                .abs
                .iter()
                .zip([r, g, b].iter())
                .map(|(absorption, value)| (*value as f64 * (-absorption).exp()) as usize)
                .collect();
            ray.result = Some([actual_rgb[0], actual_rgb[1], actual_rgb[2]]);
            true
        }
        SOP::Dark => {
            ray.result = Some([0; 3]);
            true
        }
    }
}

/// Traces all rays through the system once, adding completed ones to the bucket of finished rays.
fn many_rays_interactions(
    rays: &mut Vec<Ray>,
    surfaces: &[Surf],
    indexed_interactions: Vec<IndexedInteraction>,
    completed_rays: &mut Vec<Ray>,
) {
    let mut ray_idx: usize = 0;
    while ray_idx != rays.len() {
        match indexed_interactions[ray_idx] {
            None => {
                // set the result to dark and move the ray to the completed stack
                rays[ray_idx].result = Some([0; 3]);
                completed_rays.push(rays.remove(ray_idx));
            }
            Some((p, dsq, si)) => {
                if one_ray_interaction(&mut rays[ray_idx], p, dsq, &surfaces[si]) {
                    completed_rays.push(rays.remove(ray_idx));
                } else {
                    ray_idx += 1;
                }
            }
        };
    }
}

/// Trace all rays through the system until no more are left.
fn trace(mut rays: Vec<Ray>, surfaces: &[Surf]) -> Vec<Ray> {
    let mut completed_rays: Vec<Ray> = Vec::new();
    let mut indexed_interactions: Vec<IndexedInteraction>;
    while !rays.is_empty() {
        indexed_interactions = many_surfaces_many_rays(surfaces, &rays);
        many_rays_interactions(
            &mut rays,
            surfaces,
            indexed_interactions,
            &mut completed_rays,
        )
    }
    completed_rays
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        camera::CameraBuilder,
        surface::SurfaceBuilder,
        surface::{plane::simple::PlaneBuilder, SphereBuilder},
        Camera, VOP,
    };
    use std::collections::HashMap;

    fn default_camera(vop_map: &HashMap<String, Arc<VOP>>) -> Camera {
        CameraBuilder {
            vop: "colored_air".to_owned(),
            ..Default::default()
        }
        .build(vop_map)
    }

    fn default_plane(vop_map: &HashMap<String, Arc<VOP>>) -> Surf {
        PlaneBuilder {
            origin: [5.0, 0.0, 0.0],
            normal: [-1.0, 0.0, 0.0],
            sop: SOP::Light(255, 255, 255),
            vop_below: "colored_air".to_owned(),
            vop_above: "colored_air".to_owned(),
        }
        .build(vop_map)
    }

    fn default_sphere(vop_map: &HashMap<String, Arc<VOP>>, sop: SOP) -> Surf {
        SphereBuilder {
            center: [3.0, 0.0, 0.0],
            radius: 0.5,
            sop,
            vop_below: "glass".to_owned(),
            vop_above: "colored_air".to_owned(),
        }
        .build(vop_map)
    }

    fn default_vop_map() -> HashMap<String, Arc<VOP>> {
        let mut vop_map = HashMap::new();
        vop_map.insert(
            "colored_air".to_owned(),
            Arc::new(VOP {
                abs: [0.01, 0.05, 0.1],
                ..Default::default()
            }),
        );
        vop_map.insert(
            "glass".to_owned(),
            Arc::new(VOP {
                ior: 1.5,
                ..Default::default()
            }),
        );
        vop_map
    }

    fn test_with_sop(sop: Option<SOP>) {
        let vop_map = default_vop_map();
        let rays = default_camera(&vop_map).create_rays();

        let mut scene: Vec<Surf> = Vec::new();
        scene.push(default_plane(&vop_map));
        // add a sphere if requested, to test out various SOPs
        if let Some(x) = sop {
            scene.push(default_sphere(&vop_map, x));
        }
        // check all rays have traced
        assert!(trace(rays, &scene).iter().all(|r| r.result.is_some()));
    }

    #[test]
    fn trace_colored_air() {
        test_with_sop(None)
    }

    #[test]
    fn trace_light() {
        test_with_sop(Some(SOP::Light(200, 200, 100)))
    }

    #[test]
    fn trace_reflection() {
        test_with_sop(Some(SOP::Reflect))
    }
    #[test]
    fn trace_refraction() {
        test_with_sop(Some(SOP::Refract))
    }

    #[test]
    fn trace_dark() {
        test_with_sop(Some(SOP::Dark))
    }
}
