use std::slice::Iter;

use crate::character::CharacterClass;
use crate::errors::ReadCharacterSaveError;
use crate::reader::common::ReaderExt;

pub fn read_character_class(data: &mut Iter<u8>) -> Result<CharacterClass, ReadCharacterSaveError> {
    let data = data.read_u8()?;
    match data {
        0 => Ok(CharacterClass::Amazon),
        1 => Ok(CharacterClass::Sorceress),
        2 => Ok(CharacterClass::Necromancer),
        3 => Ok(CharacterClass::Paladin),
        4 => Ok(CharacterClass::Barbarian),
        5 => Ok(CharacterClass::Druid),
        6 => Ok(CharacterClass::Assassin),
        // TODO: warlock
        _ => Err(ReadCharacterSaveError::InvalidCharacterClass(data)),
    }
}
