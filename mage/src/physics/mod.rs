use hecs::Entity;
use nalgebra::Vector3;
pub use rapier3d::dynamics::RigidBodyBuilder;
pub use rapier3d::geometry::{ActiveCollisionTypes, ColliderBuilder};
pub use rapier3d::pipeline::ActiveEvents;

pub mod engine;
pub mod scalable_shape;

#[derive(Clone, Debug)]
pub struct Collision {
    pub entity_id: Entity,
    pub started: bool,
}

#[derive(Clone, Debug)]
pub struct Collisions(pub Vec<Collision>);

pub struct Triggers(pub Vec<Collision>);

pub struct Velocity(pub Vector3<f32>);

