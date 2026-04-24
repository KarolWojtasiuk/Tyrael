mod common;
mod v71_v87_v89;
mod v92_v96;

use bitter::LittleEndianReader;

use crate::CharacterSave;
use crate::errors::ReadCharacterSaveError;
use crate::reader::common::ReaderExt;

const SIGNATURE: u32 = 0xAA55AA55;
pub fn read_character_save(data: &[u8]) -> Result<CharacterSave, ReadCharacterSaveError> {
    let mut reader = LittleEndianReader::new(data);

    let signature = reader.u32()?;
    if signature != SIGNATURE {
        return Err(ReadCharacterSaveError::InvalidSignature(signature));
    }

    let version = reader.u32()?;
    match version {
        71 | 87 | 89 => v71_v87_v89::read_character_save(&mut reader, version),
        92 | 96 => v92_v96::read_character_save(&mut reader, version),
        _ => Err(ReadCharacterSaveError::UnsupportedVersion(version)),
    }
}
