use crate::MageError;
use nalgebra::Vector3;
use rapier3d::parry::either::Either;
use rapier3d::parry::shape::{
    Ball, Capsule, Cone, ConvexPolyhedron, Cylinder, Polyline, Shape, TriMesh, Triangle,
};
use thiserror::Error;

const SUBDIVS: u32 = 64;

type ScaledResult<T> = Result<Either<T, ConvexPolyhedron>, MageError>;

#[derive(Debug, Error)]
pub enum ScalingError {
    #[error("The scaled approximation has degenerated normals")]
    DegeneratedNormalsWhileScaling,
}

pub trait ScalableShape<R = Self>: Shape {
    fn scale_shape(&self, scale: &Vector3<f32>) -> ScaledResult<R>;
}

impl ScalableShape for Ball {
    fn scale_shape(&self, scale: &Vector3<f32>) -> ScaledResult<Self> {
        self.scaled(scale, SUBDIVS)
            .ok_or_else(|| ScalingError::DegeneratedNormalsWhileScaling.into())
    }
}

impl ScalableShape for Capsule {
    fn scale_shape(&self, scale: &Vector3<f32>) -> ScaledResult<Self> {
        self.scaled(scale, SUBDIVS)
            .ok_or_else(|| ScalingError::DegeneratedNormalsWhileScaling.into())
    }
}

impl ScalableShape for Cylinder {
    fn scale_shape(&self, scale: &Vector3<f32>) -> ScaledResult<Self> {
        self.scaled(scale, SUBDIVS)
            .ok_or_else(|| ScalingError::DegeneratedNormalsWhileScaling.into())
    }
}

impl ScalableShape for Cone {
    fn scale_shape(&self, scale: &Vector3<f32>) -> ScaledResult<Self> {
        self.scaled(scale, SUBDIVS)
            .ok_or_else(|| ScalingError::DegeneratedNormalsWhileScaling.into())
    }
}

impl ScalableShape for Polyline {
    fn scale_shape(&self, scale: &Vector3<f32>) -> ScaledResult<Self> {
        Ok(Either::Left(self.clone().scaled(scale)))
    }
}

impl ScalableShape for Triangle {
    fn scale_shape(&self, scale: &Vector3<f32>) -> ScaledResult<Self> {
        Ok(Either::Left(self.scaled(scale)))
    }
}

impl ScalableShape for TriMesh {
    fn scale_shape(&self, scale: &Vector3<f32>) -> ScaledResult<Self> {
        Ok(Either::Left(self.clone().scaled(scale)))
    }
}
