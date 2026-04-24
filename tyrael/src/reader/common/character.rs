use crate::character::{
    CharacterActiveWeaponSet,
    CharacterClass,
    CharacterMenuAppearance,
    CharacterProgression,
    CharacterSkillShortcuts,
    CharacterStatus,
};
use crate::errors::{CharacterDataError, ReadCharacterSaveError};

pub fn read_name(data: [u8; 16]) -> Result<String, ReadCharacterSaveError> {
    let mut result = String::new();
    let mut null_present = false;
    let mut dash_or_underscore_present = false;

    for byte in data {
        let character = byte as char;

        if character == '\0' {
            null_present = true;
            continue;
        } else if null_present {
            return Err(CharacterDataError::NameContainsOtherCharacterAfterNull.into());
        }

        if character == '-' || character == '_' {
            if dash_or_underscore_present {
                return Err(CharacterDataError::NameContainsMultipleDashesOrUnderscores.into());
            }
            dash_or_underscore_present = true;
            result.push(character);
            continue;
        }

        if !character.is_ascii_alphabetic() {
            return Err(CharacterDataError::NameContainsInvalidCharacter(character).into());
        }

        result.push(character);
    }

    if result.len() < 2 {
        Err(CharacterDataError::NameTooShort.into())
    } else if result.len() > 15 {
        Err(CharacterDataError::NameTooLong.into())
    } else if result.starts_with('-') || result.starts_with('_') {
        Err(CharacterDataError::NameStartsWithDashOrUnderscore.into())
    } else if result.ends_with('-') || result.ends_with('_') {
        Err(CharacterDataError::NameEndsWithDashOrUnderscore.into())
    } else {
        Ok(result)
    }
}

pub fn read_class(data: u16) -> Result<CharacterClass, ReadCharacterSaveError> {
    match data {
        0 => Ok(CharacterClass::Amazon),
        1 => Ok(CharacterClass::Sorceress),
        2 => Ok(CharacterClass::Necromancer),
        3 => Ok(CharacterClass::Paladin),
        4 => Ok(CharacterClass::Barbarian),
        5 => Ok(CharacterClass::Druid),
        6 => Ok(CharacterClass::Assassin),
        // TODO: warlock
        _ => Err(CharacterDataError::InvalidClass(data).into()),
    }
}

pub fn read_menu_level(data: u16) -> Result<u8, ReadCharacterSaveError> {
    match data {
        0..100 => Ok(data as u8),
        _ => Err(CharacterDataError::InvalidMenuLevel(data).into()),
    }
}

pub fn read_status(data: u8) -> Result<CharacterStatus, ReadCharacterSaveError> {
    if data & 0b1101_0011 > 0 {
        return Err(CharacterDataError::InvalidStatus(data).into());
    }

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
                return Err(CharacterDataError::InvalidProgression {
                    expansion,
                    progression: data,
                }
                .into());
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
                return Err(CharacterDataError::InvalidProgression {
                    expansion,
                    progression: data,
                }
                .into());
            }
        },
    })
}

pub fn read_active_weapon_set(
    data: u32,
) -> Result<CharacterActiveWeaponSet, ReadCharacterSaveError> {
    match data {
        0 => Ok(CharacterActiveWeaponSet::WeaponSet1),
        1 => Ok(CharacterActiveWeaponSet::WeaponSet2),
        _ => Err(CharacterDataError::InvalidActiveWeaponSet(data).into()),
    }
}

pub fn read_menu_appearance(
    _data: [u8; 32],
) -> Result<CharacterMenuAppearance, ReadCharacterSaveError> {
    // TODO
    Ok(CharacterMenuAppearance)
}

pub fn read_skill_shortcuts_old(
    _data: [u8; 18],
) -> Result<CharacterSkillShortcuts, ReadCharacterSaveError> {
    // TODO
    Ok(CharacterSkillShortcuts)
}

pub fn read_skill_shortcuts_new(
    _data: [u8; 80],
) -> Result<CharacterSkillShortcuts, ReadCharacterSaveError> {
    // TODO
    Ok(CharacterSkillShortcuts)
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use super::*;

    #[test_case("XX")]
    #[test_case("GigaChadPaladin")]
    #[test_case("Mule_Endgame")]
    #[test_case("ber-stash")]
    fn returns_ok_on_valid_name(name: &'static str) {
        let len = name.len().clamp(0, 16);
        let mut name_data = [0u8; 16];
        name_data[..len].copy_from_slice(&name.as_bytes()[..len]);

        assert_eq!(name, read_name(name_data).unwrap())
    }

    #[test_case("", CharacterDataError::NameTooShort)]
    #[test_case("A", CharacterDataError::NameTooShort)]
    #[test_case("VeryLongBarbName", CharacterDataError::NameTooLong)]
    #[test_case("VeryLongBarbarianName", CharacterDataError::NameTooLong)]
    #[test_case(
        "AndyDestroyer69",
        CharacterDataError::NameContainsInvalidCharacter('6')
    )]
    #[test_case("_DruidBuild", CharacterDataError::NameStartsWithDashOrUnderscore)]
    #[test_case("-Singer", CharacterDataError::NameStartsWithDashOrUnderscore)]
    #[test_case("JavazonX_", CharacterDataError::NameEndsWithDashOrUnderscore)]
    #[test_case("Hammerdin-", CharacterDataError::NameEndsWithDashOrUnderscore)]
    #[test_case(
        "Giga-Pala-Build",
        CharacterDataError::NameContainsMultipleDashesOrUnderscores
    )]
    #[test_case(
        "Giga_Barb_Build",
        CharacterDataError::NameContainsMultipleDashesOrUnderscores
    )]
    #[test_case(
        "Giga-Sorc_Build",
        CharacterDataError::NameContainsMultipleDashesOrUnderscores
    )]
    #[test_case(
        "GigaBroken\0Name",
        CharacterDataError::NameContainsOtherCharacterAfterNull
    )]
    fn returns_err_on_invalid_names(name: &'static str, error: CharacterDataError) {
        let len = name.len().clamp(0, 16);
        let mut name_data = [0u8; 16];
        name_data[..len].copy_from_slice(&name.as_bytes()[..len]);

        assert_eq!(
            ReadCharacterSaveError::InvalidCharacterData(error),
            read_name(name_data).unwrap_err()
        )
    }

    #[test_case(0, CharacterClass::Amazon)]
    #[test_case(1, CharacterClass::Sorceress)]
    #[test_case(2, CharacterClass::Necromancer)]
    #[test_case(3, CharacterClass::Paladin)]
    #[test_case(4, CharacterClass::Barbarian)]
    #[test_case(5, CharacterClass::Druid)]
    #[test_case(6, CharacterClass::Assassin)]
    fn returns_ok_on_valid_class(data: u16, class: CharacterClass) {
        assert_eq!(Ok(class), read_class(data));
    }

    #[test_case(7)]
    #[test_case(10)]
    #[test_case(255)]
    #[test_case(256)]
    fn returns_err_on_invalid_class(data: u16) {
        assert_eq!(
            Err(ReadCharacterSaveError::InvalidCharacterData(
                CharacterDataError::InvalidClass(data)
            )),
            read_class(data)
        )
    }

    #[test_case(0b0000_0000, CharacterStatus::new(false, false, false))]
    #[test_case(0b0000_0100, CharacterStatus::new(true, false, false))]
    #[test_case(0b0000_1000, CharacterStatus::new(false, true, false))]
    #[test_case(0b0000_1100, CharacterStatus::new(true, true, false))]
    #[test_case(0b0010_0000, CharacterStatus::new(false, false, true))]
    #[test_case(0b0010_0100, CharacterStatus::new(true, false, true))]
    #[test_case(0b0010_1000, CharacterStatus::new(false, true, true))]
    #[test_case(0b0010_1100, CharacterStatus::new(true, true, true))]
    fn returns_ok_on_valid_status(data: u8, status: CharacterStatus) {
        assert_eq!(Ok(status), read_status(data));
    }

    #[test_case(0b0000_0001)]
    #[test_case(0b0000_0010)]
    #[test_case(0b0001_0000)]
    #[test_case(0b0100_0000)]
    #[test_case(0b1000_0000)]
    #[test_case(0b1111_1111)]
    fn returns_err_on_invalid_status(data: u8) {
        assert_eq!(
            Err(ReadCharacterSaveError::InvalidCharacterData(
                CharacterDataError::InvalidStatus(data)
            )),
            read_status(data)
        );
    }

    #[test_case(0, false, CharacterProgression::None)]
    #[test_case(1, false, CharacterProgression::NormalAndarielKilled)]
    #[test_case(2, false, CharacterProgression::NormalDurielKilled)]
    #[test_case(3, false, CharacterProgression::NormalMephistoKilled)]
    #[test_case(4, false, CharacterProgression::NormalFinalBossKilled)]
    #[test_case(5, false, CharacterProgression::NightmareAndarielKilled)]
    #[test_case(6, false, CharacterProgression::NightmareDurielKilled)]
    #[test_case(7, false, CharacterProgression::NightmareMephistoKilled)]
    #[test_case(8, false, CharacterProgression::NightmareFinalBossKilled)]
    #[test_case(9, false, CharacterProgression::HellAndarielKilled)]
    #[test_case(10, false, CharacterProgression::HellDurielKilled)]
    #[test_case(11, false, CharacterProgression::HellMephistoKilled)]
    #[test_case(12, false, CharacterProgression::HellFinalBossKilled)]
    #[test_case(0, true, CharacterProgression::None)]
    #[test_case(1, true, CharacterProgression::NormalAndarielKilled)]
    #[test_case(2, true, CharacterProgression::NormalDurielKilled)]
    #[test_case(3, true, CharacterProgression::NormalMephistoKilled)]
    #[test_case(5, true, CharacterProgression::NormalFinalBossKilled)]
    #[test_case(6, true, CharacterProgression::NightmareAndarielKilled)]
    #[test_case(7, true, CharacterProgression::NightmareDurielKilled)]
    #[test_case(8, true, CharacterProgression::NightmareMephistoKilled)]
    #[test_case(10, true, CharacterProgression::NightmareFinalBossKilled)]
    #[test_case(11, true, CharacterProgression::HellAndarielKilled)]
    #[test_case(12, true, CharacterProgression::HellDurielKilled)]
    #[test_case(13, true, CharacterProgression::HellMephistoKilled)]
    #[test_case(15, true, CharacterProgression::HellFinalBossKilled)]
    fn returns_ok_on_valid_progression(
        data: u8,
        expansion: bool,
        progression: CharacterProgression,
    ) {
        assert_eq!(Ok(progression), read_progression(data, expansion));
    }

    #[test_case(4, true)]
    #[test_case(9, true)]
    #[test_case(14, true)]
    #[test_case(13, false)]
    #[test_case(14, false)]
    #[test_case(16, true)]
    #[test_case(17, true)]
    fn returns_err_on_invalid_progression(data: u8, expansion: bool) {
        assert_eq!(
            Err(ReadCharacterSaveError::InvalidCharacterData(
                CharacterDataError::InvalidProgression {
                    expansion,
                    progression: data
                }
            )),
            read_progression(data, expansion)
        );
    }

    #[test_case(0, CharacterActiveWeaponSet::WeaponSet1)]
    #[test_case(1, CharacterActiveWeaponSet::WeaponSet2)]
    fn returns_ok_on_valid_active_weapon_set(data: u32, set: CharacterActiveWeaponSet) {
        assert_eq!(Ok(set), read_active_weapon_set(data));
    }

    #[test_case(2)]
    #[test_case(5)]
    #[test_case(256)]
    fn returns_err_on_invalid_active_weapon_set(data: u32) {
        assert_eq!(
            Err(ReadCharacterSaveError::InvalidCharacterData(
                CharacterDataError::InvalidActiveWeaponSet(data)
            )),
            read_active_weapon_set(data)
        );
    }
}
