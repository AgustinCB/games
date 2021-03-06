use crate::core::system::System;
use crate::physics::engine::PhysicsEngine;
use crate::physics::scalable_shape::scale_shape;
use crate::rendering::Transform;
use approx::RelativeEq;
use hecs::World as HecsWorld;
use log::error;
use nalgebra::{vector, Vector3};
use rapier3d::pipeline::{EventHandler, PhysicsHooks};

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

pub struct World<E: EventHandler, P: PhysicsHooks> {
    pub(crate) physics_engine: PhysicsEngine<E, P>,
    systems: Vec<Box<dyn System>>,
    pub(crate) world: HecsWorld,
}

impl World<(), ()> {
    pub fn new() -> World<(), ()> {
        World {
            physics_engine: PhysicsEngine::new(GRAVITY, (), ()),
            systems: vec![],
            world: HecsWorld::new(),
        }
    }
}

impl<E: EventHandler, P: PhysicsHooks> World<E, P> {
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
}

impl Default for World<(), ()> {
    fn default() -> Self {
        Self::new()
    }
}
