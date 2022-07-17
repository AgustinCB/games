use std::cell::RefCell;
use std::sync::Arc;
use std::sync::atomic::{AtomicU8, Ordering};

use hecs::{Entity, World};
use nalgebra::Vector3;

use mage::core::system::System;
use mage::MageError;
use mage::physics::Velocity;
use mage::rendering::Transform;
use mage::resources::texture::TextureLoader;

use crate::{BouncingProperties, GameTextures, LevelElement};
use crate::bouncing_controls::BouncingStatus;
use crate::level::Level;

#[repr(u8)]
pub(crate) enum GameState {
    Active = 0u8,
    Menu,
    Win,
    Loose,
}

pub(crate) struct StartingProperties {
    pub(crate) position: Vector3<f32>,
    pub(crate) velocity: Vector3<f32>,
}

impl From<u8> for GameState {
    fn from(val: u8) -> GameState {
        match val {
            0 => GameState::Active,
            1 => GameState::Menu,
            2 => GameState::Win,
            3 => GameState::Loose,
            _ => unreachable!(),
        }
    }
}

fn load_starting_properties(world: &mut World) {
    for (_, (transform, velocity, starting_position)) in
    world.query_mut::<(&mut Transform, &mut Velocity, &StartingProperties)>()
    {
        transform.position = starting_position.position;
        velocity.0 = starting_position.velocity;
    }
    for (_, (props, starting_properties)) in
    world.query_mut::<(&mut BouncingProperties, &StartingProperties)>()
    {
        props.initial_velocity = starting_properties.velocity.xy();
        props.current_velocity = props.initial_velocity;
        props.status = BouncingStatus::Stuck;
    }
}

pub(crate) struct GameLogic {
    game_textures: GameTextures,
    _height: u32,
    level: RefCell<usize>,
    levels: Vec<Level>,
    state: Arc<AtomicU8>,
    texture_loader: Arc<TextureLoader>,
    _width: u32,
}
impl GameLogic {
    pub(crate) fn new(
        texture_loader: Arc<TextureLoader>,
        game_textures: GameTextures,
        width: u32,
        height: u32,
        state: Arc<AtomicU8>,
    ) -> Result<GameLogic, MageError> {
        Ok(GameLogic {
            level: RefCell::new(0),
            levels: vec![
                Level::new(
                    include_bytes!("../resources/level1").iter().cloned(),
                    texture_loader.clone(),
                    &game_textures,
                    height / 2,
                    width,
                )?,
                Level::new(
                    include_bytes!("../resources/level2").iter().cloned(),
                    texture_loader.clone(),
                    &game_textures,
                    height / 2,
                    width,
                )?,
                Level::new(
                    include_bytes!("../resources/level3").iter().cloned(),
                    texture_loader.clone(),
                    &game_textures,
                    height / 2,
                    width,
                )?,
                Level::new(
                    include_bytes!("../resources/level4").iter().cloned(),
                    texture_loader.clone(),
                    &game_textures,
                    height / 2,
                    width,
                )?,
                Level::new(
                    include_bytes!("../resources/level5").iter().cloned(),
                    texture_loader.clone(),
                    &game_textures,
                    height / 2,
                    width,
                )?,
            ],
            game_textures,
            state,
            texture_loader,
            _height: height,
            _width: width,
        })
    }

    fn load_level(&self, world: &mut World) {
        self.levels[*self.level.borrow()].load(world);
        load_starting_properties(world);
    }
}

impl System for GameLogic {
    fn name(&self) -> &str {
        "Game Logic"
    }

    fn start(&self, world: &mut World) -> Result<(), MageError> {
        self.texture_loader
            .load_texture_2d(&self.game_textures.background)?;
        self.texture_loader
            .load_texture_2d(&self.game_textures.ball)?;
        self.texture_loader
            .load_texture_2d(&self.game_textures.block_solid)?;
        self.texture_loader
            .load_texture_2d(&self.game_textures.blue_block)?;
        self.texture_loader
            .load_texture_2d(&self.game_textures.green_block)?;
        self.texture_loader
            .load_texture_2d(&self.game_textures.orange_block)?;
        self.texture_loader
            .load_texture_2d(&self.game_textures.white_block)?;
        self.texture_loader
            .load_texture_2d(&self.game_textures.yellow_block)?;
        self.load_level(world);
        Ok(())
    }

    fn early_update(&self, _: &mut World, _: u64) -> Result<(), MageError> {
        Ok(())
    }

    fn update(&self, _: &mut World, _: u64) -> Result<(), MageError> {
        Ok(())
    }

    fn late_update(&self, world: &mut World, _: u64) -> Result<(), MageError> {
        match GameState::from(self.state.load(Ordering::Relaxed)) {
            GameState::Loose => {
                #[allow(clippy::needless_collect)]
                    let bricks = world
                    .query_mut::<&LevelElement>()
                    .into_iter()
                    .filter(|(_, le)| le.is_block())
                    .map(|(e, _)| e)
                    .collect::<Vec<Entity>>();
                bricks.into_iter().for_each(|e| {
                    world.despawn(e).unwrap();
                });
                self.level.replace(0);
                self.load_level(world);
                self.state.store(GameState::Active as u8, Ordering::Relaxed);
            }
            GameState::Active => {
                if Level::is_complete(world) {
                    let level = *self.level.borrow();
                    self.level.replace((level + 1) % self.levels.len());
                    self.load_level(world);
                }
            }
            _ => {}
        }
        Ok(())
    }
}
