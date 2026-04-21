use std::slice::Iter;

use crate::character::CharacterStatus;
use crate::errors::ReadCharacterSaveError;
use crate::reader::common::ReaderExt;

pub fn read_character_status(
    data: &mut Iter<u8>,
) -> Result<CharacterStatus, ReadCharacterSaveError> {
    let data = data.read_u8()?;

    Ok(CharacterStatus {
        hardcore: data & 0b0000_0100 > 0,
        dead: data & 0b0000_1000 > 0,
        expansion: data & 0b0010_0000 > 0,
    })
}
