use crate::MageError;
use nalgebra::Vector3;
use rapier3d::geometry::{Collider, SharedShape};
use rapier3d::parry::either::Either;
use rapier3d::parry::shape::{
    Ball, Capsule, Cone, ConvexPolyhedron, Cylinder, Polyline, TriMesh, Triangle, TypedShape,
};
use thiserror::Error;

const SUBDIVS: u32 = 64;

#[derive(Debug, Error)]
pub enum ScalableShapeError {
    #[error("Unsupported shape")]
    UnsupportedShape,
    #[error("The scaled approximation has degenerated normals")]
    DegeneratedNormalsWhileScaling,
}

pub enum ScalableTypedShape {
    Ball(Ball),
    Capsule(Capsule),
    Cone(Cone),
    ConvexPolyhedron(ConvexPolyhedron),
    Cylinder(Cylinder),
    Polyline(Polyline),
    TriMesh(TriMesh),
    Triangle(Triangle),
}

impl ScalableTypedShape {
    pub fn set_to_collider(self, collider: &mut Collider) {
        match self {
            ScalableTypedShape::Ball(s) => collider.set_shape(SharedShape::new(s)),
            ScalableTypedShape::Capsule(s) => collider.set_shape(SharedShape::new(s)),
            ScalableTypedShape::Cone(s) => collider.set_shape(SharedShape::new(s)),
            ScalableTypedShape::ConvexPolyhedron(s) => collider.set_shape(SharedShape::new(s)),
            ScalableTypedShape::Cylinder(s) => collider.set_shape(SharedShape::new(s)),
            ScalableTypedShape::Polyline(s) => collider.set_shape(SharedShape::new(s)),
            ScalableTypedShape::TriMesh(s) => collider.set_shape(SharedShape::new(s)),
            ScalableTypedShape::Triangle(s) => collider.set_shape(SharedShape::new(s)),
        }
    }
}

pub fn scale_shape<'a>(
    shape: &TypedShape<'a>,
    scale: &Vector3<f32>,
) -> Result<ScalableTypedShape, MageError> {
    match shape {
        TypedShape::Ball(b) => b
            .scaled(scale, SUBDIVS)
            .map(|e| match e {
                Either::Left(b) => ScalableTypedShape::Ball(b),
                Either::Right(o) => ScalableTypedShape::ConvexPolyhedron(o),
            })
            .ok_or_else(|| ScalableShapeError::DegeneratedNormalsWhileScaling.into()),
        TypedShape::Capsule(c) => c
            .scaled(scale, SUBDIVS)
            .map(|e| match e {
                Either::Left(c) => ScalableTypedShape::Capsule(c),
                Either::Right(o) => ScalableTypedShape::ConvexPolyhedron(o),
            })
            .ok_or_else(|| ScalableShapeError::DegeneratedNormalsWhileScaling.into()),
        TypedShape::Cone(c) => c
            .scaled(scale, SUBDIVS)
            .map(|e| match e {
                Either::Left(c) => ScalableTypedShape::Cone(c),
                Either::Right(o) => ScalableTypedShape::ConvexPolyhedron(o),
            })
            .ok_or_else(|| ScalableShapeError::DegeneratedNormalsWhileScaling.into()),
        TypedShape::Cylinder(c) => c
            .scaled(scale, SUBDIVS)
            .map(|e| match e {
                Either::Left(c) => ScalableTypedShape::Cylinder(c),
                Either::Right(o) => ScalableTypedShape::ConvexPolyhedron(o),
            })
            .ok_or_else(|| ScalableShapeError::DegeneratedNormalsWhileScaling.into()),
        TypedShape::Polyline(p) => Ok(ScalableTypedShape::Polyline((*p).clone().scaled(scale))),
        TypedShape::Triangle(t) => Ok(ScalableTypedShape::Triangle(t.scaled(scale))),
        TypedShape::TriMesh(t) => Ok(ScalableTypedShape::TriMesh((*t).clone().scaled(scale))),
        _ => Err(ScalableShapeError::UnsupportedShape.into()),
    }
}
