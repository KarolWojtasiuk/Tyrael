#![feature(iter_next_chunk)]

use character::CharacterInfo;
use errors::ReadCharacterSaveError;

use crate::errors::WriteCharacterSaveError;

pub mod character;
pub mod errors;

mod reader;
mod writer;

pub struct CharacterSave {
    pub version: u32,
    pub character: CharacterInfo,
}

impl CharacterSave {
    pub fn read(bytes: &[u8]) -> Result<Self, ReadCharacterSaveError> {
        reader::read_character_save(&mut bytes.iter())
    }

    pub fn write(&self) -> Result<Vec<u8>, WriteCharacterSaveError> {
        writer::write_character_save(self)
    }
}
