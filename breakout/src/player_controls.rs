use std::sync::Arc;
use std::sync::atomic::{AtomicU32, Ordering};

use hecs::World;
use log::error;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use mage::core::system::System;
use mage::gameplay::input::Input;
use mage::MageError;
use mage::rendering::Transform;

pub(crate) struct PlayerControls {
    pub(crate) player_velocity: Arc<AtomicU32>,
}

impl System for PlayerControls {
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
        for (_e, (_transform, input)) in world.query_mut::<(&mut Transform, &Input)>() {
            let mut velocity = 0f32;
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
                    } => {
                        velocity -= player_velocity;
                    }
                    Event::KeyDown {
                        keycode: Some(Keycode::D),
                        ..
                    }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Right),
                        ..
                    } => {
                        velocity += player_velocity;
                    }
                    _ => {}
                }
            }
            error!(
                "Player velocity {} {} {:?}",
                player_velocity, velocity, input.events
            );
        }
        Ok(())
    }

    fn late_update(&self, _world: &mut World, _delta_time: u64) -> Result<(), MageError> {
        Ok(())
    }
}
