use crate::GameTextures;
use hecs::World;
use mage::rendering::model::cube::cuboid;
use mage::rendering::model::mesh::{RenderingMesh, TextureInfo};
use mage::rendering::Transform;
use mage::resources::texture::TextureLoader;
use mage::MageError;
use nalgebra::Vector3;
use std::sync::Arc;
use thiserror::Error;

#[derive(Debug, Error)]
enum LevelError {
    #[error("Invalid brick type {0}")]
    BrickParsingError(u8),
    #[error("Missing width in level definition")]
    MissingWidth,
    #[error("Missing height in level definition")]
    MissingHeight,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Brick {
    BlueBlock,
    Empty,
    GreenBlock,
    OrangeBlock,
    SolidBlock,
    WhiteBlock,
    YellowBlock,
}

impl TryFrom<u8> for Brick {
    type Error = LevelError;

    fn try_from(value: u8) -> Result<Brick, Self::Error> {
        match value {
            0 => Ok(Brick::Empty),
            1 => Ok(Brick::SolidBlock),
            2 => Ok(Brick::BlueBlock),
            3 => Ok(Brick::GreenBlock),
            4 => Ok(Brick::YellowBlock),
            5 => Ok(Brick::OrangeBlock),
            6 => Ok(Brick::WhiteBlock),
            _ => Err(LevelError::BrickParsingError(value)),
        }
    }
}

impl Brick {
    fn is_visible(&self) -> bool {
        self != &Brick::Empty
    }

    fn get_texture(&self, game_textures: &GameTextures) -> Vec<TextureInfo> {
        match self {
            Brick::BlueBlock => vec![game_textures.blue_block.clone()],
            Brick::Empty => vec![],
            Brick::GreenBlock => vec![game_textures.green_block.clone()],
            Brick::OrangeBlock => vec![game_textures.orange_block.clone()],
            Brick::SolidBlock => vec![game_textures.block_solid.clone()],
            Brick::WhiteBlock => vec![game_textures.white_block.clone()],
            Brick::YellowBlock => vec![game_textures.yellow_block.clone()],
        }
    }
}

pub(crate) struct Level {
    bricks: Vec<Vec<(Brick, RenderingMesh)>>,
    height: u32,
    unit_height: f32,
    unit_width: f32,
}

impl Level {
    pub(crate) fn new<I: Iterator<Item = u8>>(
        mut input: I,
        texture_loader: Arc<TextureLoader>,
        game_textures: &GameTextures,
        height: u32,
        width: u32,
    ) -> Result<Level, MageError> {
        let mut bricks = vec![];
        let rows = input.next().ok_or(LevelError::MissingWidth)?;
        let cols = input.next().ok_or(LevelError::MissingHeight)?;
        let unit_width = width as f32 / rows as f32;
        let unit_height = height as f32 / cols as f32;

        let mesh = cuboid(unit_width / 2.0, unit_height / 2.0, 0.1, vec![])
            .to_rendering_mesh(texture_loader.clone())?;
        let mut current_cell = 0u8;
        for raw_brick in input {
            if current_cell == 0 {
                bricks.push(vec![]);
            }
            let brick = Brick::try_from(raw_brick)?;
            bricks.last_mut().unwrap().push((
                brick,
                mesh.clone_with_textures(texture_loader.clone(), brick.get_texture(game_textures))?,
            ));
            current_cell = (current_cell + 1) % rows;
        }

        Ok(Level {
            bricks,
            unit_height,
            unit_width,
            height,
        })
    }

    pub(crate) fn load(&self, world: &mut World) {
        let mut entities = vec![];
        world
            .query::<&Brick>()
            .iter()
            .for_each(|(e, _)| entities.push(e));
        entities.into_iter().for_each(|e| world.despawn(e).unwrap());
        let height = self.height as f32 * 2.0;
        for (y, row) in self.bricks.iter().enumerate() {
            for (x, (brick, mesh)) in row.iter().enumerate() {
                if brick.is_visible() {
                    let mut transform = Transform::identity();
                    transform.position = Vector3::new(
                        self.unit_width * x as f32 + self.unit_width / 2.0,
                        height - self.unit_height * y as f32 - self.unit_height / 2.0,
                        0.0,
                    );
                    world.spawn((*brick, transform, mesh.clone()));
                }
            }
        }
    }
}
