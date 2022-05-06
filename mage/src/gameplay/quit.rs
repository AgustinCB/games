use crate::core::system::System;
use crate::gameplay::input::Input;
use crate::MageError;
use hecs::World;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct QuitControl {
    pub quit_keycode: Keycode,
}

pub struct QuitSystem {
    pub game_ended: Arc<AtomicBool>,
}

impl System for QuitSystem {
    fn name(&self) -> &str {
        "Quit"
    }

    fn start(&self, _world: &mut World) -> Result<(), MageError> {
        Ok(())
    }

    fn early_update(&self, world: &mut World, _delta_time: u64) -> Result<(), MageError> {
        for (_e, (input, quit_control)) in world.query_mut::<(&Input, &QuitControl)>() {
            for event in input.events.iter() {
                match event {
                    Event::Quit { .. } => {
                        self.game_ended.store(true, Ordering::Relaxed);
                    }
                    Event::KeyDown {
                        keycode: Some(k), ..
                    } if *k == quit_control.quit_keycode => {
                        self.game_ended.store(true, Ordering::Relaxed);
                    }
                    _ => {}
                }
            }
        }
        Ok(())
    }

    fn update(&self, _world: &mut World, _delta_time: u64) -> Result<(), MageError> {
        Ok(())
    }

    fn late_update(&self, _world: &mut World, _delta_time: u64) -> Result<(), MageError> {
        Ok(())
    }
}
