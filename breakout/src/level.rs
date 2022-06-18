use hecs::World;
use mage::rendering::Transform;
use mage::MageError;
use thiserror::Error;

#[derive(Debug, Error)]
enum LevelError {
    #[error("Invalid brick type {0}")]
    BrickParsingError(u8),
    #[error("Missing width in level definition")]
    MissingWidth,
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
            1 => Ok(Brick::WhiteBlock),
            2 => Ok(Brick::SolidBlock),
            3 => Ok(Brick::BlueBlock),
            4 => Ok(Brick::GreenBlock),
            5 => Ok(Brick::YellowBlock),
            6 => Ok(Brick::OrangeBlock),
            _ => Err(LevelError::BrickParsingError(value)),
        }
    }
}

impl Brick {
    fn is_visible(&self) -> bool {
        self != &Brick::Empty
    }
}

pub(crate) struct Level {
    bricks: Vec<Vec<Brick>>,
}

impl Level {
    pub(crate) fn new<I: Iterator<Item = u8>>(mut input: I) -> Result<Level, MageError> {
        let mut bricks = vec![];
        let width = input.next().ok_or(LevelError::MissingWidth)?;

        let mut current_cell = 0u8;
        for raw_brick in input {
            if current_cell == 0 {
                bricks.push(vec![]);
            }
            bricks.last_mut().unwrap().push(Brick::try_from(raw_brick)?);
            current_cell = (current_cell + 1) % width;
        }

        Ok(Level { bricks })
    }

    pub(crate) fn load(&self, world: &mut World) {
        let mut entities = vec![];
        world
            .query::<&Brick>()
            .iter()
            .for_each(|(e, _)| entities.push(e));
        entities.into_iter().for_each(|e| world.despawn(e).unwrap());
        for row in &self.bricks {
            for brick in row {
                if brick.is_visible() {
                    let transform = Transform::identity();
                    world.spawn((*brick, transform));
                }
            }
        }
    }
}
