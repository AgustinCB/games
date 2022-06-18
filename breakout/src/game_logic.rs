use crate::level::Level;
use crate::GameTextures;
use hecs::World;
use mage::core::system::System;
use mage::MageError;
use std::sync::atomic::AtomicU32;
use std::sync::Arc;

#[repr(u32)]
enum GameState {
    Active = 0u32,
    Menu,
    Win,
    Loose,
}

impl From<u32> for GameState {
    fn from(val: u32) -> GameState {
        match val {
            0 => GameState::Active,
            1 => GameState::Menu,
            2 => GameState::Win,
            3 => GameState::Loose,
            _ => unreachable!(),
        }
    }
}

pub(crate) struct GameLogic {
    _game_textures: GameTextures,
    _height: u32,
    level: usize,
    levels: Vec<Level>,
    _state: Arc<AtomicU32>,
    _width: u32,
}
impl GameLogic {
    pub(crate) fn new(
        game_textures: GameTextures,
        height: u32,
        width: u32,
    ) -> Result<GameLogic, MageError> {
        Ok(GameLogic {
            _game_textures: game_textures,
            _height: height,
            _width: width,
            level: 0,
            levels: vec![Level::new(
                include_bytes!("../resources/level1").iter().cloned(),
            )?],
            _state: Arc::new(AtomicU32::new(GameState::Active as _)),
        })
    }

    fn load_level(&self, world: &mut World) {
        self.levels[self.level].load(world);
    }
}

impl System for GameLogic {
    fn name(&self) -> &str {
        "Game Logic"
    }

    fn start(&self, world: &mut hecs::World) -> Result<(), Box<(dyn std::error::Error + 'static)>> {
        self.load_level(world);
        Ok(())
    }

    fn early_update(
        &self,
        _: &mut hecs::World,
        _: u64,
    ) -> Result<(), Box<(dyn std::error::Error + 'static)>> {
        todo!()
    }

    fn update(
        &self,
        _: &mut hecs::World,
        _: u64,
    ) -> Result<(), Box<(dyn std::error::Error + 'static)>> {
        todo!()
    }

    fn late_update(
        &self,
        _: &mut hecs::World,
        _: u64,
    ) -> Result<(), Box<(dyn std::error::Error + 'static)>> {
        Ok(())
    }
}
