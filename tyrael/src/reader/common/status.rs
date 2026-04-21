use crate::character::CharacterStatus;
use crate::errors::ReadCharacterSaveError;

pub fn read_character_status(data: u8) -> Result<CharacterStatus, ReadCharacterSaveError> {
    Ok(CharacterStatus {
        hardcore: data & 0b0000_0100 > 0,
        dead: data & 0b0000_1000 > 0,
        expansion: data & 0b0010_0000 > 0,
    })
}
