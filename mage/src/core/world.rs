use std::collections::HashMap;

use approx::RelativeEq;
use hecs::World as HecsWorld;
use log::error;
use nalgebra::{vector, Vector3};
use rapier3d::crossbeam::channel::{Receiver, unbounded};
use rapier3d::pipeline::ChannelEventCollector;
use rapier3d::prelude::CollisionEvent;

use crate::core::system::System;
use crate::physics::{Collision, Collisions, Triggers, Velocity};
use crate::physics::engine::PhysicsEngine;
use crate::physics::scalable_shape::scale_shape;
use crate::rendering::Transform;

const GRAVITY: Vector3<f32> = vector!(0.0, -9.81, 0.0);

fn handle_result<T, E: ToString>(result: Result<T, E>) -> Option<T> {
    match result {
        Ok(v) => Some(v),
        Err(s) => {
            error!("{}", &s.to_string());
            None
        }
    }
}

pub struct World {
    events_receiver: Receiver<CollisionEvent>,
    pub(crate) physics_engine: PhysicsEngine,
    systems: Vec<Box<dyn System>>,
    pub(crate) world: HecsWorld,
}

impl World {
    pub fn new() -> World {
        let (events_sender, events_receiver) = unbounded();
        World {
            events_receiver,
            physics_engine: PhysicsEngine::new(
                GRAVITY,
                ChannelEventCollector::new(events_sender),
            ),
            systems: vec![],
            world: HecsWorld::new(),
        }
    }
}

impl World {
    pub fn add_system(&mut self, system: Box<dyn System>) {
        self.systems.push(system);
    }

    pub fn get_mut(&mut self) -> &mut HecsWorld {
        &mut self.world
    }

    pub fn start(&mut self) {
        for system in self.systems.iter() {
            handle_result(
                system
                    .start(&mut self.world)
                    .map_err(|s| format!("There was an error on {}: {}", system.name(), &s)),
            );
        }
    }

    pub fn early_update(&mut self, delta_time: u64) {
        for system in self.systems.iter() {
            handle_result(
                system
                    .early_update(&mut self.world, delta_time)
                    .map_err(|s| format!("There was an error on {}: {}", system.name(), &s)),
            );
        }
    }

    pub fn update(&mut self, delta_time: u64) {
        self.physics_engine.step();

        self.clean_collisions();
        self.add_collisions();

        for (entity, r) in self.physics_engine.iter_rigidbody() {
            if let Some(transform) =
            handle_result(self.world.query_one_mut::<&mut Transform>(entity))
            {
                transform.position = *r.translation();
                transform.rotation = *r.rotation();
            }
        }

        for (entity, c) in self.physics_engine.iter_colliders() {
            if let Some(transform) =
                handle_result(self.world.query_one_mut::<&mut Transform>(entity))
            {
                transform.position = *c.translation();
                transform.rotation = *c.rotation();
            }
        }
        for system in self.systems.iter() {
            handle_result(
                system
                    .update(&mut self.world, delta_time)
                    .map_err(|s| format!("There was an error on {}: {}", system.name(), &s)),
            );
        }
    }

    pub fn late_update(&mut self, delta_time: u64) {
        for system in self.systems.iter() {
            handle_result(
                system
                    .late_update(&mut self.world, delta_time)
                    .map_err(|s| format!("There was an error on {}: {}", system.name(), &s)),
            );
        }

        for (entity, r) in self.physics_engine.iter_mut_rigidbody() {
            if let Some(mut transform) =
                handle_result(self.world.query_one::<&mut Transform>(entity))
            {
                if let Some(transform) = transform.get() {
                    if !r
                        .translation()
                        .relative_eq(&transform.position, f32::EPSILON, f32::EPSILON)
                    {
                        r.set_translation(transform.position, false);
                    }
                    if !r.rotation().clone().relative_eq(
                        &transform.rotation,
                        f32::EPSILON,
                        f32::EPSILON,
                    ) {
                        r.set_rotation(transform.rotation.scaled_axis(), false);
                    }
                }
            }
            if let Some(mut velocity) =
            handle_result(self.world.query_one::<&Velocity>(entity)) && r.is_kinematic() {
                if let Some(velocity) = velocity.get() {
                    r.set_linvel(velocity.0, true);
                }
            }
        }

        let mut new_scales = vec![];
        for (entity, c, handle, scale) in self.physics_engine.iter_mut_colliders() {
            if let Some(mut transform) =
                handle_result(self.world.query_one::<&mut Transform>(entity))
            {
                if let Some(transform) = transform.get() {
                    if !c
                        .translation()
                        .relative_eq(&transform.position, f32::EPSILON, f32::EPSILON)
                    {
                        c.set_translation(transform.position);
                    }
                    if !c.rotation().clone().relative_eq(
                        &transform.rotation,
                        f32::EPSILON,
                        f32::EPSILON,
                    ) {
                        c.set_rotation(transform.rotation.scaled_axis());
                    }
                    if !scale.relative_eq(&transform.scale, f32::EPSILON, f32::EPSILON) {
                        if let Some(shape) = handle_result(scale_shape(
                            &c.shape().as_typed_shape(),
                            &transform.scale,
                        )) {
                            shape.set_to_collider(c);
                            new_scales.push((handle, transform.scale));
                        }
                    }
                }
            }
        }
        self.physics_engine.set_scales(new_scales);
    }

    fn add_collisions(&mut self) {
        let mut collisions_per_entity = HashMap::new();
        while !self.events_receiver.is_empty() {
            if let Some(collision) = handle_result(self.events_receiver.recv()) && !collision.sensor() {
                let entity_handler1 = handle_result(
                    self.physics_engine.get_entity_from_collider(collision.collider1())
                );
                let entity_handler2 = handle_result(
                    self.physics_engine.get_entity_from_collider(collision.collider2())
                );
                if let (
                    Some(entity_handler1), Some(entity_handler2)
                ) = (entity_handler1, entity_handler2) {
                    collisions_per_entity.entry(entity_handler2).or_insert(vec![]).push(
                        Collision {
                            entity_id: entity_handler1,
                            started: collision.started(),
                        }
                    );
                    collisions_per_entity.entry(entity_handler1).or_insert(vec![]).push(
                        Collision {
                            entity_id: entity_handler2,
                            started: collision.started(),
                        }
                    );
                }
            }
        }
        for (entity, collisions) in collisions_per_entity.into_iter() {
            handle_result(self.world.insert_one(entity, Collisions(collisions)));
        }
    }

    fn clean_collisions(&mut self) {
        let mut to_remove = vec![];
        for (entity, _collisions) in self.world.query::<&Collisions>().iter() {
            to_remove.push(entity);
        }
        for e in to_remove.iter().cloned() {
            let _ = self.world.remove_one::<Collisions>(e);
        }
        to_remove.clear();

        for (entity, _triggers) in self.world.query::<&Triggers>().iter() {
            to_remove.push(entity);
        }
        for e in to_remove.iter().cloned() {
            let _ = self.world.remove_one::<Triggers>(e);
        }
    }
}

impl Default for World {
    fn default() -> Self {
        Self::new()
    }
}
