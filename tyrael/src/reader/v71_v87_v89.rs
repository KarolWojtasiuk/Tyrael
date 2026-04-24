use bitter::LittleEndianReader;

use crate::CharacterSave;
use crate::attribute::AttributeData;
use crate::character::CharacterData;
use crate::errors::ReadCharacterSaveError;
use crate::item::ItemData;
use crate::location::LocationData;
use crate::npc::NpcData;
use crate::quest::QuestData;
use crate::reader::common::*;
use crate::skill::SkillData;
use crate::waypoint::WaypointData;

pub fn read_character_save(
    reader: &mut LittleEndianReader,
    version: u32,
) -> Result<CharacterSave, ReadCharacterSaveError> {
    Ok(CharacterSave {
        version,
        character: read_character_data(reader, version)?,
        location: read_location_data(reader)?,
        mercenary: None,
        quests: read_quest_data(reader)?,
        waypoints: read_waypoint_data(reader)?,
        npcs: read_npc_data(reader)?,
        attributes: read_attribute_data(reader, version)?,
        skills: read_skill_data(reader)?,
        items: read_item_data(reader)?,
    })
}

fn read_character_data(
    reader: &mut LittleEndianReader,
    version: u32,
) -> Result<CharacterData, ReadCharacterSaveError> {
    let name = character::read_name(reader.bytes::<16>()?)?;
    let status = character::read_status(reader.u8()?)?;
    let progression = character::read_progression(reader.u8()?, status.expansion)?;
    let active_weapon_set = character::read_active_weapon_set(reader.u16()? as u32)?;

    magic::assert_magic_value_u16(
        match version {
            71 => 0x00DD,
            87 | 89 => 0x013F,
            _ => unreachable!("unexpected version '{}' for v71_v87_v89 reader", version),
        },
        reader.u16()?,
    )?;
    magic::assert_magic_value_u32(0x00820010, reader.u32()?)?;

    let class = character::read_class(reader.u16()?)?;
    let menu_level = character::read_menu_level(reader.u16()?)?;
    let menu_appearance = character::read_menu_appearance(reader.bytes::<32>()?)?;
    let skill_shortcuts = character::read_skill_shortcuts_old(reader.bytes::<18>()?)?;

    Ok(CharacterData {
        name,
        class,
        status,
        progression,
        active_weapon_set,
        menu_level,
        menu_appearance,
        skill_shortcuts,
        last_played_at: None,
    })
}

fn read_location_data(
    reader: &mut LittleEndianReader,
) -> Result<LocationData, ReadCharacterSaveError> {
    let save_location = location::read_save_location_old(reader.u16()?)?;
    magic::assert_magic_value([0x00; 36], reader.bytes::<36>()?)?;
    let seed = reader.u32()?;

    Ok(LocationData::new(seed, save_location))
}

fn read_quest_data(data: &mut LittleEndianReader) -> Result<QuestData, ReadCharacterSaveError> {
    magic::assert_magic_value_u32(0x216F6F57, data.u32()?)?;
    magic::assert_magic_value_u32(0x00000006, data.u32()?)?;

    // TODO
    data.bytes::<290>()?;
    Ok(QuestData)
}

fn read_waypoint_data(
    reader: &mut LittleEndianReader,
) -> Result<WaypointData, ReadCharacterSaveError> {
    magic::assert_magic_value_u16(0x5357, reader.u16()?)?;
    magic::assert_magic_value_u32(0x00000001, reader.u32()?)?;

    // TODO
    reader.bytes::<74>()?;
    Ok(WaypointData)
}

fn read_npc_data(data: &mut LittleEndianReader) -> Result<NpcData, ReadCharacterSaveError> {
    magic::assert_magic_value_u16(0x7701, data.u16()?)?;

    // TODO
    data.bytes::<50>()?;
    Ok(NpcData)
}

fn read_attribute_data(
    reader: &mut LittleEndianReader,
    version: u32,
) -> Result<AttributeData, ReadCharacterSaveError> {
    magic::assert_magic_value_u16(0x6667, reader.u16()?)?;
    attribute::read_attributes_old(reader, version)
}

fn read_skill_data(reader: &mut LittleEndianReader) -> Result<SkillData, ReadCharacterSaveError> {
    magic::assert_magic_value_u16(0x6669, reader.u16()?)?;

    // TODO
    Ok(SkillData)
}

fn read_item_data(_reader: &mut LittleEndianReader) -> Result<ItemData, ReadCharacterSaveError> {
    // TODO
    Ok(ItemData)
}
