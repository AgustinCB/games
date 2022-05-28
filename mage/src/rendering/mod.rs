use nalgebra::{Matrix4, Scale3, Translation3, Vector3};
use rapier3d::math::Rotation;

pub mod engine;
pub mod model;
pub mod opengl;

#[derive(Clone, Debug)]
pub struct Transform {
    pub position: Vector3<f32>,
    pub rotation: Rotation<f32>,
    pub scale: Vector3<f32>,
}

impl Transform {
    pub fn identity() -> Transform {
        Transform {
            position: Vector3::new(0f32, 0f32, 0f32),
            rotation: Rotation::identity(),
            scale: Vector3::new(1f32, 1f32, 1f32),
        }
    }

    pub fn get_model_matrix(&self) -> Matrix4<f32> {
        let t = Translation3::from(self.position);
        let s = Scale3::from(self.scale);
        eprintln!(
            "{:?}",
            t.to_homogeneous() * self.rotation.to_homogeneous() * s.to_homogeneous()
        );
        t.to_homogeneous() * self.rotation.to_homogeneous() * s.to_homogeneous()
    }
}

pub struct TransformBuilder {
    transform: Transform,
}

impl TransformBuilder {
    pub fn new() -> TransformBuilder {
        TransformBuilder {
            transform: Transform::identity(),
        }
    }

    pub fn with_rotation(&self, rotation: Rotation<f32>) -> TransformBuilder {
        let mut transform = self.transform.clone();
        transform.rotation = rotation;
        TransformBuilder { transform }
    }

    pub fn with_position(&self, position: Vector3<f32>) -> TransformBuilder {
        let mut transform = self.transform.clone();
        transform.position = position;
        TransformBuilder { transform }
    }

    pub fn with_scale(&self, scale: Vector3<f32>) -> TransformBuilder {
        let mut transform = self.transform.clone();
        transform.scale = scale;
        TransformBuilder { transform }
    }

    pub fn build(self) -> Transform {
        self.transform
    }
}

impl Default for TransformBuilder {
    fn default() -> TransformBuilder {
        TransformBuilder::new()
    }
}
