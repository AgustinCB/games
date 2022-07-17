use std::cell::RefCell;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

use hecs::World;
use nalgebra::Vector3;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use mage::core::system::System;
use mage::gameplay::input::Input;
use mage::MageError;
use mage::physics::{Collisions, Velocity};
use mage::rendering::Transform;

use crate::LevelElement;

pub(crate) struct PlayerVelocity(pub(crate) f32);

pub(crate) struct PlayerControlsSystem {
    pub(crate) against_left_wall: RefCell<bool>,
    pub(crate) against_right_wall: RefCell<bool>,
    pub(crate) hx: f32,
    pub(crate) unstick: Arc<AtomicBool>,
    pub(crate) width: f32,
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
        for (_e, (collisions, input, player_velocity, velocity, transform)) in world.query_mut::<(
            &Collisions,
            &Input,
            &PlayerVelocity,
            &mut Velocity,
            &mut Transform,
        )>() {
            for collision in &collisions.0 {
                let element = LevelElement::from(collision.user_data() as u8);
                match element {
                    LevelElement::RightWall => {
                        self.against_right_wall.replace(collision.started());
                    }
                    LevelElement::LeftWall => {
                        self.against_left_wall.replace(collision.started());
                    }
                    _ => {}
                };
            }

            let mut x_velocity = 0f32;
            let player_velocity = player_velocity.0 * (delta_time as f32 / 1000.0);
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
                    Event::KeyDown {
                        keycode: Some(Keycode::Space), ..
                    } if !self.unstick.load(Ordering::Relaxed) => {
                        self.unstick.store(true, Ordering::Relaxed);
                    }
                    _ => {}
                }
            }
            *velocity = Velocity(Vector3::new(x_velocity, 0.0, 0.0));
            if *self.against_left_wall.borrow() {
                transform.position.x = self.hx;
            } else if *self.against_right_wall.borrow() {
                transform.position.x = self.width - self.hx;
            }
        }
        Ok(())
    }

    fn late_update(&self, _world: &mut World, _delta_time: u64) -> Result<(), MageError> {
        Ok(())
    }
}
