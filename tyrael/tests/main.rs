mod common;
mod saves;

use common::*;
use tyrael::CharacterSave;
use tyrael::character::{
    CharacterActiveWeaponSet,
    CharacterClass,
    CharacterData,
    CharacterMenuAppearance,
    CharacterProgression,
    CharacterSkillShortcuts,
    CharacterStatus,
};
use tyrael::location::{GameAct, GameDifficulty, GameSaveLocation, LocationData};
use tyrael::mercenary::{MercenaryData, MercenaryKind};
use tyrael::quest::QuestData;

#[test]
fn all_saves_are_correctly_load_and_saved() {
    // TODO: unfilter resurrected and pd2
    for file in saves::all()
        .iter()
        .filter(|s| s.game == TestSaveGame::Classic || s.game == TestSaveGame::ClassicLoD)
    {
        let save = match CharacterSave::read(file.bytes) {
            Ok(s) => s,
            Err(e) => panic!("{}: Cannot read save file ({})", file.filename, e),
        };

        assert_eq!(
            file.class, save.character.class,
            "{}: Character class mismatch (expected: {}, actual: {})",
            file.filename, file.class, save.character.class
        );

        assert!(
            save.character.menu_appearance.0.iter().any(|d| *d != 0),
            "{}: Empty character menu appearance",
            file.filename
        );

        assert!(
            save.character.menu_level > 0 && save.character.menu_level <= 99,
            "{}: Invalid character menu level {}",
            file.filename,
            save.character.menu_level
        );

        assert_ne!(
            0, save.location.seed,
            "{}: Invalid seed {}",
            file.filename, save.location.seed
        );

        // TODO:
        // let output = match save.write() {
        //     Ok(o) => o,
        //     Err(e) => panic!("{}: Cannot write save file ({})", file.filename, e),
        // };
        // assert_eq!(file.bytes, output.as_slice());
    }
}

#[test]
fn sacrifice_paladin_is_parsed_correctly() {
    let file = saves::normal::SACRIFICE_PALADIN_LEVEL_4;
    let save = CharacterSave::read(file.bytes).unwrap();
    let expected_save = CharacterSave {
        version: 71,
        character: CharacterData {
            name: "Test".to_string(),
            class: CharacterClass::Paladin,
            status: CharacterStatus::new(false, false, false),
            progression: CharacterProgression::None,
            active_weapon_set: CharacterActiveWeaponSet::WeaponSet1,
            menu_level: 4,
            menu_appearance: CharacterMenuAppearance([
                // TODO
                5, 1, 1, 1, 1, 47, 255, 27, 2, 2, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
                255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
            ]),
            skill_shortcuts: CharacterSkillShortcuts::Short {
                // TODO
                keyboard: [
                    0x0100, 0x0160, 0x00FF, 0x00FF, 0x00FF, 0x00FF, 0x00FF, 0x00FF,
                ],
                lmb: 0x60,
                rmb: 0x62,
            },
            last_played_at: None,
        },
        location: LocationData::new(
            11293296,
            GameSaveLocation::new(GameDifficulty::Normal, GameAct::Act1),
        ),
        mercenary: None,
        quests: QuestData {},
    };
    assert_eq!(expected_save, save);

    // attributes: 25 20 40 15
    // life 73/119
    // mana 23/23
    // level 4
    // exp: 6595/7875
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
fn tiger_assassin_is_parsed_correctly() {
    let file = saves::normal::TIGER_ASSASSIN_LEVEL_5;
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
            menu_appearance: CharacterMenuAppearance([
                // TODO
                255, 2, 1, 1, 1, 4, 255, 79, 2, 2, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
                255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
            ]),
            skill_shortcuts: CharacterSkillShortcuts::Long {
                // TODO
                keyboard: [
                    0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF,
                    0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF,
                ],
                lmb: 0x0000,
                rmb: 0xFE,
                lmb_switch: 0x0000,
                rmb_switch: 0xFE,
            },
            last_played_at: Some(1776850490),
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
        quests: QuestData {},
    };
    assert_eq!(expected_save, save);

    // attributes: 20 20 40 25
    // life 126/126 (base:123)
    // mana 31/31
    // level 5
    // exp: 13115/14175
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
    // gold: 757
    // stash gold: 111
    // stash 0,0: bardiche of the bat: 4% mana stolen per hit
    // merc:
    //      name: kundri
    //      level: 4
    //      exp: 8400/15750
    //      head: skull cap with amethyst
    //      hand: 3 sockets short bow
    //      body: quilted armor
    // stony field wp taken
}
