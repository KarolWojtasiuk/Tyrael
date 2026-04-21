use std::slice::Iter;

use crate::errors::{CharacterNameError, ReadCharacterSaveError};
use crate::reader::common::ReaderExt;

pub fn read_character_name(data: &mut Iter<u8>) -> Result<String, ReadCharacterSaveError> {
    let data = data.read_bytes::<16>()?;

    let mut result = String::new();
    let mut null_present = false;
    let mut dash_or_underscore_present = false;

    for byte in data {
        let character = byte as char;

        if character == '\0' {
            null_present = true;
            continue;
        } else if null_present {
            return Err(ReadCharacterSaveError::InvalidCharacterName(
                CharacterNameError::NameContainsOtherCharacterAfterNull,
            ));
        }

        if character == '-' || character == '_' {
            if dash_or_underscore_present {
                return Err(ReadCharacterSaveError::InvalidCharacterName(
                    CharacterNameError::NameContainsMultipleDashesOrUnderscores,
                ));
            }
            dash_or_underscore_present = true;
            result.push(character);
            continue;
        }

        if !character.is_ascii_alphabetic() {
            return Err(ReadCharacterSaveError::InvalidCharacterName(
                CharacterNameError::NameContainsInvalidCharacter(character),
            ));
        }

        result.push(character);
    }

    if result.len() < 2 {
        Err(ReadCharacterSaveError::InvalidCharacterName(
            CharacterNameError::NameTooShort,
        ))
    } else if result.len() > 15 {
        Err(ReadCharacterSaveError::InvalidCharacterName(
            CharacterNameError::NameTooLong,
        ))
    } else if result.starts_with('-') || result.starts_with('_') {
        Err(ReadCharacterSaveError::InvalidCharacterName(
            CharacterNameError::NameStartsWithDashOrUnderscore,
        ))
    } else if result.ends_with('-') || result.ends_with('_') {
        Err(ReadCharacterSaveError::InvalidCharacterName(
            CharacterNameError::NameEndsWithDashOrUnderscore,
        ))
    } else {
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::read_character_name;
    use crate::errors::{CharacterNameError, ReadCharacterSaveError};

    #[test]
    fn returns_ok_on_valid_names() {
        for name in ["XX", "GigaChadPaladin", "Mule_Endgame", "ber-stash"] {
            let len = name.len().clamp(0, 16);
            let mut name_data = [0u8; 16];
            name_data[..len].copy_from_slice(&name.as_bytes()[..len]);

            assert_eq!(name, read_character_name(&mut name_data.iter()).unwrap())
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

            assert_eq!(
                ReadCharacterSaveError::InvalidCharacterName(error),
                read_character_name(&mut name_data.iter()).unwrap_err()
            )
        }
    }
}
