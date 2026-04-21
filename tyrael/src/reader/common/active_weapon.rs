use std::slice::Iter;

use crate::character::CharacterActiveWeaponSet;
use crate::errors::ReadCharacterSaveError;
use crate::reader::common::ReaderExt;

pub fn read_character_active_weapon_u16(
    data: &mut Iter<u8>,
) -> Result<CharacterActiveWeaponSet, ReadCharacterSaveError> {
    let set = data.read_u16()?;
    match set {
        0 => Ok(CharacterActiveWeaponSet::WeaponSet1),
        1 => Ok(CharacterActiveWeaponSet::WeaponSet1),
        _ => Err(ReadCharacterSaveError::InvalidCharacterWeaponSet(
            set as u32,
        )),
    }
}

pub fn read_character_active_weapon_u32(
    data: &mut Iter<u8>,
) -> Result<CharacterActiveWeaponSet, ReadCharacterSaveError> {
    let set = data.read_u32()?;
    match set {
        0 => Ok(CharacterActiveWeaponSet::WeaponSet1),
        1 => Ok(CharacterActiveWeaponSet::WeaponSet1),
        _ => Err(ReadCharacterSaveError::InvalidCharacterWeaponSet(set)),
    }
}
