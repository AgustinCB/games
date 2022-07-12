use std::sync::Arc;
use std::sync::atomic::{AtomicU8, Ordering};

use hecs::World;
use nalgebra::{Vector2, Vector3};

use mage::core::system::System;
use mage::MageError;
use mage::physics::{Collision, Collisions, Velocity};

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
        let mut broken_blocks = vec![];
        let multiplier = delta_time as f32 / 1000.0;
        for (_, (collisions, props, velocity)) in
        world.query_mut::<(&Collisions, &mut BouncingProperties, &mut Velocity)>()
        {
            let (mut x, mut y) = (false, false);
            for collision in &collisions.0 {
                if let Collision::Started(entity_id, contact_pair, user_data) = collision {
                    let element = LevelElement::from(*user_data as u8);
                    match element {
                        LevelElement::RightWall => {
                            x = true;
                        }
                        LevelElement::LeftWall => {
                            x = true;
                        }
                        LevelElement::TopWall => {
                            y = true;
                        }
                        LevelElement::Player => {
                            y = true;
                        }
                        LevelElement::Block | LevelElement::SolidBlock => {
                            if let Some((contact, _)) = contact_pair.find_deepest_contact() {
                                if contact.local_n1.x.abs() > contact.local_n1.y.abs() {
                                    x = true;
                                } else {
                                    y = true;
                                }
                                if LevelElement::Block == element {
                                    broken_blocks.push(*entity_id);
                                }
                            }
                        }
                        LevelElement::BottomWall => {
                            self.game_state
                                .store(GameState::Loose as u8, Ordering::Relaxed);
                            return Ok(());
                        }
                        _ => {}
                    }
                }
            }
            props.velocity.x *= x.then_some(-1.0).unwrap_or(1.0);
            props.velocity.y *= y.then_some(-1.0).unwrap_or(1.0);
            velocity.0 = Vector3::new(
                props.velocity.x * multiplier,
                props.velocity.y * multiplier,
                0.0,
            );
        }
        for broken_block in broken_blocks {
            world.despawn(broken_block)?;
        }
        Ok(())
    }

    fn late_update(&self, _world: &mut World, _delta_time: u64) -> Result<(), MageError> {
        Ok(())
    }
}
