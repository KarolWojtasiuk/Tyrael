use bitter::{BitReader, BitWriter, LittleEndianReader, LittleEndianWriter};

use crate::data::SaveData;
use crate::errors::{ConvertToRawSaveDataError, ReadRawSaveDataError, WriteRawSaveDataError};
use crate::raw::common::EofExt;

mod common;
pub mod v71;
pub mod v87;
pub mod v89;
pub mod v92;
pub mod v96;

#[derive(Debug)]
pub enum RawSaveData {
    V71(v71::RawSaveDataV71),
    V87(v87::RawSaveDataV87),
    V89(v89::RawSaveDataV89),
    V92(v92::RawSaveDataV92),
    V96(v96::RawSaveDataV96),
}

const VERSION_HEADER_SIZE: usize = size_of::<u32>() * 2; // SIGNATURE + VERSION
const CHECKSUM_HEADER_SIZE: usize = size_of::<u32>() * 2; // SIZE + CHECKSUM

const SIGNATURE: u32 = 0xAA55AA55;
const MERCENARY_HEADER: u16 = 0x0000;
const QUESTS_HEADER: u32 = 0x216F6F57;
const WAYPOINTS_HEADER: u16 = 0x5357;
const NPCS_HEADER: u16 = 0x7701;
const ATTRIBUTES_HEADER: u16 = 0x6667;
const SKILLS_HEADER: u16 = 0x6669;
const ITEMS_HEADER: u16 = 0x4D4A;

impl RawSaveData {
    pub fn read(data: &[u8]) -> Result<Self, ReadRawSaveDataError> {
        let mut reader = LittleEndianReader::new(data);

        let signature = reader.read_u32().ok_or_eof()?;
        if signature != SIGNATURE {
            return Err(ReadRawSaveDataError::InvalidSignature { signature });
        }

        let version = reader.read_u32().ok_or_eof()?;
        match version {
            71 => Ok(Self::V71(v71::RawSaveDataV71::read(reader)?)),
            87 => Ok(Self::V87(v87::RawSaveDataV87::read(reader)?)),
            89 => Ok(Self::V89(v89::RawSaveDataV89::read(reader)?)),
            92 => Ok(Self::V92(v92::RawSaveDataV92::read(reader)?)),
            96 => Ok(Self::V96(v96::RawSaveDataV96::read(reader)?)),
            _ => Err(ReadRawSaveDataError::UnuspportedVersion { version }),
        }
    }

    pub fn write<W: std::io::Write>(&self, buffer: W) -> Result<(), WriteRawSaveDataError> {
        let version = match self {
            RawSaveData::V71(_) => 71,
            RawSaveData::V87(_) => 87,
            RawSaveData::V89(_) => 89,
            RawSaveData::V92(_) => 92,
            RawSaveData::V96(_) => 96,
        };

        let mut writer = LittleEndianWriter::new(buffer);
        writer.write_u32(SIGNATURE)?;
        writer.write_u32(version)?;

        match self {
            RawSaveData::V71(data) => data.write(writer),
            RawSaveData::V87(data) => data.write(writer),
            RawSaveData::V89(data) => data.write(writer),
            RawSaveData::V92(data) => data.write(writer),
            RawSaveData::V96(data) => data.write(writer),
        }
    }

    pub fn to_data(&self) -> SaveData {
        match self {
            RawSaveData::V71(data) => data.to_data(),
            RawSaveData::V87(data) => data.to_data(),
            RawSaveData::V89(data) => data.to_data(),
            RawSaveData::V92(data) => data.to_data(),
            RawSaveData::V96(data) => data.to_data(),
        }
    }

    pub fn from_data(
        data: &SaveData,
        target_version: u32,
    ) -> Result<Self, ConvertToRawSaveDataError> {
        match target_version {
            71 => Ok(Self::V71(v71::RawSaveDataV71::from_data(data))),
            87 => Ok(Self::V87(v87::RawSaveDataV87::from_data(data))),
            89 => Ok(Self::V89(v89::RawSaveDataV89::from_data(data))),
            92 => Ok(Self::V92(v92::RawSaveDataV92::from_data(data))),
            96 => Ok(Self::V96(v96::RawSaveDataV96::from_data(data))),
            _ => Err(ConvertToRawSaveDataError::UnuspportedVersion { target_version }),
        }
    }
}
