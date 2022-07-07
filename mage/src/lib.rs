#![feature(let_chains)]
#![feature(negative_impls)]

use std::error::Error;

pub type MageError = Box<dyn Error>;

pub mod core;
pub mod gameplay;
pub mod physics;
pub mod rendering;
pub mod resources;
