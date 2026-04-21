mod common;
mod saves;

use common::*;
use tyrael::CharacterSave;
use tyrael::character::{
    CharacterActiveWeaponSet,
    CharacterClass,
    CharacterProgression,
    CharacterStatus,
};

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
    assert_eq!(71, save.version);
    assert_eq!("Test", save.character.name);
    assert_eq!(CharacterClass::Paladin, save.character.class);
    assert_eq!(
        CharacterStatus::new(false, false, false),
        save.character.status
    );
    assert_eq!(CharacterProgression::None, save.character.progression);
    assert_eq!(
        CharacterActiveWeaponSet::WeaponSet1,
        save.character.active_weapon
    );
    assert_eq!(4, save.character.menu_level);
    assert_eq!(None, save.character.last_played_at);

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
