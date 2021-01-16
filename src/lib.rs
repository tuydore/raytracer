pub mod camera;
pub mod geometry;
pub mod ray;

pub use {
    camera::Camera,
    geometry::{
        shape::{Plane, Rectangle, Sphere},
        sop::SOP,
        vop::VOP,
        Point3D, Vector3D,
    },
    ray::{BounceResult, Ray},
};

pub const SURFACE_INCLUSION: f64 = 1e-7;
pub const DOT_PRODUCT_IDENTITY: f64 = 1e-5;
