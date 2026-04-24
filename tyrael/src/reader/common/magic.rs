use crate::errors::ReadCharacterSaveError;

pub fn assert_magic_value<const S: usize>(
    expected: [u8; S],
    actual: [u8; S],
) -> Result<(), ReadCharacterSaveError> {
    if cfg!(feature = "strict") && actual != expected {
        return Err(ReadCharacterSaveError::InvalidMagicValue {
            expected: expected.to_vec(),
            actual: actual.to_vec(),
        });
    }
    Ok(())
}

pub fn assert_magic_value_u8(expected: u8, actual: u8) -> Result<(), ReadCharacterSaveError> {
    if cfg!(feature = "strict") && actual != expected {
        return Err(ReadCharacterSaveError::InvalidMagicValue {
            expected: vec![expected],
            actual: vec![actual],
        });
    }
    Ok(())
}

pub fn assert_magic_value_u16(expected: u16, actual: u16) -> Result<(), ReadCharacterSaveError> {
    if cfg!(feature = "strict") && actual != expected {
        return Err(ReadCharacterSaveError::InvalidMagicValue {
            expected: expected.to_le_bytes().to_vec(),
            actual: actual.to_le_bytes().to_vec(),
        });
    }
    Ok(())
}

pub fn assert_magic_value_u32(expected: u32, actual: u32) -> Result<(), ReadCharacterSaveError> {
    if cfg!(feature = "strict") && actual != expected {
        return Err(ReadCharacterSaveError::InvalidMagicValue {
            expected: expected.to_le_bytes().to_vec(),
            actual: actual.to_le_bytes().to_vec(),
        });
    }
    Ok(())
}
