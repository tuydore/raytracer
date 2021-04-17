use raytracer::trace::render_scene;
use raytracer::{Surf, VolumeMap};
use std::path::PathBuf;
use structopt::StructOpt;
use {
    rayon::ThreadPoolBuilder,
    raytracer::{
        camera::{combine_rays, save_jpg, trace_rays, Camera, CameraBuilder},
        surface::{
            CheckerboardBuilder, CylinderBuilder, MandelbrotPlaneBuilder, ParaboloidBuilder,
            PlaneBuilder, RectangleBuilder, SphereBuilder, SurfaceBuilder,
            TexturedRectangleBuilder,
        },
        Ray, Surface,
    },
    serde_yaml::{from_str, from_value, Mapping, Value},
    std::{fs, sync::Arc},
};

/// Extract VOPs.
fn extract_vops(lhm: &Mapping) -> VolumeMap {
    let volumes = lhm
        .get(&Value::String("volumes".to_owned()))
        .expect("No volumes given.")
        .as_mapping()
        .expect("Volumes must be given as dictionary.");

    volumes
        .into_iter()
        .map(|(k, v)| {
            (
                k.as_str()
                    .expect("Volume names must be strings.")
                    .to_owned(),
                Arc::new(from_value(v.clone()).expect("Could not parse volume")),
            )
        })
        .collect()
}

/// Get the camera configuration.
fn extract_camera(lhm: &Mapping, vop_map: &VolumeMap) -> Camera {
    let camera_builder: CameraBuilder = from_value(
        lhm.get(&Value::String("camera".to_owned()))
            .expect("No camera given.")
            .to_owned(),
    )
    .expect("Could not parse camera.");
    camera_builder.build(vop_map)
}

fn extract_surfaces(lhm: &Mapping, vop_map: &VolumeMap) -> Vec<Surf> {
    let surfaces = lhm
        .get(&Value::String("surfaces".to_owned()))
        .expect("No surfaces give")
        .as_sequence()
        .expect("Surfaces must be a list.")
        .to_owned();

    let mut surface_list = Vec::with_capacity(surfaces.len());

    for s in surfaces.iter() {
        let surface = match s
            .as_mapping()
            .expect("Each surfacemust be given as a mapping.")
            .get(&Value::String("type".to_owned()))
            .expect("Surface must have a type.")
            .as_str()
            .expect("Surface type must be a string.")
        {
            "checkerboard" => from_value::<CheckerboardBuilder>(s.to_owned())
                .expect("Error parsing checkerboard.")
                .build(vop_map),
            "rectangle" => from_value::<RectangleBuilder>(s.to_owned())
                .expect("Error parsing rectangle.")
                .build(vop_map),
            "texturedrectangle" => from_value::<TexturedRectangleBuilder>(s.to_owned())
                .expect("Error parsing textured rectangle.")
                .build(vop_map),
            "plane" => from_value::<PlaneBuilder>(s.to_owned())
                .expect("Error parsing plane.")
                .build(vop_map),
            "mandelbrotplane" => from_value::<MandelbrotPlaneBuilder>(s.to_owned())
                .expect("Error parsing Mandelbrot plane.")
                .build(vop_map),
            "sphere" => from_value::<SphereBuilder>(s.to_owned())
                .expect("Error parsing sphere.")
                .build(vop_map),
            "paraboloid" => from_value::<ParaboloidBuilder>(s.to_owned())
                .expect("Error parsing paraboloid.")
                .build(vop_map),
            "cylinder" => from_value::<CylinderBuilder>(s.to_owned())
                .expect("Error parsing cylinder.")
                .build(vop_map),
            _ => panic!("Unknown surface type"),
        };
        surface_list.push(surface);
    }

    surface_list
}
#[derive(Debug, StructOpt)]
#[structopt(name = "raytracer", about = "Powered by Lockdown Boredom™")]
struct Opt {
    /// Input YAML file with scene configuration.
    #[structopt(parse(from_os_str), name = "INPUT")]
    input: PathBuf,

    /// Path to file containing output render. Must be of type jpg/jpeg/tif/tiff.
    #[structopt(short, long, parse(from_os_str))]
    output: PathBuf,

    /// Number of threads to use.
    #[structopt(short, long)]
    threads: Option<usize>,

    /// Use legacy ray tracing algorithm.
    #[structopt(short, long)]
    legacy: bool,
}

impl Opt {
    // TODO: remove excessive Arcs?
    /// Load the scene given in the YAML input file.
    fn load_scene_from_yaml(&self) -> (VolumeMap, Camera, Vec<Surf>) {
        let contents =
            fs::read_to_string(self.input.clone()).expect("Something went wrong reading the file");
        let document: Mapping = from_str(&contents).expect("Error in parsing the file");
        let volumes = extract_vops(&document);
        let camera = extract_camera(&document, &volumes);
        let surfaces = extract_surfaces(&document, &volumes);
        println!(
            "Loaded: {} volume(s), {} surface(s).",
            volumes.len(),
            surfaces.len()
        );
        (volumes, camera, surfaces)
    }

    /// Start the global thread pool using given number of threads or the number of CPUs.
    fn initialize_global_thread_pool(&self) {
        if let Some(x) = self.threads {
            ThreadPoolBuilder::new()
                .num_threads(x)
                .build_global()
                .unwrap();
            println!("Using {} thread(s).", x);
        } else {
            println!("Using {} thread(s)", num_cpus::get());
        }
    }

    /// Check if the output file is valid. If it is, ensure all directory structure to it exists
    /// and return the path as a String.
    fn create_output_dir(&self) -> String {
        match self.output.extension() {
            Some(s) => match s.to_str().expect("Extension is not valid unicode.") {
                "jpg" | "jpeg" | "tif" | "tiff" => {
                    fs::create_dir_all(
                        self.output
                            .parent()
                            .expect("No parent directory for save file."),
                    )
                    .expect("Could not create parent directory to output file.");
                    self.output.to_str().unwrap().to_owned()
                }
                _ => panic!(
                    "Invalid file extension {:?}, only jpg/jpeg/tif/tiff output is supported.",
                    s
                ),
            },
            None => panic!("Invalid file name for output: {:?}", self.output),
        }
    }
}

// TODO: remove this once new method is fully implemented
fn raytrace_old(camera: &Camera, scene: &[Arc<dyn Surface + Send + Sync>], filepath: &str) {
    let rays: Vec<Ray> = camera.create_rays();
    save_jpg(
        filepath,
        combine_rays(trace_rays(rays, scene), camera.antialiasing),
        camera.num_x,
        camera.num_y,
    );
}

fn main() {
    let opt = Opt::from_args();
    opt.initialize_global_thread_pool();
    let savefile = opt.create_output_dir();
    let (_, camera, surfaces): (VolumeMap, Camera, Vec<Surf>) = opt.load_scene_from_yaml();
    if opt.legacy {
        println!("Using legacy ray tracing algorithm...");
        raytrace_old(&camera, &surfaces, &savefile);
    } else {
        render_scene(&camera, &surfaces, &savefile);
    }
    println!("Result saved: {}", savefile);
}
