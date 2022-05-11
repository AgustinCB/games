use hecs::Entity;
use nalgebra::Vector3;
use rapier3d::dynamics::{
    CCDSolver, ImpulseJointSet, IntegrationParameters, IslandManager, MultibodyJointSet, RigidBody,
    RigidBodySet,
};
use rapier3d::geometry::{BroadPhase, Collider, ColliderSet, NarrowPhase};
use rapier3d::pipeline::{EventHandler, PhysicsHooks, PhysicsPipeline};
use std::collections::HashMap;

pub struct PhysicsEngine<E: EventHandler, P: PhysicsHooks> {
    broad_phase: BroadPhase,
    ccd_solver: CCDSolver,
    collider_set: ColliderSet,
    colliders: HashMap<Entity, Collider>,
    event_handler: E,
    gravity: Vector3<f32>,
    impulse_joins: ImpulseJointSet,
    integration_parameters: IntegrationParameters,
    island_manager: IslandManager,
    multibody_joints: MultibodyJointSet,
    narrow_phase: NarrowPhase,
    physics_hooks: P,
    physics_pipeline: PhysicsPipeline,
    rigidbodies: HashMap<Entity, RigidBody>,
    rigidbody_set: RigidBodySet,
}

impl<E: EventHandler, P: PhysicsHooks> PhysicsEngine<E, P> {
    pub fn new(gravity: Vector3<f32>, hooks: P, handler: E) -> PhysicsEngine<E, P> {
        PhysicsEngine {
            broad_phase: BroadPhase::default(),
            ccd_solver: CCDSolver::default(),
            collider_set: ColliderSet::new(),
            colliders: HashMap::new(),
            event_handler: handler,
            impulse_joins: ImpulseJointSet::new(),
            integration_parameters: IntegrationParameters::default(),
            island_manager: IslandManager::new(),
            multibody_joints: MultibodyJointSet::new(),
            narrow_phase: NarrowPhase::default(),
            physics_hooks: hooks,
            physics_pipeline: PhysicsPipeline::default(),
            rigidbodies: HashMap::new(),
            rigidbody_set: RigidBodySet::new(),
            gravity,
        }
    }

    pub fn iter_mut_colliders(&mut self) -> impl Iterator<Item = &mut Collider> {
        self.colliders.values_mut()
    }

    pub fn iter_mut_rigidbody(&mut self) -> impl Iterator<Item = &mut RigidBody> {
        self.rigidbodies.values_mut()
    }

    pub fn add_collider(&mut self, entity: Entity, collider: Collider) {
        self.colliders.insert(entity, collider);
    }

    pub fn add_rigidbody(&mut self, entity: Entity, rigidbody: RigidBody) {
        self.rigidbodies.insert(entity, rigidbody);
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
