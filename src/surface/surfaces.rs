use collections::HashMap;
use serde::Deserialize;
use std::{collections, error::Error};

use crate::{
    shape::{InfinitePlaneShape, ParaboloidShape, RectangleShape, SphereShape},
    Point3D, Shape, Surface, Vector3D, SOP, VOP,
};

use super::SurfaceBuilder;

pub struct Rectangle<'a> {
    geometry: RectangleShape,
    sop: SOP,
    vop_above: &'a VOP,
    vop_below: &'a VOP,
}

impl<'a> Rectangle<'a> {
    pub fn new(
        origin: Point3D,
        normal: Vector3D,
        orientation: Vector3D,
        size: (f64, f64),
        sop: SOP,
        vop_above: &'a VOP,
        vop_below: &'a VOP,
    ) -> Rectangle<'a> {
        Self {
            geometry: RectangleShape::new(origin, normal, orientation, size),
            sop,
            vop_above,
            vop_below,
        }
    }
}

impl<'a> Surface<'a> for Rectangle<'a> {
    fn geometry(&self) -> &dyn Shape {
        &self.geometry
    }
    fn vop_above_at(&self, _point: &Point3D) -> &'a VOP {
        &self.vop_above
    }
    fn vop_below_at(&self, _point: &Point3D) -> &'a VOP {
        &self.vop_below
    }
    fn sop_at(&self, _point: &Point3D) -> SOP {
        self.sop
    }
}

pub struct Plane<'a> {
    geometry: InfinitePlaneShape,
    sop: SOP,
    vop_above: &'a VOP,
    vop_below: &'a VOP,
}

impl<'a> Plane<'a> {
    pub fn new(
        origin: Point3D,
        normal: Vector3D,
        sop: SOP,
        vop_above: &'a VOP,
        vop_below: &'a VOP,
    ) -> Plane<'a> {
        Self {
            geometry: InfinitePlaneShape::new(origin, normal),
            sop,
            vop_above,
            vop_below,
        }
    }
}

impl<'a> Surface<'a> for Plane<'a> {
    fn geometry(&self) -> &dyn Shape {
        &self.geometry
    }
    fn vop_above_at(&self, _: &Point3D) -> &'a VOP {
        &self.vop_above
    }
    fn vop_below_at(&self, _: &Point3D) -> &'a VOP {
        &self.vop_below
    }
    fn sop_at(&self, _: &Point3D) -> SOP {
        self.sop
    }
}

pub struct Sphere<'a> {
    geometry: SphereShape,
    sop: SOP,
    vop_above: &'a VOP,
    vop_below: &'a VOP,
}

impl<'a> Sphere<'a> {
    pub fn new(
        center: Point3D,
        radius: f64,
        sop: SOP,
        vop_above: &'a VOP,
        vop_below: &'a VOP,
    ) -> Sphere<'a> {
        Self {
            geometry: SphereShape::new(center, radius),
            sop,
            vop_above,
            vop_below,
        }
    }
}

impl<'a> Surface<'a> for Sphere<'a> {
    fn geometry(&self) -> &dyn Shape {
        &self.geometry
    }
    fn vop_above_at(&self, _point: &Point3D) -> &'a VOP {
        &self.vop_above
    }
    fn vop_below_at(&self, _point: &Point3D) -> &'a VOP {
        &self.vop_below
    }
    fn sop_at(&self, _point: &Point3D) -> SOP {
        self.sop
    }
}

pub struct ZParaboloid<'a> {
    geometry: ParaboloidShape,
    sop: SOP,
    vop_above: &'a VOP,
    vop_below: &'a VOP,
}

#[derive(Deserialize)]
pub struct ZParaboloidBuilder {
    pub origin: [f64; 3],
    pub a: f64,
    pub b: f64,
    pub sop: SOP,
    pub vop_above: String,
    pub vop_below: String,
}

impl<'a> ZParaboloidBuilder {
    fn build(&self, vop_map: &'a)
}

impl<'a> ZParaboloid<'a> {
    pub fn new(
        x0: f64,
        y0: f64,
        z0: f64,
        asq: f64,
        bsq: f64,
        sop: SOP,
        vop_above: &'a VOP,
        vop_below: &'a VOP,
    ) -> ZParaboloid<'a> {
        Self {
            geometry: ParaboloidShape::new(x0, y0, z0, asq, bsq),
            sop,
            vop_above,
            vop_below,
        }
    }
}

impl<'a> Surface<'a> for ZParaboloid<'a> {
    fn geometry(&self) -> &dyn Shape {
        &self.geometry
    }
    fn vop_above_at(&self, _: &Point3D) -> &'a VOP {
        &self.vop_above
    }
    fn vop_below_at(&self, _: &Point3D) -> &'a VOP {
        &self.vop_below
    }
    fn sop_at(&self, _: &Point3D) -> SOP {
        self.sop
    }
}
