use image::{io::Reader, RgbImage};
use nalgebra::Vector3;

pub mod checkerboard;
pub mod mandelbrotplane;
pub mod plane;
pub mod rectangle;
pub mod sphere;
pub mod zparaboloid;

use {
    crate::{Shape, VOP},
    nalgebra::Point3,
    serde::Deserialize,
    std::collections::HashMap,
    std::sync::Arc,
};
pub use {
    checkerboard::CheckerboardBuilder, mandelbrotplane::MandelbrotPlaneBuilder,
    plane::PlaneBuilder, rectangle::RectangleBuilder, sphere::SphereBuilder,
    zparaboloid::ZParaboloidBuilder,
};

#[derive(Debug, Clone, Copy, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SOP {
    Reflect,
    Refract,
    Light(u8, u8, u8),
    Dark,
}

// TODO: SOP by loading image texture?

pub trait Surface {
    fn geometry(&self) -> &dyn Shape;

    fn vop_above_at(&self, point: &Point3<f64>) -> Arc<VOP>;

    fn vop_below_at(&self, point: &Point3<f64>) -> Arc<VOP>;

    fn sop_at(&self, point: &Point3<f64>) -> SOP;
}

pub trait SurfaceBuilder {
    fn build(self, vop_map: &HashMap<String, Arc<VOP>>) -> Arc<dyn Surface + Send + Sync>;
}

/// Load an image to be used as a texture.
fn load_texture_from_file(filepath: &str) -> RgbImage {
    Reader::open(filepath)
        .unwrap_or_else(|_| panic!("Could not load texture from {}", filepath))
        .decode()
        .unwrap_or_else(|_| panic!("Could not decode texture loaded from {}", filepath))
        .as_rgb8()
        .unwrap_or_else(|| panic!("Could not parse texture as RGB8 image: {}", filepath))
        .to_owned()
}

fn unchecked_rectangle_sop_from_texture(
    texture: RgbImage,
    normal: &Vector3<f64>,
    image_vertical: &Vector3<f64>,
    center: &Point3<f64>,
    point: &Point3<f64>,
    real_size: &[f64; 2],
) -> SOP {
    let right: Vector3<f64> = image_vertical.cross(normal);
    let upper_left_corner: Point3<f64> =
        center + real_size[0] / 2.0 * image_vertical - real_size[1] / 2.0 * right;

    // determine position of point in fractional coordinates, from center
    let offset = point - center;

    // sizes in real space
    let y = offset.dot(image_vertical);
    let x = offset.dot(&image_vertical.cross(normal));

    todo!()
}
