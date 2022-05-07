use crate::core::system::System;
use crate::core::window::Window;
use crate::core::world::World;
use crate::gameplay::input::{Input, InputSystem, InputType};
use crate::gameplay::quit::{QuitControl, QuitSystem};
use crate::rendering::engine::Engine;
use crate::MageError;
use hecs::{Component, DynamicBundle, Entity};
use sdl2::keyboard::Keycode;
use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

pub struct GameBuilder {
    game_ended: Arc<AtomicBool>,
    window: Window,
    world: World,
}

impl GameBuilder {
    pub fn new(name: &str, width: u32, height: u32) -> Result<GameBuilder, MageError> {
        let world = World::new();
        let window = Window::new(name, width, height)?;
        Ok(GameBuilder {
            game_ended: Arc::new(AtomicBool::new(false)),
            window,
            world,
        })
    }

    pub fn build<E: Engine>(self, engine: E) -> Game<E> {
        Game {
            engine,
            game_ended: self.game_ended,
            window: self.window,
            world: self.world,
        }
    }
}

pub struct Game<E: Engine> {
    engine: E,
    game_ended: Arc<AtomicBool>,
    window: Window,
    world: World,
}

impl<E: Engine> Game<E> {
    pub fn spawn(&mut self, components: impl DynamicBundle) -> Entity {
        self.world.get_mut().spawn(components)
    }

    pub fn add_to(&mut self, entity: Entity, component: impl Component) -> Result<(), MageError> {
        self.world
            .get_mut()
            .insert_one(entity, component)
            .map_err(Box::new)?;
        Ok(())
    }

    pub fn play(&mut self, systems: Vec<Box<dyn System>>) -> Result<(), MageError> {
        self.spawn((
            Input::new(vec![InputType::Quit, InputType::Keyboard]),
            QuitControl {
                quit_keycode: Keycode::Escape,
            },
        ));
        self.world.add_system(Box::new(InputSystem {
            event_pumper: RefCell::new(self.window.get_pumper()),
            pressed_down: RefCell::new(HashMap::new()),
        }));
        self.world.add_system(Box::new(QuitSystem {
            game_ended: self.game_ended.clone(),
        }));
        for system in systems {
            self.world.add_system(system);
        }

        self.window.start_timer();
        self.engine.setup(&mut self.world.world)?;
        self.world.start();
        while !self.game_ended.load(Ordering::Relaxed) {
            let delta_time = self.window.delta_time();

            self.world.early_update(delta_time);
            self.world.update(delta_time);
            self.world.late_update(delta_time);
            self.engine.render(&mut self.world.world, delta_time)?;

            self.window.swap_buffers();
        }
        Ok(())
    }
}
