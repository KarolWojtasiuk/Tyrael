use std::slice::Iter;

use crate::errors::ReadCharacterSaveError;

pub fn read_header(_data: &mut Iter<u8>) -> Result<(), ReadCharacterSaveError> {
    // {
    //     const EXPECTED: u16 = 0x6669;
    //     let actual = data.read_u16().unwrap();
    //     if actual != EXPECTED {
    //         return Err(ReadCharacterSaveError::InvalidMagicValue {
    //             expected: EXPECTED.to_le_bytes().to_vec(),
    //             actual: actual.to_le_bytes().to_vec(),
    //         });
    //     }
    // }

    // TODO
    Ok(())
}
