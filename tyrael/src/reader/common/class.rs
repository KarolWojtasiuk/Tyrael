use crate::character::CharacterClass;
use crate::errors::ReadCharacterSaveError;

pub fn read_character_class(data: u8) -> Result<CharacterClass, ReadCharacterSaveError> {
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
