#![feature(iter_next_chunk)]

use crate::character::CharacterData;
use crate::errors::{ReadCharacterSaveError, WriteCharacterSaveError};
use crate::location::LocationData;
use crate::mercenary::MercenaryData;

pub mod character;
pub mod errors;
pub mod location;
pub mod mercenary;

mod reader;
mod writer;

#[derive(Debug, PartialEq, Clone)]
pub struct CharacterSave {
    pub version: u32,
    pub character: CharacterData,
    pub location: LocationData,
    pub mercenary: Option<MercenaryData>,
}

impl CharacterSave {
    pub fn read(bytes: &[u8]) -> Result<Self, ReadCharacterSaveError> {
        reader::read_character_save(&mut bytes.iter())
    }

    pub fn write(&self) -> Result<Vec<u8>, WriteCharacterSaveError> {
        writer::write_character_save(self)
    }
}
