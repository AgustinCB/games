use std::collections::HashMap;

use hecs::Entity;
use nalgebra::Vector3;
use rapier3d::dynamics::{
    CCDSolver, ImpulseJointSet, IntegrationParameters, IslandManager, MultibodyJointSet, RigidBody,
    RigidBodyHandle, RigidBodySet,
};
use rapier3d::geometry::{BroadPhase, Collider, ColliderHandle, ColliderSet, NarrowPhase};
use rapier3d::pipeline::{ChannelEventCollector, PhysicsPipeline};
use rapier3d::prelude::ContactPair;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum PhysicsEngineError {
    #[error("Missing collision for entity")]
    MissingCollision,
}

pub struct PhysicsEngine {
    broad_phase: BroadPhase,
    ccd_solver: CCDSolver,
    collider_scale: HashMap<ColliderHandle, Vector3<f32>>,
    collider_set: ColliderSet,
    colliders: HashMap<ColliderHandle, Entity>,
    event_handler: ChannelEventCollector,
    gravity: Vector3<f32>,
    impulse_joins: ImpulseJointSet,
    integration_parameters: IntegrationParameters,
    island_manager: IslandManager,
    multibody_joints: MultibodyJointSet,
    narrow_phase: NarrowPhase,
    physics_hooks: (),
    physics_pipeline: PhysicsPipeline,
    rigidbodies: HashMap<RigidBodyHandle, Entity>,
    rigidbody_set: RigidBodySet,
}

impl PhysicsEngine {
    pub fn new(gravity: Vector3<f32>, event_handler: ChannelEventCollector) -> PhysicsEngine {
        PhysicsEngine {
            event_handler,
            broad_phase: BroadPhase::default(),
            ccd_solver: CCDSolver::default(),
            collider_scale: HashMap::new(),
            collider_set: ColliderSet::new(),
            colliders: HashMap::new(),
            impulse_joins: ImpulseJointSet::new(),
            integration_parameters: IntegrationParameters::default(),
            island_manager: IslandManager::new(),
            multibody_joints: MultibodyJointSet::new(),
            narrow_phase: NarrowPhase::default(),
            physics_hooks: (),
            physics_pipeline: PhysicsPipeline::default(),
            rigidbodies: HashMap::new(),
            rigidbody_set: RigidBodySet::new(),
            gravity,
        }
    }

    pub fn iter_colliders(&self) -> impl Iterator<Item=(Entity, ColliderHandle, &Collider)> {
        self.collider_set
            .iter()
            .map(|(h, c)| (*self.colliders.get(&h).unwrap(), h, c))
    }

    pub fn iter_rigidbody(&self) -> impl Iterator<Item=(Entity, RigidBodyHandle, &RigidBody)> {
        self.rigidbody_set
            .iter()
            .map(|(h, r)| (*self.rigidbodies.get(&h).unwrap(), h, r))
    }

    pub fn iter_mut_colliders(
        &mut self,
    ) -> impl Iterator<Item = (Entity, &mut Collider, ColliderHandle, Vector3<f32>)> {
        self.collider_set.iter_mut().map(|(h, c)| {
            (
                *self.colliders.get(&h).unwrap(),
                c,
                h,
                *self.collider_scale.get(&h).unwrap(),
            )
        })
    }

    pub fn iter_mut_rigidbody(&mut self) -> impl Iterator<Item=(Entity, &mut RigidBody)> {
        self.rigidbody_set
            .iter_mut()
            .map(|(h, r)| (*self.rigidbodies.get(&h).unwrap(), r))
    }

    pub fn get_entity_from_collider(&self, collider: ColliderHandle) -> Option<Entity> {
        self.colliders.get(&collider).map(|e| *e)
    }

    pub fn get_user_data_from_collider(&self, handle: ColliderHandle) -> Option<u128> {
        self.collider_set.get(handle).map(|c| c.user_data)
    }

    pub fn remove_rigidbody(&mut self, handle: RigidBodyHandle) {
        self.rigidbody_set.remove(
            handle,
            &mut self.island_manager,
            &mut self.collider_set,
            &mut self.impulse_joins,
            &mut self.multibody_joints,
            true,
        );
        self.rigidbodies.remove(&handle);
    }

    pub fn remove_collider(&mut self, handle: ColliderHandle) {
        self.collider_set.remove(
            handle,
            &mut self.island_manager,
            &mut self.rigidbody_set,
            true,
        );
        self.colliders.remove(&handle);
        self.collider_scale.remove(&handle);
    }

    pub fn add_collider(&mut self, entity: Entity, collider: Collider) {
        let handle = self.collider_set.insert(collider);
        self.colliders.insert(handle, entity);
        self.collider_scale
            .insert(handle, Vector3::new(1.0, 1.0, 1.0));
    }

    pub fn add_rigidbody(&mut self, entity: Entity, rigidbody: RigidBody) {
        let handle = self.rigidbody_set.insert(rigidbody);
        self.rigidbodies.insert(handle, entity);
    }

    pub fn add_collider_and_rigidbody(
        &mut self,
        entity: Entity,
        collider: Collider,
        rigidbody: RigidBody,
    ) {
        let body_handle = self.rigidbody_set.insert(rigidbody);
        self.rigidbodies.insert(body_handle, entity);

        let collider_handle =
            self.collider_set
                .insert_with_parent(collider, body_handle, &mut self.rigidbody_set);
        self.colliders.insert(collider_handle, entity);
        self.collider_scale
            .insert(collider_handle, Vector3::new(1.0, 1.0, 1.0));
    }

    pub fn contact_pair(&self, collider_handle1: ColliderHandle, collider_handle2: ColliderHandle) -> Option<&ContactPair> {
        self.narrow_phase.contact_pair(collider_handle1, collider_handle2)
    }

    pub fn set_scales(&mut self, scales: Vec<(ColliderHandle, Vector3<f32>)>) {
        for (handle, scale) in scales {
            self.collider_scale.insert(handle, scale);
        }
    }

    pub fn step(&mut self) {
        self.physics_pipeline.step(
            &self.gravity,
            &self.integration_parameters,
            &mut self.island_manager,
            &mut self.broad_phase,
            &mut self.narrow_phase,
            &mut self.rigidbody_set,
            &mut self.collider_set,
            &mut self.impulse_joins,
            &mut self.multibody_joints,
            &mut self.ccd_solver,
            &self.physics_hooks,
            &self.event_handler,
        );
    }
}
