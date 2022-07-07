use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

use hecs::{Component, DynamicBundle, Entity};
use rapier3d::dynamics::RigidBody;
use rapier3d::geometry::Collider;
pub use rapier3d::pipeline::{EventHandler, PhysicsHooks};
use sdl2::keyboard::Keycode;

use crate::core::system::System;
use crate::core::window::Window;
use crate::core::world::World;
use crate::gameplay::input::{Input, InputSystem, InputType};
use crate::gameplay::quit::{QuitControl, QuitSystem};
use crate::MageError;
use crate::rendering::engine::Engine;

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
}

impl GameBuilder {
    pub fn build<N: Engine>(self, engine: N) -> Game<N> {
        Game {
            engine,
            frame_rate: 1000 / 60, // 60 frames per second
            game_ended: self.game_ended,
            window: self.window,
            world: self.world,
        }
    }
}

pub struct Game<N: Engine> {
    engine: N,
    frame_rate: u64,
    game_ended: Arc<AtomicBool>,
    window: Window,
    world: World,
}

impl<N: Engine> Game<N> {
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

    pub fn add_collider(&mut self, entity: Entity, collider: Collider) {
        self.world.physics_engine.add_collider(entity, collider);
    }

    pub fn add_rigidbody(&mut self, entity: Entity, rigidbody: RigidBody) {
        self.world.physics_engine.add_rigidbody(entity, rigidbody);
    }

    pub fn add_collider_and_rigidbody(
        &mut self,
        entity: Entity,
        collider: Collider,
        rigidbody: RigidBody,
    ) {
        self.world
            .physics_engine
            .add_collider_and_rigidbody(entity, collider, rigidbody);
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
        let mut lag = 0;
        while !self.game_ended.load(Ordering::Relaxed) {
            let delta_time = self.window.delta_time();
            lag += delta_time;

            self.world.early_update(delta_time);

            while lag >= self.frame_rate {
                self.world.update(delta_time);
                lag -= self.frame_rate;
            }

            self.world.late_update(delta_time);
            self.engine.render(
                &mut self.world.world,
                delta_time as f32 / self.frame_rate as f32,
            )?;

            self.window.swap_buffers();
        }
        Ok(())
    }
}
