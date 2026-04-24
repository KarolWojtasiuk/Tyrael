pub mod attribute;
pub mod character;
pub mod item;
pub mod location;
pub mod magic;
pub mod mercenary;
pub mod npc;
pub mod quest;
pub mod skill;
pub mod waypoint;

use bitter::{BitReader, LittleEndianReader};

use crate::errors::ReadCharacterSaveError;

pub trait ReaderExt {
    fn u8(&mut self) -> Result<u8, ReadCharacterSaveError>;
    fn u16(&mut self) -> Result<u16, ReadCharacterSaveError>;
    fn u32(&mut self) -> Result<u32, ReadCharacterSaveError>;
    fn bits(&mut self, bits: u32) -> Result<u64, ReadCharacterSaveError>;
    fn bytes<const B: usize>(&mut self) -> Result<[u8; B], ReadCharacterSaveError>;
}

impl<'a> ReaderExt for LittleEndianReader<'a> {
    fn u8(&mut self) -> Result<u8, ReadCharacterSaveError> {
        match self.read_u8() {
            Some(d) => Ok(d),
            None => Err(ReadCharacterSaveError::UnexpectedEOF),
        }
    }

    fn u16(&mut self) -> Result<u16, ReadCharacterSaveError> {
        match self.read_u16() {
            Some(d) => Ok(d),
            None => Err(ReadCharacterSaveError::UnexpectedEOF),
        }
    }

    fn u32(&mut self) -> Result<u32, ReadCharacterSaveError> {
        match self.read_u32() {
            Some(d) => Ok(d),
            None => Err(ReadCharacterSaveError::UnexpectedEOF),
        }
    }

    fn bits(&mut self, bits: u32) -> Result<u64, ReadCharacterSaveError> {
        match self.read_bits(bits) {
            Some(d) => Ok(d),
            None => Err(ReadCharacterSaveError::UnexpectedEOF),
        }
    }

    fn bytes<const B: usize>(&mut self) -> Result<[u8; B], ReadCharacterSaveError> {
        let mut buffer = [0; B];
        if self.read_bytes(&mut buffer) {
            Ok(buffer)
        } else {
            Err(ReadCharacterSaveError::UnexpectedEOF)
        }
    }
}
