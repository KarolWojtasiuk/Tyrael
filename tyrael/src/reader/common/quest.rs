use std::slice::Iter;

use crate::errors::ReadCharacterSaveError;
use crate::reader::common::ReaderExt;

pub fn read_woo_header(data: &mut Iter<u8>) -> Result<(), ReadCharacterSaveError> {
    {
        // Woo! header
        const EXPECTED: u32 = 0x216F6F57;
        let actual = data.read_u32().unwrap();
        if actual != EXPECTED {
            return Err(ReadCharacterSaveError::InvalidMagicValue {
                expected: EXPECTED.to_le_bytes().to_vec(),
                actual: actual.to_le_bytes().to_vec(),
            });
        }
    }

    {
        // Woo! header
        const EXPECTED: u32 = 0x00000006;
        let actual = data.read_u32()?;
        if actual != EXPECTED {
            return Err(ReadCharacterSaveError::InvalidMagicValue {
                expected: EXPECTED.to_le_bytes().to_vec(),
                actual: actual.to_le_bytes().to_vec(),
            });
        }
    }

    Ok(())
}
