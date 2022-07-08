use std::sync::Arc;
use std::sync::atomic::{AtomicU8, Ordering};

use hecs::World;
use nalgebra::{Vector2, Vector3};

use mage::core::system::System;
use mage::MageError;
use mage::physics::{Collisions, Velocity};

use crate::game_logic::GameState;
use crate::LevelElement;

#[derive(Clone, Debug)]
pub(crate) struct BouncingProperties {
    pub(crate) velocity: Vector2<f32>,
}

pub(crate) struct BouncingControlsSystem {
    pub(crate) game_state: Arc<AtomicU8>,
}

impl System for BouncingControlsSystem {
    fn name(&self) -> &str {
        "Bouncing Controls"
    }

    fn start(&self, _world: &mut World) -> Result<(), MageError> {
        Ok(())
    }

    fn early_update(&self, _world: &mut World, _delta_time: u64) -> Result<(), MageError> {
        Ok(())
    }

    fn update(&self, world: &mut World, delta_time: u64) -> Result<(), MageError> {
        let mut work_queue = vec![];
        for (e, _props) in world.query::<&BouncingProperties>().iter() {
            let (mut x, mut y) = (1.0, 1.0);
            if let Some(collisions) = world.query_one::<&Collisions>(e)
                .map(|mut q| q.get().cloned())
                .ok()
                .flatten() {
                for collision in collisions.0 {
                    if !collision.started {
                        continue;
                    }
                    if let Ok(mut result) = world.query_one::<&LevelElement>(collision.entity_id) {
                        let result = result.get();
                        match result {
                            Some(LevelElement::RightWall) => {
                                x *= -1.0;
                            },
                            Some(LevelElement::LeftWall) => {
                                x *= -1.0;
                            },
                            Some(LevelElement::TopWall) => {
                                y *= -1.0;
                            },
                            Some(LevelElement::Player) => {
                                y *= -1.0;
                            }
                            Some(LevelElement::BottomWall) => {
                                self.game_state.store(GameState::Loose as u8, Ordering::Relaxed);
                                return Ok(());
                            }
                            _ => {}
                        }
                    }
                }
            }
            work_queue.push((e, x, y));
        }
        let mut velocities = vec![];
        let multiplier = (delta_time as f32 / 1000.0).clamp(0.0, 0.03);
        for (e, x, y) in work_queue {
            if let Ok(props) = world.query_one_mut::<&mut BouncingProperties>(e) {
                props.velocity.x *= x;
                props.velocity.y *= y;
                let velocity = Vector3::new(
                    props.velocity.x * multiplier,
                    props.velocity.y * multiplier,
                    0.0,
                );
                velocities.push((e, velocity));
            }
        }
        for (e, velocity) in velocities {
            world.insert_one(e, Velocity(velocity))?;
        }
        Ok(())
    }

    fn late_update(&self, _world: &mut World, _delta_time: u64) -> Result<(), MageError> {
        Ok(())
    }
}
