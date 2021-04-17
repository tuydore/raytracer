use std::collections::HashMap;

pub mod camera;
pub mod colormap;
pub mod ray;
pub mod surface;
pub mod trace;
pub mod vop;

pub use {
    camera::Camera,
    nalgebra::Point3,
    ray::{BounceResult, Ray},
    std::sync::Arc,
    surface::{Surface, SOP},
    vop::VOP,
};

pub type Surf = Arc<dyn Surface + Send + Sync>;
pub type Interaction = Option<(Point3<f64>, f64)>;
pub type IndexedInteraction = Option<(Point3<f64>, f64, usize)>;
pub type VolumeMap = HashMap<String, Arc<VOP>>;

pub const TOLERANCE: f64 = 1e-5;
