use crate::{camera::save_jpg, Camera, Ray, Surface, SOP};
use nalgebra::Point3;
use prettytable::{cell, row, Table};
use rayon::prelude::*;
use std::time::Instant;
use std::{sync::Arc, time::Duration};

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
    mut indexed_interactions: Vec<IndexedInteraction>,
    completed_rays: &mut Vec<Ray>,
) {
    let mut ray_idx: usize = 0;
    while ray_idx != rays.len() {
        match indexed_interactions[ray_idx] {
            None => {
                // set the result to dark and move the ray to the completed stack
                rays[ray_idx].result = Some([0; 3]);
                completed_rays.push(rays.remove(ray_idx));
                indexed_interactions.remove(ray_idx);
            }
            Some((p, dsq, si)) => {
                if one_ray_interaction(&mut rays[ray_idx], p, dsq, &surfaces[si]) {
                    completed_rays.push(rays.remove(ray_idx));
                    indexed_interactions.remove(ray_idx);
                } else {
                    ray_idx += 1;
                }
            }
        };
    }
}

/// Trace all rays through the system until no more are left.
pub fn trace_rays(mut rays: Vec<Ray>, surfaces: &[Surf]) -> (Vec<Ray>, Duration, Duration) {
    let mut completed_rays: Vec<Ray> = Vec::new();
    let mut indexed_interactions: Vec<IndexedInteraction>;

    let mut t0: Instant;
    let mut intersections: Duration = Duration::new(0, 0);
    let mut interactions: Duration = Duration::new(0, 0);

    while !rays.is_empty() {
        println!("Rays left in stack: {}", rays.len());
        t0 = Instant::now();
        indexed_interactions = many_surfaces_many_rays(surfaces, &rays);
        intersections += t0.elapsed();
        t0 = Instant::now();
        many_rays_interactions(
            &mut rays,
            surfaces,
            indexed_interactions,
            &mut completed_rays,
        );
        interactions += t0.elapsed();
    }
    (completed_rays, intersections, interactions)
}

pub fn raytrace(camera: &Camera, scene: &[Surf], filepath: &str) {
    let mut t0 = Instant::now();

    let rays: Vec<Ray> = camera.create_rays();
    let ray_generation_time_s = t0.elapsed().as_nanos() as f64 / 1e9;
    t0 = Instant::now();

    let (traced_rays, intersections_time, interactions_time) = trace_rays(rays, scene);
    let ray_trace_time_s = t0.elapsed().as_nanos() as f64 / 1e9;
    let intersections_time = intersections_time.as_nanos() as f64 / 1e9;
    let interactions_time = interactions_time.as_nanos() as f64 / 1e9;
    t0 = Instant::now();

    let ray_data: Vec<[u8; 3]> = read_ray_data(traced_rays);
    let ray_process_time_s = t0.elapsed().as_nanos() as f64 / 1e9;
    t0 = Instant::now();

    save_jpg(filepath, ray_data, camera.num_x, camera.num_y);
    let file_save_time_s = t0.elapsed().as_nanos() as f64 / 1e9;

    // show breakdown of time
    let total_time =
        ray_generation_time_s + ray_trace_time_s + ray_process_time_s + file_save_time_s;
    let mut table = Table::new();
    table.add_row(row!["OPERATION", "TIME (s)", "TIME (%)"]);
    table.add_row(row!["Total", &format!("{:.2}", total_time), "100.0"]);
    table.add_row(row![
        "Ray Generation",
        &format!("{:.2}", ray_generation_time_s),
        &format!("{:.2}", (ray_generation_time_s / total_time * 100.0))
    ]);
    table.add_row(row![
        "Ray Trace",
        &format!("{:.2}", ray_trace_time_s),
        &format!("{:.2}", (ray_trace_time_s / total_time * 100.0))
    ]);
    table.add_row(row![
        ">> Ray Intersections",
        &format!("{:.2}", intersections_time),
        &format!("{:.2}", (intersections_time / total_time * 100.0))
    ]);
    table.add_row(row![
        ">> Ray Interactions",
        &format!("{:.2}", interactions_time),
        &format!("{:.2}", (interactions_time / total_time * 100.0))
    ]);
    table.add_row(row![
        "Ray Processing",
        &format!("{:.2}", ray_process_time_s),
        &format!("{:.2}", (ray_process_time_s / total_time * 100.0))
    ]);
    table.add_row(row![
        "File Save",
        &format!("{:.2}", file_save_time_s),
        &format!("{:.2}", (file_save_time_s / total_time * 100.0))
    ]);
    table.printstd();
}

pub fn read_ray_data(mut rays: Vec<Ray>) -> Vec<[u8; 3]> {
    println!("Merging ray data...");
    rays.sort_unstable_by_key(|r| r.pixel_idx);
    let mut result: Vec<[u8; 3]> = Vec::new();
    let mut current_pixel_idx: usize = 0;
    let mut num_subpixels: usize = 0;
    let mut total_value: [usize; 3] = [0; 3];
    let mut total_value_u8: [u8; 3] = [0; 3];
    for ray in rays.iter() {
        if ray.pixel_idx != current_pixel_idx {
            for i in 0..=2 {
                total_value_u8[i] = (total_value[i] / num_subpixels) as u8;
            }
            result.push(total_value_u8);
            num_subpixels = 0;
            total_value = [0; 3];
            current_pixel_idx += 1;
        }
        for (rx, tvx) in ray
            .result
            .expect("Ray did not have a result.")
            .iter()
            .zip(total_value.iter_mut())
        {
            *tvx += rx;
        }
        num_subpixels += 1;
    }
    // TODO: clean up this duplicate
    for i in 0..=2 {
        total_value_u8[i] = (total_value[i] / num_subpixels) as u8;
    }
    result.push(total_value_u8);
    result
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
                abs: [0.01, 0.05, 0.0],
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

    fn test_with_sop(sop: Option<SOP>) -> Vec<Ray> {
        let vop_map = default_vop_map();
        let rays = default_camera(&vop_map).create_rays();

        let mut scene: Vec<Surf> = Vec::new();
        scene.push(default_plane(&vop_map));
        // add a sphere if requested, to test out various SOPs
        if let Some(x) = sop {
            scene.push(default_sphere(&vop_map, x));
        }
        // check all rays have traced
        let (traced, _, _) = trace_rays(rays, &scene);
        assert!(traced.iter().all(|r| r.result.is_some()));
        traced
    }

    #[test]
    fn trace_colored_air() {
        test_with_sop(None);
    }

    #[test]
    fn trace_light() {
        test_with_sop(Some(SOP::Light(200, 200, 100)));
    }

    #[test]
    fn trace_reflection() {
        test_with_sop(Some(SOP::Reflect));
    }
    #[test]
    fn trace_refraction() {
        test_with_sop(Some(SOP::Refract));
    }

    #[test]
    fn trace_dark() {
        test_with_sop(Some(SOP::Dark));
    }

    #[test]
    fn ray_data() {
        let rays = test_with_sop(None);
        read_ray_data(rays);
    }

    #[test]
    fn correct_intersection() {
        let vop_map = default_vop_map();
        let rays = default_camera(&vop_map).create_rays();
        let plane = PlaneBuilder {
            origin: [5.0, 0.0, 0.0],
            normal: [-1.0, 0.0, 0.0],
            sop: SOP::Light(255, 255, 255),
            vop_below: "colored_air".to_owned(),
            vop_above: "colored_air".to_owned(),
        }
        .build(&vop_map);

        let mut scene: Vec<Surf> = Vec::new();
        scene.push(plane);
        scene.push(default_sphere(&vop_map, SOP::Light(60, 60, 60)));
        // check all rays have traced
        let (traced, _, _) = trace_rays(rays, &scene);
        assert!(traced.iter().all(|r| r.result.is_some()));
        assert_eq!(traced[0].result.unwrap()[2], 60);
    }
}
