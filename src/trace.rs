use crate::{camera::save_jpg, Camera, Ray, Surface, SOP};
use indicatif::ProgressBar;
use nalgebra::Point3;
use prettytable::{cell, row, Table};
use rayon::iter::Either;
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
    // TODO: flatten this and then pick every other element
    let interactions: Vec<Vec<Interaction>> = surfaces
        .par_iter()
        .map(|s| one_surface_many_rays(s, rays))
        .collect();

    // contains the nearest interaction and the index of the surface it involves, if any
    let mut positional_interactions: Vec<IndexedInteraction> = vec![None; rays.len()];

    let mut closest_distance_squared: &f64;
    // TODO: multithread this
    for (ri, indexed_interaction) in positional_interactions.iter_mut().enumerate() {
        closest_distance_squared = &f64::MAX;
        for (si, interaction) in interactions.iter().enumerate() {
            if let Some(x) = interaction[ri] {
                if x.1 < *closest_distance_squared {
                    *indexed_interaction = Some((x.0, x.1, si));
                    closest_distance_squared = &indexed_interaction.as_ref().unwrap().1;
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

fn many_rays_interactions(
    rays: Vec<Ray>,
    surfaces: &[Surf],
    indexed_interactions: Vec<IndexedInteraction>,
    completed_rays: &mut Vec<Ray>,
) -> Vec<Ray> {
    // split into no interactions and interactions
    let (mut no_interactions, new_rays): (Vec<Ray>, Vec<Ray>) = rays
        .into_par_iter()
        .zip(indexed_interactions)
        .partition_map(|(mut ray, idxi)| {
            // if the ray does intersect a surface
            if let Some((point, dsq, i)) = idxi {
                // if the next intersection ends the ray's journey
                if one_ray_interaction(&mut ray, point, dsq, &surfaces[i]) {
                    Either::Left(ray)
                } else {
                    Either::Right(ray)
                }
            // kill rays with no further intersection
            } else {
                ray.result = Some([0; 3]);
                Either::Left(ray)
            }
        });

    // add no interactions to completed
    completed_rays.append(&mut no_interactions);
    new_rays
}

/// Trace all rays through the system until no more are left.
pub fn trace_rays(mut rays: Vec<Ray>, surfaces: &[Surf]) -> (Vec<Ray>, Duration, Duration) {
    let mut completed_rays: Vec<Ray> = Vec::new();
    let mut indexed_interactions: Vec<IndexedInteraction>;

    let mut t0: Instant;
    let mut intersections: Duration = Duration::new(0, 0);
    let mut interactions: Duration = Duration::new(0, 0);

    let bar = ProgressBar::new(rays.len() as u64);
    while !rays.is_empty() {
        let total = rays.len() + completed_rays.len();
        bar.set_length(total as u64);
        bar.set_position((total - rays.len()) as u64);

        // calculate all intersections with all surfaces
        t0 = Instant::now();
        indexed_interactions = many_surfaces_many_rays(surfaces, &rays);
        intersections += t0.elapsed();

        // update rays according to their interactions
        t0 = Instant::now();
        rays = many_rays_interactions(rays, surfaces, indexed_interactions, &mut completed_rays);
        interactions += t0.elapsed();
    }
    (completed_rays, intersections, interactions)
}

pub fn render_scene(camera: &Camera, scene: &[Surf], filepath: &str) {
    let mut t0 = Instant::now();

    let rays: Vec<Ray> = camera.create_rays();
    let ray_generation_time_s = t0.elapsed().as_nanos() as f64 / 1e9;
    t0 = Instant::now();

    let (traced_rays, intersections_time, interactions_time) = trace_rays(rays, scene);
    let ray_trace_time_s = t0.elapsed().as_nanos() as f64 / 1e9;
    let intersections_time = intersections_time.as_nanos() as f64 / 1e9;
    let interactions_time = interactions_time.as_nanos() as f64 / 1e9;
    t0 = Instant::now();

    let ray_data: Vec<[u8; 3]> = process_ray_data(traced_rays, camera.num_x * camera.num_y);
    let ray_process_time_s = t0.elapsed().as_nanos() as f64 / 1e9;
    t0 = Instant::now();

    save_jpg(filepath, ray_data, camera.num_x, camera.num_y);
    let file_save_time_s = t0.elapsed().as_nanos() as f64 / 1e9;

    // show breakdown of time
    let total_time =
        ray_generation_time_s + ray_trace_time_s + ray_process_time_s + file_save_time_s;
    let mut table = Table::new();
    table.add_row(row![Fgb => "OPERATION", "TIME (s)", "TIME (%)"]);
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
    table.add_row(row![Fy =>
        ">> Ray Intersections",
        &format!("{:.2}", intersections_time),
        &format!("{:.2}", (intersections_time / total_time * 100.0))
    ]);
    table.add_row(row![Fy =>
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

pub fn process_ray_data(rays: Vec<Ray>, num_pixels: usize) -> Vec<[u8; 3]> {
    println!("Merging...");
    // pixel number as index => (number of components, sum of components)
    let mut pixel_values = vec![(0, [0; 3]); num_pixels];

    // add to each
    rays.into_iter().for_each(|ray| {
        let x = &mut pixel_values[ray.pixel_idx];
        x.0 += 1;
        for i in 0..=2 {
            x.1[i] += ray.result.expect("Ray did not have a result.")[i];
        }
    });

    // QUESTION: is it worth parallelizing this? There doesn't seem to be much performance gain.
    pixel_values
        .into_iter()
        .map(|(n, [r, g, b])| [(r / n) as u8, (g / n) as u8, (b / n) as u8])
        .collect()
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
        process_ray_data(rays, 25);
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
