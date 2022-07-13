use hecs::Entity;
use nalgebra::Vector3;
pub use rapier3d::dynamics::LockedAxes;
pub use rapier3d::dynamics::RigidBodyBuilder;
pub use rapier3d::geometry::{ActiveCollisionTypes, ColliderBuilder};
pub use rapier3d::pipeline::ActiveEvents;
use rapier3d::prelude::ContactPair;

pub mod engine;
pub mod scalable_shape;

#[derive(Clone)]
pub enum Collision {
    Started(Entity, ContactPair, u128),
    StartedTrigger(Entity, u128),
    Stopped(Entity, u128),
}

impl Collision {
    pub fn started(&self) -> bool {
        matches!(self, Collision::Started(_, _, _) | Collision::StartedTrigger(_, _))
    }

    pub fn entity_id(&self) -> Entity {
        match self {
            Collision::Started(entity, _, _) | Collision::StartedTrigger(entity, _) |
            Collision::Stopped(entity, _) => *entity,
        }
    }

    pub fn user_data(&self) -> u128 {
        match self {
            Collision::Started(_, _, u) | Collision::StartedTrigger(_, u) |
            Collision::Stopped(_, u) => *u,
        }
    }
}

#[derive(Clone)]
pub struct Collisions(pub Vec<Collision>);

pub struct Triggers(pub Vec<Collision>);

pub struct Velocity(pub Vector3<f32>);
