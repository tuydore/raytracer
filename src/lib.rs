pub mod camera;
pub mod ray;
pub mod shape;
pub mod surface;
pub mod vop;

pub use {
    camera::Camera,
    ray::{BounceResult, Ray},
    shape::Shape,
    surface::{Surface, SOP},
    vop::VOP,
};

pub const TOLERANCE: f64 = 1e-5;
