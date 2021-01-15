mod shape2d;
mod shape3d;

use crate::{
    geometry::{Point3D, Vector3D, VOP},
    light::Ray,
};

pub trait Shape {
    fn intersections(&self, ray: &Ray) -> Vec<Point3D>;
    fn first_intersection(&self, ray: &Ray) -> Option<Point3D> {
        let intersections = self.intersections(ray);
        if intersections.len() <= 1 {
            return intersections.first().cloned();
        }

        let distances: Vec<f64> = intersections
            .iter()
            .map(|i| (ray.origin - *i).length_squared())
            .collect();

        // TODO: optimize this
        let index_of_min = distances
            .iter()
            .enumerate()
            .min_by(|(_, a), (_, b)| {
                a.partial_cmp(b)
                    .expect("NaN encountered in Point-to-Point distance.")
            })
            .map(|(index, _)| index)
            .unwrap();
        intersections.get(index_of_min).cloned()
    }
    fn intersects(&self, ray: &Ray) -> bool;
    fn contains(&self, point: &Point3D) -> bool;
    fn normal_at(&self, point: &Point3D) -> Option<Vector3D>;
    fn bounce(&self, ray: &mut Ray);
    fn vop_above(&self) -> &VOP;
    fn vop_below(&self) -> &VOP;
}
