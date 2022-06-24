use crate::level::Level;
use crate::GameTextures;
use hecs::World;
use mage::core::system::System;
use mage::resources::texture::TextureLoader;
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
    game_textures: GameTextures,
    _height: u32,
    level: usize,
    levels: Vec<Level>,
    _state: Arc<AtomicU32>,
    texture_loader: Arc<TextureLoader>,
    _width: u32,
}
impl GameLogic {
    pub(crate) fn new(
        texture_loader: Arc<TextureLoader>,
        game_textures: GameTextures,
        height: u32,
        width: u32,
    ) -> Result<GameLogic, MageError> {
        Ok(GameLogic {
            level: 0,
            levels: vec![Level::new(
                include_bytes!("../resources/level1").iter().cloned(),
                texture_loader.clone(),
                &game_textures,
                height,
                width,
            )?],
            _state: Arc::new(AtomicU32::new(GameState::Active as _)),
            game_textures,
            texture_loader,
            _height: height,
            _width: width,
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

    fn start(&self, world: &mut hecs::World) -> Result<(), MageError> {
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
