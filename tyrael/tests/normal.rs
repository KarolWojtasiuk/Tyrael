mod common;

use tyrael::attribute::*;
use tyrael::character::*;
use tyrael::errors::ReadCharacterSaveError;
use tyrael::item::*;
use tyrael::location::*;
use tyrael::mercenary::*;
use tyrael::npc::*;
use tyrael::quest::*;
use tyrael::skill::*;
use tyrael::waypoint::*;
use tyrael::{CharacterSave, U24F8};

#[test]
fn v100_sacrifice_paladin_level_4_is_parsed_correctly() -> Result<(), ReadCharacterSaveError> {
    let file = save!(Classic, "1.00", Classic, Paladin, Normal, "SacrificeLevel4");
    let save = CharacterSave::read(file.bytes)?;
    let expected_save = CharacterSave {
        version: 71,
        character: CharacterData {
            name: "Test".to_string(),
            class: CharacterClass::Paladin,
            status: CharacterStatus::new(false, false, false),
            progression: CharacterProgression::None,
            active_weapon_set: CharacterActiveWeaponSet::WeaponSet1,
            menu_level: 4,
            menu_appearance: CharacterMenuAppearance,
            skill_shortcuts: CharacterSkillShortcuts,
            last_played_at: None,
        },
        location: LocationData::new(
            11293296,
            GameSaveLocation::new(GameDifficulty::Normal, GameAct::Act1),
        ),
        mercenary: None,
        quests: QuestData,
        waypoints: WaypointData,
        npcs: NpcData,
        attributes: AttributeData::new(
            PermanentStats::new(25, 20, 40, 15, 0, 0),
            DynamicStats::new(
                U24F8::lit("73.8245"),
                U24F8::lit("106"),
                U24F8::lit("23.5"),
                U24F8::lit("19.5"),
                U24F8::lit("107"),
                U24F8::lit("107"),
            ),
            RankStats::new(4, 6595, 55, 0),
        ),
        skills: SkillData,
        items: ItemData,
    };
    assert_eq!(expected_save, save);
    Ok(())

    // skills: sacrifice (1) (lmb), might(2) (rmb)
    // equipment:
    //      left hand: rare hand axe (demon thirst) dur: 12/28
    //          +1 to mana after each kill
    //          enhanced damage
    //          +1 damage
    //          slightly increased attack speed
    //      right hand: crude buckler dur: 2/4
    //      head: lizard's skull cap of the fox dur: 16/18 + +4 mana, +9 life
    //      armor: cracked quilted armor dur: 1/6
    //      hands: sturdy heavy gloves dur: 7/14 + enhanced defense
    //      belt: azure light belt: +6% cold resist
    //      ring: ring of the jackal: +4 life
    // inventory:
    // 0,0: unid hand axe
    // 1,0: large shield of balance +fhr
    // merc: paige
    // cold plains wp taken
}

#[test]
fn v113d_tiger_assassin_level_5_is_parsed_correctly() -> Result<(), ReadCharacterSaveError> {
    let file = save!(ClassicLoD, "1.13d", LoD, Assassin, Normal, "TigerLevel5");
    let save = CharacterSave::read(file.bytes).unwrap();
    let expected_save = CharacterSave {
        version: 96,
        character: CharacterData {
            name: "Test".to_string(),
            class: CharacterClass::Assassin,
            status: CharacterStatus::new(true, false, true),
            progression: CharacterProgression::None,
            active_weapon_set: CharacterActiveWeaponSet::WeaponSet2,
            menu_level: 5,
            menu_appearance: CharacterMenuAppearance,
            skill_shortcuts: CharacterSkillShortcuts,
            last_played_at: Some(1777055826),
        },
        location: LocationData::new(
            1194327793,
            GameSaveLocation::new(GameDifficulty::Normal, GameAct::Act1),
        ),
        mercenary: Some(MercenaryData {
            seed: 1055498175,
            name_id: 10,
            kind: MercenaryKind::Act1,
            experience: 8400,
            dead: false,
        }),
        quests: QuestData,
        waypoints: WaypointData,
        npcs: NpcData,
        attributes: AttributeData::new(
            PermanentStats::new(20, 20, 40, 25, 0, 0),
            DynamicStats::new(
                U24F8::lit("126"),
                U24F8::lit("118"),
                U24F8::lit("31"),
                U24F8::lit("31"),
                U24F8::lit("125"),
                U24F8::lit("125"),
            ),
            RankStats::new(5, 13115, 757, 111),
        ),
        skills: SkillData,
        items: ItemData,
    };
    assert_eq!(expected_save, save);
    Ok(())

    // skills: normal attack (lmb), tiger strike (4) (rmb)
    // equipment:
    //      left hand: eth rare hand axe (bramble razor) dur: 13/15
    //          +2 to min/max
    //          +18 attack rating
    //          +1-2 fire dmg
    //          +1 mana after kill
    //      right hand: rare buckler (grim tower) dur: 12/12
    //          +1 life
    //          +5% poison res
    //          attacker takes 1dmg
    //          +1 light radius lmao
    //          repairs 1 dur in 33s
    //      armor: cracked leather armor dur: 3/7
    //      belt: rare sash (blood clasp):
    //          +2 str
    //          +5 life
    //          +5% cold res
    //          attacker takes 1dmg
    //      amu: tangerine amulet of the jackal: +2 life +5% light res
    // inventory:
    // 0,0: large charm: 10% extra gold
    // 7,0: rare cap (soul shell)
    //      +11%ed
    //      +2 energy
    //      +5% fire res
    //      repairs 1 dur in 33s
    //      2 sockets
    // stash 0,0: bardiche of the bat: 4% mana stolen per hit
    // merc:
    //      head: skull cap with amethyst
    //      hand: 3 sockets short bow
    //      body: quilted armor
    // stony field wp taken
}
