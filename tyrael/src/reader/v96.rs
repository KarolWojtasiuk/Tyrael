use std::slice::Iter;

use super::common::ReaderExt;
use crate::CharacterSave;
use crate::errors::ReadCharacterSaveError;

pub fn read_character_save(data: &mut Iter<u8>) -> Result<CharacterSave, ReadCharacterSaveError> {
    let _filesize = data.read_u32()?;
    let _checksum = data.read_u32()?;

    Ok(CharacterSave {
        version: 96,
        character: super::v92_v96::read_character_info(data)?,
    })
}
