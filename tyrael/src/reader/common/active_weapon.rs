use crate::character::CharacterActiveWeaponSet;
use crate::errors::ReadCharacterSaveError;

pub fn read_character_active_weapon(
    data: u8,
) -> Result<CharacterActiveWeaponSet, ReadCharacterSaveError> {
    match data {
        0 => Ok(CharacterActiveWeaponSet::WeaponSet1),
        1 => Ok(CharacterActiveWeaponSet::WeaponSet2),
        _ => Err(ReadCharacterSaveError::InvalidCharacterWeaponSet(data)),
    }
}
