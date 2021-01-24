use {
    rayon::ThreadPoolBuilder,
    raytracer::{
        camera::CameraBuilder,
        surface::{
            CheckerboardBuilder, PlaneBuilder, RectangleBuilder, SphereBuilder, SurfaceBuilder,
            ZParaboloidBuilder,
        },
        Camera, Surface, VOP,
    },
    serde_yaml::{from_str, from_value, Mapping, Value},
    std::{collections::HashMap, env, fs, sync::Arc},
};

/// Load the given configuration file and return its contents as a parsed yaml hash.
fn load_from_yaml() -> Mapping {
    let args: Vec<String> = env::args().collect();

    let contents =
        fs::read_to_string(args[1].clone()).expect("Something went wrong reading the file");

    from_str(&contents).expect("Error in parsing the file")
}

/// Extract the path to the image to be saved.
fn extract_filepath(lhm: &Mapping) -> String {
    lhm.get(&Value::String("filepath".to_owned()))
        .expect("No filepath given.")
        .as_str()
        .expect("Filepath must be a string.")
        .to_owned()
}

/// Extract VOPs.
fn extract_vops(lhm: &Mapping) -> HashMap<String, Arc<VOP>> {
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
fn extract_camera(lhm: &Mapping, vop_map: &HashMap<String, Arc<VOP>>) -> Camera {
    let camera_builder: CameraBuilder = from_value(
        lhm.get(&Value::String("camera".to_owned()))
            .expect("No camera given.")
            .to_owned(),
    )
    .expect("Could not parse camera.");
    camera_builder.build(vop_map)
}

fn extract_threads(lhm: &Mapping) -> Option<usize> {
    match lhm.get(&Value::String("threads".to_owned())) {
        Some(v) => Some(v.as_u64().expect("Number of threads must be an integer.") as usize),
        None => None,
    }
}

fn extract_surfaces(
    lhm: &Mapping,
    vop_map: &HashMap<String, Arc<VOP>>,
) -> Vec<Arc<dyn Surface + Send + Sync>> {
    let surfaces = lhm
        .get(&Value::String("surfaces".to_owned()))
        .expect("No surfaces give")
        .as_sequence()
        .expect("Surfaces must be a list.")
        .to_owned();

    let mut surface_list = Vec::new();

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
            "plane" => from_value::<PlaneBuilder>(s.to_owned())
                .expect("Error parsing plane.")
                .build(vop_map),
            "sphere" => from_value::<SphereBuilder>(s.to_owned())
                .expect("Error parsing sphere.")
                .build(vop_map),
            "zparaboloid" => from_value::<ZParaboloidBuilder>(s.to_owned())
                .expect("Error parsing zparaboloid.")
                .build(vop_map),
            _ => panic!("Unknown surface type"),
        };
        surface_list.push(surface);
    }

    surface_list
}

fn main() {
    let document = load_from_yaml();

    // if a number of threads has been given, set it
    if let Some(x) = extract_threads(&document) {
        ThreadPoolBuilder::new()
            .num_threads(x)
            .build_global()
            .unwrap();
        println!("Using {} thread(s).", x);
    } else {
        println!("Using {} thread(s)", num_cpus::get());
    }

    let filepath = extract_filepath(&document);
    let volumes = extract_vops(&document);
    let camera = extract_camera(&document, &volumes);
    let surfaces = extract_surfaces(&document, &volumes);
    println!(
        "Loaded: {} volume(s), {} surface(s).",
        volumes.len(),
        surfaces.len()
    );

    let result = camera.look(&surfaces);
    camera.save_jpg(&filepath, result);
    println!("Result saved: {}", filepath);
}
