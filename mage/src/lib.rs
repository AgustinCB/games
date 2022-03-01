use std::error::Error;

pub type MageError = Box<dyn Error>;

pub mod rendering;
