use std::slice::Iter;

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
    data: &mut Iter<u8>,
    version: u32,
) -> Result<CharacterSave, ReadCharacterSaveError> {
    Ok(CharacterSave {
        version,
        character: read_character_data(data, version)?,
        location: read_location_data(data)?,
        mercenary: None,
        quests: read_quest_data(data)?,
        waypoints: read_waypoint_data(data)?,
        npcs: read_npc_data(data)?,
        attributes: read_attribute_data(data)?,
        skills: read_skill_data(data)?,
        items: read_item_data(data)?,
    })
}

fn read_character_data(
    data: &mut Iter<u8>,
    version: u32,
) -> Result<CharacterData, ReadCharacterSaveError> {
    let name = character::read_name(data.read_bytes::<16>()?)?;
    let status = character::read_status(data.read_u8()?)?;
    let progression = character::read_progression(data.read_u8()?, status.expansion)?;
    let active_weapon_set = character::read_active_weapon_set(data.read_u16()? as u32)?;

    magic::assert_magic_value_u16(
        match version {
            71 => 0x00DD,
            87 | 89 => 0x013F,
            _ => unreachable!("unexpected version '{}' for v71_v87_v89 reader", version),
        },
        data.read_u16()?,
    )?;
    magic::assert_magic_value_u32(0x00820010, data.read_u32()?)?;

    let class = character::read_class(data.read_u16()?)?;
    let menu_level = character::read_menu_level(data.read_u16()?)?;
    let menu_appearance = character::read_menu_appearance(data.read_bytes::<32>()?)?;
    let skill_shortcuts = character::read_skill_shortcuts_old(data.read_bytes::<18>()?)?;

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

fn read_location_data(data: &mut Iter<u8>) -> Result<LocationData, ReadCharacterSaveError> {
    let save_location = location::read_save_location_old(data.read_u16()?)?;
    magic::assert_magic_value([0x00; 36], data.read_bytes::<36>()?)?;
    let seed = data.read_u32()?;

    Ok(LocationData::new(seed, save_location))
}

fn read_quest_data(data: &mut Iter<u8>) -> Result<QuestData, ReadCharacterSaveError> {
    magic::assert_magic_value_u32(0x216F6F57, data.read_u32()?)?;
    magic::assert_magic_value_u32(0x00000006, data.read_u32()?)?;

    // TODO
    data.read_bytes::<290>()?;
    Ok(QuestData)
}

fn read_waypoint_data(data: &mut Iter<u8>) -> Result<WaypointData, ReadCharacterSaveError> {
    magic::assert_magic_value_u16(0x5357, data.read_u16()?)?;
    magic::assert_magic_value_u32(0x00000001, data.read_u32()?)?;

    // TODO
    data.read_bytes::<74>()?;
    Ok(WaypointData)
}

fn read_npc_data(data: &mut Iter<u8>) -> Result<NpcData, ReadCharacterSaveError> {
    magic::assert_magic_value_u16(0x7701, data.read_u16()?)?;

    // TODO
    data.read_bytes::<50>()?;
    Ok(NpcData)
}

fn read_attribute_data(data: &mut Iter<u8>) -> Result<AttributeData, ReadCharacterSaveError> {
    magic::assert_magic_value_u16(0x6667, data.read_u16()?)?;
    attribute::read_attributes_old(data)
}

fn read_skill_data(data: &mut Iter<u8>) -> Result<SkillData, ReadCharacterSaveError> {
    magic::assert_magic_value_u16(0x6669, data.read_u16()?)?;

    // TODO
    Ok(SkillData)
}

fn read_item_data(_data: &mut Iter<u8>) -> Result<ItemData, ReadCharacterSaveError> {
    // TODO
    Ok(ItemData)
}
