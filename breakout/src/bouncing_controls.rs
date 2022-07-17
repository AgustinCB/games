use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicU8, Ordering};

use hecs::{Entity, World};
use nalgebra::{Vector2, Vector3};
use thiserror::Error;

use mage::core::system::System;
use mage::gameplay::input::Input;
use mage::MageError;
use mage::physics::{Collision, Collisions, Triggers, Velocity};

use crate::game_logic::GameState;
use crate::LevelElement;

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum BouncingStatus {
    Normal,
    Stuck,
}

#[derive(Clone, Debug)]
pub(crate) struct BouncingProperties {
    pub(crate) initial_velocity: Vector2<f32>,
    pub(crate) current_velocity: Vector2<f32>,
    pub(crate) max_distance: f32,
    pub(crate) status: BouncingStatus,
}

pub(crate) struct BouncingControlsSystem {
    pub(crate) game_state: Arc<AtomicU8>,
    pub(crate) unstick: Arc<AtomicBool>,
}

#[derive(Debug, Error)]
enum BouncingError {
    #[error("Missing player")]
    NoPlayer,
}

fn get_player_velocity(world: &mut World) -> Result<Vector3<f32>, MageError> {
    let result = world.query_mut::<&Velocity>().with::<Input>().into_iter().next();
    result.ok_or_else(|| BouncingError::NoPlayer.into())
        .map(|(_, v)| v.0)
}

fn handle_ball_bounces(delta_time: u64, broken_blocks: &mut Vec<Entity>, collisions: &&Collisions, props: &mut BouncingProperties, velocity: &mut Velocity) {
    let (mut x, mut y) = (false, false);
    for collision in &collisions.0 {
        if let Collision::Started(entity_id, contact_pair, user_data) = collision {
            match LevelElement::from(*user_data as u8) {
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
                    if let Some((_, contact)) = contact_pair.find_deepest_contact() {
                        let multiplier = contact.local_p1.x / props.max_distance * 2.0;
                        let old_velocity = props.current_velocity;
                        props.current_velocity.y *= -1.0;
                        props.current_velocity.x = props.initial_velocity.x * multiplier;
                        props.current_velocity =
                            props.current_velocity.normalize() * old_velocity.norm();
                    }
                }
                element @ (LevelElement::Block | LevelElement::SolidBlock) => {
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
                _ => {}
            }
        }
    }
    props.current_velocity.x *= x.then_some(-1.0).unwrap_or(1.0);
    props.current_velocity.y *= y.then_some(-1.0).unwrap_or(1.0);
    velocity.0 = Vector3::new(props.current_velocity.x, props.current_velocity.y, 0.0)
        * delta_time as f32;
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
        self.check_triggers(world);
        self.check_collisions(world, delta_time)?;
        Ok(())
    }

    fn late_update(&self, _world: &mut World, _delta_time: u64) -> Result<(), MageError> {
        Ok(())
    }
}

impl BouncingControlsSystem {
    fn check_triggers(&self, world: &mut World) {
        for (_, triggers) in world.query_mut::<&Triggers>().with::<BouncingProperties>() {
            for collision in &triggers.0 {
                if let Collision::StartedTrigger(_, user_data) = collision {
                    if let LevelElement::BottomWall = LevelElement::from(*user_data as u8) {
                        self.game_state
                            .store(GameState::Loose as u8, Ordering::Relaxed);
                        return;
                    }
                }
            }
        }
    }

    fn check_collisions(&self, world: &mut World, delta_time: u64) -> Result<(), MageError> {
        let mut broken_blocks = vec![];
        let player_velocity = get_player_velocity(world)?;
        for (_, (collisions, props, velocity)) in
            world.query_mut::<(&Collisions, &mut BouncingProperties, &mut Velocity)>()
        {
            if self.unstick.load(Ordering::Relaxed) {
                props.status = BouncingStatus::Normal;
                self.unstick.store(false, Ordering::Relaxed);
            }
            if props.status == BouncingStatus::Stuck {
                velocity.0 = player_velocity;
            } else {
                handle_ball_bounces(delta_time, &mut broken_blocks, &collisions, props, velocity);
            }
        }
        for broken_block in broken_blocks {
            world.despawn(broken_block)?;
        }
        Ok(())
    }
}
