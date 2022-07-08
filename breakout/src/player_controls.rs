use std::cell::RefCell;
use std::sync::Arc;
use std::sync::atomic::{AtomicU32, Ordering};

use hecs::World;
use nalgebra::Vector3;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use mage::core::system::System;
use mage::gameplay::input::Input;
use mage::MageError;
use mage::physics::{Collisions, Velocity};

use crate::LevelElement;

pub(crate) struct PlayerControlsSystem {
    pub(crate) player_velocity: Arc<AtomicU32>,
    pub(crate) against_left_wall: RefCell<bool>,
    pub(crate) against_right_wall: RefCell<bool>,
}

impl System for PlayerControlsSystem {
    fn name(&self) -> &str {
        "Player Controls"
    }

    fn start(&self, _world: &mut World) -> Result<(), MageError> {
        Ok(())
    }

    fn early_update(&self, _world: &mut World, _delta_time: u64) -> Result<(), MageError> {
        Ok(())
    }

    fn update(&self, world: &mut World, delta_time: u64) -> Result<(), MageError> {
        self.update_collisions(world);
        self.update_velocity(world, delta_time);
        Ok(())
    }

    fn late_update(&self, _world: &mut World, _delta_time: u64) -> Result<(), MageError> {
        Ok(())
    }
}

impl PlayerControlsSystem {
    fn update_collisions(&self, world: &mut World) {
        for (e, collisions) in world.query::<&Collisions>().iter() {
            if !world.query_one::<&Input>(e).map_or(false, |mut q| q.get().is_some()) {
                continue;
            }
            for collision in &collisions.0 {
                if let Ok(mut result) = world.query_one::<&LevelElement>(collision.entity_id) {
                    let result = result.get();
                    match result {
                        Some(LevelElement::RightWall) => {
                            self.against_right_wall.replace(collision.started);
                        },
                        Some(LevelElement::LeftWall) => {
                            self.against_left_wall.replace(collision.started);
                        },
                        _ => {}
                    }
                }
            }
        }
    }

    fn update_velocity(&self, world: &mut World, delta_time: u64) {
        for (_e, (input, velocity)) in world.query_mut::<(&Input, &mut Velocity)>() {
            let mut x_velocity = 0f32;
            let player_velocity =
                self.player_velocity.load(Ordering::Relaxed) as f32 * (delta_time as f32 / 1000.0);
            for e in input.events.iter() {
                match e {
                    Event::KeyDown {
                        keycode: Some(Keycode::A),
                        ..
                    }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Left),
                        ..
                    } if !*self.against_left_wall.borrow() => {
                        x_velocity -= player_velocity;
                    }
                    Event::KeyDown {
                        keycode: Some(Keycode::D),
                        ..
                    }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Right),
                        ..
                    } if !*self.against_right_wall.borrow() => {
                        x_velocity += player_velocity;
                    }
                    _ => {}
                }
            }
            *velocity = Velocity(Vector3::new(x_velocity, 0.0, 0.0));
        }
    }
}
