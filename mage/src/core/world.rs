use std::collections::HashMap;
use std::time::Duration;

use approx::RelativeEq;
use hecs::World as HecsWorld;
use log::error;
use nalgebra::{vector, Vector3};
use rapier3d::crossbeam::channel::{Receiver, unbounded};
use rapier3d::pipeline::ChannelEventCollector;
use rapier3d::prelude::{Collider, CollisionEvent};

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
            physics_engine: PhysicsEngine::new(GRAVITY, ChannelEventCollector::new(events_sender)),
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
        self.remove_colliders_and_rigidbodies();
        self.add_colliders();

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
        self.update_transforms();

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

        self.update_rigidbodies();
        self.update_colliders();
    }

    fn update_colliders(&mut self) {
        let mut new_scales = vec![];
        for (entity, c, handle, scale) in self.physics_engine.iter_mut_colliders() {
            if !self.world.contains(entity) {
                continue;
            }
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

    fn update_rigidbodies(&mut self) {
        for (entity, r) in self.physics_engine.iter_mut_rigidbody() {
            if !self.world.contains(entity) {
                continue;
            }
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
            if let Some(mut velocity) = handle_result(self.world.query_one::<&Velocity>(entity)) {
                if let Some(velocity) = velocity.get() {
                    r.set_linvel(velocity.0, true);
                }
            }
        }
    }

    fn update_transforms(&mut self) {
        for (entity, _r, r) in self.physics_engine.iter_rigidbody() {
            if let Some(transform) =
            handle_result(self.world.query_one_mut::<&mut Transform>(entity))
            {
                transform.position = *r.translation();
                transform.rotation = *r.rotation();
            }
        }

        for (entity, _h, c) in self.physics_engine.iter_colliders() {
            if let Some(transform) =
            handle_result(self.world.query_one_mut::<&mut Transform>(entity))
            {
                transform.position = *c.translation();
                transform.rotation = *c.rotation();
            }
        }
    }

    fn add_colliders(&mut self) {
        let mut colliders = vec![];
        for (e, c) in self.world.query::<&Collider>().iter() {
            colliders.push((e, c.clone()));
        }
        for (e, c) in colliders {
            self.physics_engine.add_collider(e, c);
            handle_result(self.world.remove_one::<Collider>(e));
        }
    }

    fn remove_colliders_and_rigidbodies(&mut self) {
        let mut colliders_to_remove = vec![];
        for (e, collider_handle, _colliders) in self.physics_engine.iter_colliders() {
            if !self.world.contains(e) {
                colliders_to_remove.push(collider_handle);
            }
        }
        for collider_handle in colliders_to_remove {
            self.physics_engine.remove_collider(collider_handle);
        }
        let mut rigidbodies_to_remove = vec![];
        for (e, rigidbody_handle, _rigidbody) in self.physics_engine.iter_rigidbody() {
            if !self.world.contains(e) {
                rigidbodies_to_remove.push(rigidbody_handle);
            }
        }
        for rigidbody_handle in rigidbodies_to_remove {
            self.physics_engine.remove_rigidbody(rigidbody_handle);
        }
    }

    fn add_collisions(&mut self) {
        let mut collisions_per_entity = HashMap::new();
        let mut triggers_per_entity = HashMap::new();
        while !self.events_receiver.is_empty() {
            if let Some(collision) = handle_result(self.events_receiver.recv_timeout(Duration::from_nanos(0))) {
                let entity_handler1 =
                    self.physics_engine.get_entity_from_collider(collision.collider1());
                let entity_handler2 =
                    self.physics_engine.get_entity_from_collider(collision.collider2());
                let user_data1 = self.physics_engine.get_user_data_from_collider(collision.collider1());
                let user_data2 = self.physics_engine.get_user_data_from_collider(collision.collider2());
                let collection = if collision.sensor() {
                    &mut triggers_per_entity
                } else {
                    &mut collisions_per_entity
                };
                if let (
                    Some(entity_handler1), Some(entity_handler2), Some(user_data1), Some(user_data2)
                ) = (entity_handler1, entity_handler2, user_data1, user_data2) {
                    if collision.started() {
                        if let Some(contact_pair) = self.physics_engine.contact_pair(collision.collider1(), collision.collider2()) {
                            collection.entry(entity_handler2).or_insert(vec![]).push(
                                Collision::Started(entity_handler1, contact_pair.clone(), user_data1),
                            );
                            collection.entry(entity_handler1).or_insert(vec![]).push(
                                Collision::Started(entity_handler2, contact_pair.clone(), user_data2),
                            );
                        } else {
                            collection.entry(entity_handler2).or_insert(vec![]).push(
                                Collision::StartedTrigger(entity_handler1, user_data1),
                            );
                            collection.entry(entity_handler1).or_insert(vec![]).push(
                                Collision::StartedTrigger(entity_handler2, user_data2),
                            );
                        }
                    } else {
                        collection.entry(entity_handler2).or_insert(vec![]).push(
                            Collision::Stopped(entity_handler1, user_data1),
                        );
                        collection.entry(entity_handler1).or_insert(vec![]).push(
                            Collision::Stopped(entity_handler2, user_data2),
                        );
                    }
                }
            }
        }
        for (entity, collisions) in triggers_per_entity.into_iter() {
            let _ = self
                .world
                .query_one_mut::<(&mut Triggers, )>(entity)
                .map(|(mut r, )| {
                    r.0 = collisions;
                });
        }
        for (entity, collisions) in collisions_per_entity.into_iter() {
            let _ = self
                .world
                .query_one_mut::<(&mut Collisions, )>(entity)
                .map(|(mut r, )| {
                    r.0 = collisions;
                });
        }
    }

    fn clean_collisions(&mut self) {
        for (_, collisions) in self.world.query_mut::<&mut Collisions>() {
            collisions.0.clear();
        }

        for (_, triggers) in self.world.query_mut::<&mut Triggers>() {
            triggers.0.clear();
        }
    }
}

impl Default for World {
    fn default() -> Self {
        Self::new()
    }
}
