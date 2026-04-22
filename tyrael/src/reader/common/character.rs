use std::slice::Iter;

use crate::character::{
    CharacterActiveWeaponSet,
    CharacterClass,
    CharacterMenuAppearance,
    CharacterProgression,
    CharacterSkillShortcuts,
    CharacterStatus,
};
use crate::errors::{CharacterNameError, ReadCharacterSaveError};
use crate::reader::common::ReaderExt;

pub fn read_name(data: [u8; 16]) -> Result<String, CharacterNameError> {
    let mut result = String::new();
    let mut null_present = false;
    let mut dash_or_underscore_present = false;

    for byte in data {
        let character = byte as char;

        if character == '\0' {
            null_present = true;
            continue;
        } else if null_present {
            return Err(CharacterNameError::NameContainsOtherCharacterAfterNull);
        }

        if character == '-' || character == '_' {
            if dash_or_underscore_present {
                return Err(CharacterNameError::NameContainsMultipleDashesOrUnderscores);
            }
            dash_or_underscore_present = true;
            result.push(character);
            continue;
        }

        if !character.is_ascii_alphabetic() {
            return Err(CharacterNameError::NameContainsInvalidCharacter(character));
        }

        result.push(character);
    }

    if result.len() < 2 {
        Err(CharacterNameError::NameTooShort)
    } else if result.len() > 15 {
        Err(CharacterNameError::NameTooLong)
    } else if result.starts_with('-') || result.starts_with('_') {
        Err(CharacterNameError::NameStartsWithDashOrUnderscore)
    } else if result.ends_with('-') || result.ends_with('_') {
        Err(CharacterNameError::NameEndsWithDashOrUnderscore)
    } else {
        Ok(result)
    }
}

pub fn read_class(data: u8) -> Result<CharacterClass, ReadCharacterSaveError> {
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

pub fn read_status(data: u8) -> Result<CharacterStatus, ReadCharacterSaveError> {
    Ok(CharacterStatus {
        hardcore: data & 0b0000_0100 > 0,
        dead: data & 0b0000_1000 > 0,
        expansion: data & 0b0010_0000 > 0,
    })
}

pub fn read_progression(
    data: u8,
    expansion: bool,
) -> Result<CharacterProgression, ReadCharacterSaveError> {
    Ok(match expansion {
        false => match data {
            0 => CharacterProgression::None,
            1 => CharacterProgression::NormalAndarielKilled,
            2 => CharacterProgression::NormalDurielKilled,
            3 => CharacterProgression::NormalMephistoKilled,
            4 => CharacterProgression::NormalFinalBossKilled,
            5 => CharacterProgression::NightmareAndarielKilled,
            6 => CharacterProgression::NightmareDurielKilled,
            7 => CharacterProgression::NightmareMephistoKilled,
            8 => CharacterProgression::NightmareFinalBossKilled,
            9 => CharacterProgression::HellAndarielKilled,
            10 => CharacterProgression::HellDurielKilled,
            11 => CharacterProgression::HellMephistoKilled,
            12 => CharacterProgression::HellFinalBossKilled,
            _ => {
                return Err(ReadCharacterSaveError::InvalidCharacterProgression {
                    expansion,
                    progression: data,
                });
            }
        },
        true => match data {
            0 => CharacterProgression::None,
            1 => CharacterProgression::NormalAndarielKilled,
            2 => CharacterProgression::NormalDurielKilled,
            3 => CharacterProgression::NormalMephistoKilled,
            5 => CharacterProgression::NormalFinalBossKilled,
            6 => CharacterProgression::NightmareAndarielKilled,
            7 => CharacterProgression::NightmareDurielKilled,
            8 => CharacterProgression::NightmareMephistoKilled,
            10 => CharacterProgression::NightmareFinalBossKilled,
            11 => CharacterProgression::HellAndarielKilled,
            12 => CharacterProgression::HellDurielKilled,
            13 => CharacterProgression::HellMephistoKilled,
            15 => CharacterProgression::HellFinalBossKilled,
            _ => {
                return Err(ReadCharacterSaveError::InvalidCharacterProgression {
                    expansion,
                    progression: data,
                });
            }
        },
    })
}

pub fn read_active_weapon_set(
    data: u8,
) -> Result<CharacterActiveWeaponSet, ReadCharacterSaveError> {
    match data {
        0 => Ok(CharacterActiveWeaponSet::WeaponSet1),
        1 => Ok(CharacterActiveWeaponSet::WeaponSet2),
        _ => Err(ReadCharacterSaveError::InvalidCharacterWeaponSet(data)),
    }
}

pub fn read_menu_appearance(
    data: [u8; 32],
) -> Result<CharacterMenuAppearance, ReadCharacterSaveError> {
    Ok(CharacterMenuAppearance(data)) // TODO
}

pub fn read_skill_shortcuts_short(
    data: &mut Iter<u8>,
) -> Result<CharacterSkillShortcuts, ReadCharacterSaveError> {
    Ok(CharacterSkillShortcuts::Short {
        keyboard: [
            data.read_u16()?,
            data.read_u16()?,
            data.read_u16()?,
            data.read_u16()?,
            data.read_u16()?,
            data.read_u16()?,
            data.read_u16()?,
            data.read_u16()?,
        ],
        lmb: data.read_u8()?,
        rmb: data.read_u8()?,
    })
}

pub fn read_skill_shortcuts_long(
    data: &mut Iter<u8>,
) -> Result<CharacterSkillShortcuts, ReadCharacterSaveError> {
    Ok(CharacterSkillShortcuts::Long {
        keyboard: [
            data.read_u32()?,
            data.read_u32()?,
            data.read_u32()?,
            data.read_u32()?,
            data.read_u32()?,
            data.read_u32()?,
            data.read_u32()?,
            data.read_u32()?,
            data.read_u32()?,
            data.read_u32()?,
            data.read_u32()?,
            data.read_u32()?,
            data.read_u32()?,
            data.read_u32()?,
            data.read_u32()?,
            data.read_u32()?,
        ],
        lmb: data.read_u32()?,
        rmb: data.read_u32()?,
        lmb_switch: data.read_u32()?,
        rmb_switch: data.read_u32()?,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_ok_on_valid_names() {
        for name in ["XX", "GigaChadPaladin", "Mule_Endgame", "ber-stash"] {
            let len = name.len().clamp(0, 16);
            let mut name_data = [0u8; 16];
            name_data[..len].copy_from_slice(&name.as_bytes()[..len]);

            assert_eq!(name, read_name(name_data).unwrap())
        }
    }

    #[test]
    fn returns_err_on_invalid_names() {
        for (name, error) in [
            ("", CharacterNameError::NameTooShort),
            ("A", CharacterNameError::NameTooShort),
            ("VeryLongBarbName", CharacterNameError::NameTooLong),
            ("VeryLongBarbarianName", CharacterNameError::NameTooLong),
            (
                "AndyDestroyer69",
                CharacterNameError::NameContainsInvalidCharacter('6'),
            ),
            (
                "_DruidBuild",
                CharacterNameError::NameStartsWithDashOrUnderscore,
            ),
            (
                "-Singer",
                CharacterNameError::NameStartsWithDashOrUnderscore,
            ),
            (
                "JavazonX_",
                CharacterNameError::NameEndsWithDashOrUnderscore,
            ),
            (
                "Hammerdin-",
                CharacterNameError::NameEndsWithDashOrUnderscore,
            ),
            (
                "Giga-Pala-Build",
                CharacterNameError::NameContainsMultipleDashesOrUnderscores,
            ),
            (
                "Giga_Barb_Build",
                CharacterNameError::NameContainsMultipleDashesOrUnderscores,
            ),
            (
                "Giga-Sorc_Build",
                CharacterNameError::NameContainsMultipleDashesOrUnderscores,
            ),
            (
                "GigaBroken\0Name",
                CharacterNameError::NameContainsOtherCharacterAfterNull,
            ),
        ] {
            let len = name.len().clamp(0, 16);
            let mut name_data = [0u8; 16];
            name_data[..len].copy_from_slice(&name.as_bytes()[..len]);

            assert_eq!(error, read_name(name_data).unwrap_err())
        }
    }

    #[test]
    fn returns_ok_on_valid_class() {
        for (d, class) in [
            (0, CharacterClass::Amazon),
            (1, CharacterClass::Sorceress),
            (2, CharacterClass::Necromancer),
            (3, CharacterClass::Paladin),
            (4, CharacterClass::Barbarian),
            (5, CharacterClass::Druid),
            (6, CharacterClass::Assassin),
        ] {
            assert_eq!(Ok(class), read_class(d));
        }
    }

    #[test]
    fn returns_err_on_invalid_class() {
        for d in 7..u8::MAX {
            assert_eq!(
                Err(ReadCharacterSaveError::InvalidCharacterClass(d)),
                read_class(d)
            )
        }
    }

    #[test]
    fn returns_ok_on_valid_progression() {
        for (d, expansion, progression) in [
            (0, false, CharacterProgression::None),
            (1, false, CharacterProgression::NormalAndarielKilled),
            (2, false, CharacterProgression::NormalDurielKilled),
            (3, false, CharacterProgression::NormalMephistoKilled),
            (4, false, CharacterProgression::NormalFinalBossKilled),
            (5, false, CharacterProgression::NightmareAndarielKilled),
            (6, false, CharacterProgression::NightmareDurielKilled),
            (7, false, CharacterProgression::NightmareMephistoKilled),
            (8, false, CharacterProgression::NightmareFinalBossKilled),
            (9, false, CharacterProgression::HellAndarielKilled),
            (10, false, CharacterProgression::HellDurielKilled),
            (11, false, CharacterProgression::HellMephistoKilled),
            (12, false, CharacterProgression::HellFinalBossKilled),
            (0, true, CharacterProgression::None),
            (1, true, CharacterProgression::NormalAndarielKilled),
            (2, true, CharacterProgression::NormalDurielKilled),
            (3, true, CharacterProgression::NormalMephistoKilled),
            (5, true, CharacterProgression::NormalFinalBossKilled),
            (6, true, CharacterProgression::NightmareAndarielKilled),
            (7, true, CharacterProgression::NightmareDurielKilled),
            (8, true, CharacterProgression::NightmareMephistoKilled),
            (10, true, CharacterProgression::NightmareFinalBossKilled),
            (11, true, CharacterProgression::HellAndarielKilled),
            (12, true, CharacterProgression::HellDurielKilled),
            (13, true, CharacterProgression::HellMephistoKilled),
            (15, true, CharacterProgression::HellFinalBossKilled),
        ] {
            assert_eq!(
                Ok(progression),
                read_progression(d, expansion),
                "data={}, expansion={}",
                d,
                expansion
            );
        }
    }

    #[test]
    fn returns_err_on_invalid_progression() {
        for d in [4, 9, 14] {
            assert_eq!(
                Err(ReadCharacterSaveError::InvalidCharacterProgression {
                    expansion: true,
                    progression: d
                }),
                read_progression(d, true)
            )
        }

        for d in 13..u8::MAX {
            assert_eq!(
                Err(ReadCharacterSaveError::InvalidCharacterProgression {
                    expansion: false,
                    progression: d
                }),
                read_progression(d, false)
            )
        }

        for d in 16..u8::MAX {
            assert_eq!(
                Err(ReadCharacterSaveError::InvalidCharacterProgression {
                    expansion: true,
                    progression: d
                }),
                read_progression(d, true)
            )
        }
    }

    #[test]
    fn returns_ok_on_valid_active_weapon_set() {
        for (d, set) in [
            (0, CharacterActiveWeaponSet::WeaponSet1),
            (1, CharacterActiveWeaponSet::WeaponSet2),
        ] {
            assert_eq!(Ok(set), read_active_weapon_set(d));
        }
    }

    #[test]
    fn returns_err_on_invalid_active_weapon_set() {
        for d in 2..u8::MAX {
            assert_eq!(
                Err(ReadCharacterSaveError::InvalidCharacterWeaponSet(d)),
                read_active_weapon_set(d)
            )
        }
    }
}
