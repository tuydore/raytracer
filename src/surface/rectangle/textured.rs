use image::GenericImageView;

use crate::TOLERANCE;
// TODO: fix orientation
use {
    super::{
        super::{Shape, Surface, SurfaceBuilder},
        RectangleShape,
    },
    crate::{Ray, SOP, VOP},
    collections::HashMap,
    image::{io::Reader, DynamicImage, RgbImage},
    nalgebra::{Point3, Unit, Vector3},
    serde::Deserialize,
    std::collections,
    std::sync::Arc,
};

pub struct TexturedRectangle {
    pub geometry: RectangleShape,
    pub texture: RgbImage,
    pub size_scaling: [f64; 2],
    pub vop_above: Arc<VOP>,
    pub vop_below: Arc<VOP>,
}

#[derive(Deserialize)]
pub struct TexturedRectangleBuilder {
    pub origin: [f64; 3],
    pub normal: [f64; 3],
    pub texture: String,
    pub size: [f64; 2],
    pub orientation: [f64; 3],
    pub vop_below: String,
    pub vop_above: String,
}

impl Surface for TexturedRectangle {
    fn intersection(&self, ray: &Ray) -> Option<Point3<f64>> {
        self.geometry.intersection(ray)
    }
    fn unchecked_normal_at(&self, point: &Point3<f64>) -> Unit<Vector3<f64>> {
        self.geometry.unchecked_normal_at(point)
    }
    fn unchecked_vop_above_at(&self, _point: &Point3<f64>) -> Arc<VOP> {
        self.vop_above.clone()
    }
    fn unchecked_vop_below_at(&self, _point: &Point3<f64>) -> Arc<VOP> {
        self.vop_below.clone()
    }
    fn unchecked_sop_at(&self, point: &Point3<f64>) -> SOP {
        // orientation == vertical of image / texture, along 1st size dimension
        let right: Vector3<f64> = self.geometry.orientation.cross(&self.geometry.normal);
        let upper_left_corner: Point3<f64> = self.geometry.origin
            + self.geometry.size[0] / 2.0 * self.geometry.orientation.into_inner()
            - self.geometry.size[1] / 2.0 * right;

        // size of texture image

        // calculate ox and oy in "pixel" values
        let offset: Vector3<f64> = point - upper_left_corner;
        let oy = (offset.dot(&self.geometry.orientation)).abs() * self.size_scaling[0];
        let ox = (offset.dot(&right)).abs() * self.size_scaling[1];
        let color = self.texture.get_pixel(ox as u32, oy as u32);
        SOP::Light(color[0], color[1], color[2])
    }
}

/// Load an image to be used as a texture.
fn load_texture_from_file(filepath: &str) -> DynamicImage {
    Reader::open(filepath)
        .unwrap_or_else(|_| panic!("Could not load texture from {}", filepath))
        .decode()
        .unwrap_or_else(|_| panic!("Could not decode texture loaded from {}", filepath))
}

impl SurfaceBuilder for TexturedRectangleBuilder {
    fn build(self, vop_map: &HashMap<String, Arc<VOP>>) -> Arc<dyn Surface + Send + Sync> {
        // warn the user if image is not of appropriate dimensions
        let dyn_image = load_texture_from_file(&self.texture);
        let size_scaling = [
            dyn_image.height() as f64 / self.size[0],
            dyn_image.width() as f64 / self.size[1],
        ];
        if (size_scaling[0] - size_scaling[1]).abs() >= TOLERANCE {
            log::warn!("Texture {} will be rescaled.", self.texture);
        }

        Arc::new(TexturedRectangle {
            geometry: RectangleShape::new(
                Point3::from_slice(&self.origin),
                Vector3::from_row_slice(&self.normal),
                Vector3::from_row_slice(&self.orientation),
                self.size,
            ),
            size_scaling,
            texture: dyn_image
                .as_rgb8()
                .unwrap_or_else(|| {
                    panic!("Could not parse texture as RGB8 image: {}", self.texture)
                })
                .to_owned(),
            vop_above: vop_map
                .get(&self.vop_above)
                .expect("No VOP above mapping found.")
                .clone(),
            vop_below: vop_map
                .get(&self.vop_below)
                .expect("No VOP above mapping found.")
                .clone(),
        })
    }
}
