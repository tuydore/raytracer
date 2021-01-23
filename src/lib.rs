pub mod basics;
pub mod camera;
pub mod ray;
pub mod shape;
pub mod surface;
pub mod vop;

pub use {
    basics::{Point3D, Vector3D},
    camera::Camera,
    ray::{BounceResult, Ray},
    shape::Shape,
    surface::{
        CheckerboardBuilder, PlaneBuilder, RectangleBuilder, SphereBuilder, Surface,
        ZParaboloidBuilder, SOP,
    },
    vop::VOP,
};

pub const SURFACE_INCLUSION: f64 = 1e-5;
pub const VECTOR_IDENTITY: f64 = 1e-5;
