use std::slice::Iter;

use crate::errors::ReadCharacterSaveError;

pub fn read_header(_data: &mut Iter<u8>) -> Result<(), ReadCharacterSaveError> {
    // TODO
    Ok(())
}
