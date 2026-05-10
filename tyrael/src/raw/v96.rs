use std::io::Write;

use bitter::{BitReader, BitWriter, LittleEndianReader, LittleEndianWriter};

use crate::data::SaveData;
use crate::errors::{ReadRawSaveDataError, WriteRawSaveDataError};
use crate::raw::common::{EofExt, ReaderExt, calculate_checksum};
use crate::raw::{
    ATTRIBUTES_HEADER,
    CHECKSUM_HEADER_SIZE,
    ITEMS_HEADER,
    MERCENARY_HEADER,
    NPCS_HEADER,
    QUESTS_HEADER,
    SIGNATURE,
    SKILLS_HEADER,
    VERSION_HEADER_SIZE,
    WAYPOINTS_HEADER,
};

#[derive(Debug)]
pub struct RawSaveDataV96 {
    pub character: [u8; 159],
    pub mercenary: [u8; 158],
    pub quests: [u8; 294],
    pub waypoints: [u8; 78],
    pub npcs: [u8; 50],
    pub attributes: Vec<u8>,
    pub skills: [u8; 30],
    pub items: Vec<u8>,
}

impl RawSaveDataV96 {
    pub fn read(mut reader: LittleEndianReader) -> Result<Self, ReadRawSaveDataError> {
        let _size = reader.read_u32().ok_or_eof()?;
        let _checksum = reader.read_i32().ok_or_eof()?;

        let character = reader.read_array::<159>()?;

        reader.assert_section_header_u16(MERCENARY_HEADER)?;
        let mercenary = reader.read_array::<158>()?;

        reader.assert_section_header_u32(QUESTS_HEADER)?;
        let quests = reader.read_array::<294>()?;

        reader.assert_section_header_u16(WAYPOINTS_HEADER)?;
        let waypoints = reader.read_array::<78>()?;

        reader.assert_section_header_u16(NPCS_HEADER)?;
        let npcs = reader.read_array::<50>()?;

        reader.assert_section_header_u16(ATTRIBUTES_HEADER)?;
        let attributes = read_attributes(&mut reader)?;

        reader.assert_section_header_u16(SKILLS_HEADER)?;
        let skills = reader.read_array::<30>()?;

        reader.assert_section_header_u16(ITEMS_HEADER)?;
        let items = reader.remainder().data().to_vec();

        Ok(Self {
            character,
            mercenary,
            quests,
            waypoints,
            npcs,
            attributes,
            skills,
            items,
        })
    }

    pub fn write<W: std::io::Write>(
        &self,
        mut writer: LittleEndianWriter<W>,
    ) -> Result<(), WriteRawSaveDataError> {
        let data = {
            let mut buffer = vec![];
            let writer = LittleEndianWriter::new(&mut buffer);

            Self::write_without_checksum(self, writer)?;
            buffer
        };

        let size = (VERSION_HEADER_SIZE + CHECKSUM_HEADER_SIZE + data.len()) as u32;
        let mut checksum = 0;
        calculate_checksum(&mut checksum, &SIGNATURE.to_le_bytes());
        calculate_checksum(&mut checksum, &96u32.to_le_bytes()); // version
        calculate_checksum(&mut checksum, &size.to_le_bytes());
        calculate_checksum(&mut checksum, &0u32.to_le_bytes()); // empty checksum
        calculate_checksum(&mut checksum, &data);

        writer.write_u32(size)?;
        writer.write_i32(checksum)?;
        writer.write_all(&data)?;
        Ok(())
    }

    fn write_without_checksum<W: std::io::Write>(
        &self,
        mut writer: LittleEndianWriter<W>,
    ) -> Result<(), WriteRawSaveDataError> {
        writer.write_all(&self.character)?;

        writer.write_u16(MERCENARY_HEADER)?;
        writer.write_all(&self.mercenary)?;

        writer.write_u32(QUESTS_HEADER)?;
        writer.write_all(&self.quests)?;

        writer.write_u16(WAYPOINTS_HEADER)?;
        writer.write_all(&self.waypoints)?;

        writer.write_u16(NPCS_HEADER)?;
        writer.write_all(&self.npcs)?;

        writer.write_u16(ATTRIBUTES_HEADER)?;
        writer.write_all(&self.attributes)?;

        writer.write_u16(SKILLS_HEADER)?;
        writer.write_all(&self.skills)?;

        writer.write_u16(ITEMS_HEADER)?;
        writer.write_all(&self.items)?;

        Ok(())
    }

    pub fn to_data(&self) -> SaveData {
        todo!()
    }

    pub fn from_data(_data: &SaveData) -> Self {
        todo!()
    }
}

pub fn read_attributes(reader: &mut LittleEndianReader) -> Result<Vec<u8>, ReadRawSaveDataError> {
    let mut result = vec![];
    let mut writer = LittleEndianWriter::new(&mut result);

    const SIZES: [u8; 16] = [10, 10, 10, 10, 10, 8, 21, 21, 21, 21, 21, 21, 7, 32, 25, 25];
    const SIZES_LEN: u16 = SIZES.len() as u16;

    loop {
        let id = reader.read_bits(9).ok_or_eof()?;
        writer.write_bits(9, id).expect("write to vec failed");

        match id as u16 {
            0..SIZES_LEN => {
                let size = SIZES[id as usize];
                writer
                    .write_bits(size as u32, reader.read_bits(size as u32).ok_or_eof()?)
                    .expect("write to vec failed");
            }
            511 => {
                let partial_bits = reader.remainder().partial_bits();
                writer
                    .write_bits(
                        partial_bits as u32,
                        reader.read_bits(partial_bits as u32).ok_or_eof()?,
                    )
                    .expect("write to vec failed");

                assert_eq!(0, reader.remainder().partial_bits());
                assert_eq!(0, reader.remainder().partial_byte());
                break;
            }
            _ => {
                return Err(ReadRawSaveDataError::CorruptedData {
                    reason: "invalid attribute id",
                });
            }
        }
    }

    drop(writer);
    Ok(result)
}

#[cfg(test)]
mod tests {
    use bitter::{LittleEndianReader, LittleEndianWriter};
    use test_case::test_case;

    use crate::raw::VERSION_HEADER_SIZE;
    use crate::raw::v96::RawSaveDataV96;
    use crate::tests::TestSave;
    use crate::tests::saves::classic_lod::*;

    #[test_case(v1_10::CLASSIC_AMAZON_STARTER)]
    #[test_case(v1_10::LOD_ASSASSIN_STARTER)]
    #[test_case(v1_11b::CLASSIC_BARBARIAN_STARTER)]
    #[test_case(v1_11b::LOD_DRUID_STARTER)]
    #[test_case(v1_12a::CLASSIC_NECROMANCER_STARTER)]
    #[test_case(v1_12a::LOD_ASSASSIN_STARTER)]
    #[test_case(v1_13d::CLASSIC_PALADIN_STARTER)]
    #[test_case(v1_13d::LOD_DRUID_STARTER)]
    #[test_case(v1_14b::CLASSIC_SORCERESS_STARTER)]
    #[test_case(v1_14b::LOD_ASSASSIN_STARTER)]
    #[test_case(v1_14d::CLASSIC_AMAZON_STARTER)]
    #[test_case(v1_14d::LOD_DRUID_STARTER)]
    fn data_is_read_and_written_losslessly(save: TestSave) {
        let input = &save.bytes[VERSION_HEADER_SIZE..];
        let reader = LittleEndianReader::new(input);
        let data = RawSaveDataV96::read(reader)
            .unwrap_or_else(|e| panic!("Cannot read raw save data ({e})"));

        let mut output = vec![];
        data.write(LittleEndianWriter::new(&mut output))
            .unwrap_or_else(|e| panic!("Cannot write raw save data ({e})"));

        assert_eq!(input, output);
    }
}
