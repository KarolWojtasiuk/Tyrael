mod common;
mod saves;

use common::*;
use tyrael::CharacterSave;
use tyrael::character::{
    CharacterActiveWeaponSet,
    CharacterClass,
    CharacterGameCompletion,
    CharacterInfo,
    CharacterMenuAppearance,
    CharacterSkillShortcuts,
    CharacterStatus,
};
use tyrael::progression::{GameAct, GameDifficulty, GameProgression, GameSaveLocation};

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

        if file.class != save.character.class {
            panic!(
                "{}: Character class mismatch (expected: {}, actual: {})",
                file.filename, file.class, save.character.class
            );
        }

        if save.character.menu_appearance.0.iter().all(|d| *d == 0) {
            panic!("{}: Empty character menu appearance", file.filename);
        }

        if save.character.menu_level == 0 || save.character.menu_level > 99 {
            panic!(
                "{}: Invalid character menu level {}",
                file.filename, save.character.menu_level
            );
        }

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
    let file = saves::normal::SACRIFICE_LEVEL_4;
    let save = CharacterSave::read(file.bytes).unwrap();
    let expected_save = CharacterSave {
        version: 71,
        character: CharacterInfo {
            name: "Test".to_string(),
            class: CharacterClass::Paladin,
            status: CharacterStatus::new(false, false, false),
            game_completion: CharacterGameCompletion::None,
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
        progression: GameProgression {
            save_location: GameSaveLocation::new(GameDifficulty::Normal, GameAct::Act1),
        },
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
}
