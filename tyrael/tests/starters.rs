mod common;

use common::*;
use test_case::test_matrix;
use tyrael::CharacterSave;
use tyrael::errors::ReadCharacterSaveError;

#[test_matrix([
    // 1.00
    save!(Classic, "1.00", Classic, Amazon, Starter),
    save!(Classic, "1.00", Classic, Barbarian, Starter),
    save!(Classic, "1.00", Classic, Necromancer, Starter),
    save!(Classic, "1.00", Classic, Paladin, Starter),
    save!(Classic, "1.00", Classic, Sorceress, Starter),
    // 1.01
    save!(Classic, "1.01", Classic, Amazon, Starter),
    save!(Classic, "1.01", Classic, Barbarian, Starter),
    save!(Classic, "1.01", Classic, Necromancer, Starter),
    save!(Classic, "1.01", Classic, Paladin, Starter),
    save!(Classic, "1.01", Classic, Sorceress, Starter),
    // 1.02
    save!(Classic, "1.02", Classic, Amazon, Starter),
    save!(Classic, "1.02", Classic, Barbarian, Starter),
    save!(Classic, "1.02", Classic, Necromancer, Starter),
    save!(Classic, "1.02", Classic, Paladin, Starter),
    save!(Classic, "1.02", Classic, Sorceress, Starter),
    // 1.03
    save!(Classic, "1.03", Classic, Amazon, Starter),
    save!(Classic, "1.03", Classic, Barbarian, Starter),
    save!(Classic, "1.03", Classic, Necromancer, Starter),
    save!(Classic, "1.03", Classic, Paladin, Starter),
    save!(Classic, "1.03", Classic, Sorceress, Starter),
    // 1.04c
    save!(Classic, "1.04c", Classic, Amazon, Starter),
    save!(Classic, "1.04c", Classic, Barbarian, Starter),
    save!(Classic, "1.04c", Classic, Necromancer, Starter),
    save!(Classic, "1.04c", Classic, Paladin, Starter),
    save!(Classic, "1.04c", Classic, Sorceress, Starter),
    // 1.05b
    save!(Classic, "1.05b", Classic, Amazon, Starter),
    save!(Classic, "1.05b", Classic, Barbarian, Starter),
    save!(Classic, "1.05b", Classic, Necromancer, Starter),
    save!(Classic, "1.05b", Classic, Paladin, Starter),
    save!(Classic, "1.05b", Classic, Sorceress, Starter),
    // 1.06b (last patch before LoD release)
    save!(Classic, "1.06b", Classic, Amazon, Starter),
    save!(Classic, "1.06b", Classic, Barbarian, Starter),
    save!(Classic, "1.06b", Classic, Necromancer, Starter),
    save!(Classic, "1.06b", Classic, Paladin, Starter),
    save!(Classic, "1.06b", Classic, Sorceress, Starter),
    // 1.07
    save!(ClassicLoD, "1.07", Classic, Amazon, Starter),
    save!(ClassicLoD, "1.07", Classic, Barbarian, Starter),
    save!(ClassicLoD, "1.07", Classic, Necromancer, Starter),
    save!(ClassicLoD, "1.07", Classic, Paladin, Starter),
    save!(ClassicLoD, "1.07", Classic, Sorceress, Starter),
    save!(ClassicLoD, "1.07", LoD, Amazon, Starter),
    save!(ClassicLoD, "1.07", LoD, Barbarian, Starter),
    save!(ClassicLoD, "1.07", LoD, Necromancer, Starter),
    save!(ClassicLoD, "1.07", LoD, Paladin, Starter),
    save!(ClassicLoD, "1.07", LoD, Sorceress, Starter),
    save!(ClassicLoD, "1.07", LoD, Assassin, Starter),
    save!(ClassicLoD, "1.07", LoD, Druid, Starter),
    // 1.08
    save!(ClassicLoD, "1.08", Classic, Amazon, Starter),
    save!(ClassicLoD, "1.08", Classic, Barbarian, Starter),
    save!(ClassicLoD, "1.08", Classic, Necromancer, Starter),
    save!(ClassicLoD, "1.08", Classic, Paladin, Starter),
    save!(ClassicLoD, "1.08", Classic, Sorceress, Starter),
    save!(ClassicLoD, "1.08", LoD, Amazon, Starter),
    save!(ClassicLoD, "1.08", LoD, Barbarian, Starter),
    save!(ClassicLoD, "1.08", LoD, Necromancer, Starter),
    save!(ClassicLoD, "1.08", LoD, Paladin, Starter),
    save!(ClassicLoD, "1.08", LoD, Sorceress, Starter),
    save!(ClassicLoD, "1.08", LoD, Assassin, Starter),
    save!(ClassicLoD, "1.08", LoD, Druid, Starter),
    // 1.09d
    save!(ClassicLoD, "1.09d", Classic, Amazon, Starter),
    save!(ClassicLoD, "1.09d", Classic, Barbarian, Starter),
    save!(ClassicLoD, "1.09d", Classic, Necromancer, Starter),
    save!(ClassicLoD, "1.09d", Classic, Paladin, Starter),
    save!(ClassicLoD, "1.09d", Classic, Sorceress, Starter),
    save!(ClassicLoD, "1.09d", LoD, Amazon, Starter),
    save!(ClassicLoD, "1.09d", LoD, Barbarian, Starter),
    save!(ClassicLoD, "1.09d", LoD, Necromancer, Starter),
    save!(ClassicLoD, "1.09d", LoD, Paladin, Starter),
    save!(ClassicLoD, "1.09d", LoD, Sorceress, Starter),
    save!(ClassicLoD, "1.09d", LoD, Assassin, Starter),
    save!(ClassicLoD, "1.09d", LoD, Druid, Starter),
    // // 1.10
    save!(ClassicLoD, "1.10", Classic, Amazon, Starter),
    save!(ClassicLoD, "1.10", Classic, Barbarian, Starter),
    save!(ClassicLoD, "1.10", Classic, Necromancer, Starter),
    save!(ClassicLoD, "1.10", Classic, Paladin, Starter),
    save!(ClassicLoD, "1.10", Classic, Sorceress, Starter),
    save!(ClassicLoD, "1.10", LoD, Amazon, Starter),
    save!(ClassicLoD, "1.10", LoD, Barbarian, Starter),
    save!(ClassicLoD, "1.10", LoD, Necromancer, Starter),
    save!(ClassicLoD, "1.10", LoD, Paladin, Starter),
    save!(ClassicLoD, "1.10", LoD, Sorceress, Starter),
    save!(ClassicLoD, "1.10", LoD, Assassin, Starter),
    save!(ClassicLoD, "1.10", LoD, Druid, Starter),
    // // 1.11b
    save!(ClassicLoD, "1.11b", Classic, Amazon, Starter),
    save!(ClassicLoD, "1.11b", Classic, Barbarian, Starter),
    save!(ClassicLoD, "1.11b", Classic, Necromancer, Starter),
    save!(ClassicLoD, "1.11b", Classic, Paladin, Starter),
    save!(ClassicLoD, "1.11b", Classic, Sorceress, Starter),
    save!(ClassicLoD, "1.11b", LoD, Amazon, Starter),
    save!(ClassicLoD, "1.11b", LoD, Barbarian, Starter),
    save!(ClassicLoD, "1.11b", LoD, Necromancer, Starter),
    save!(ClassicLoD, "1.11b", LoD, Paladin, Starter),
    save!(ClassicLoD, "1.11b", LoD, Sorceress, Starter),
    save!(ClassicLoD, "1.11b", LoD, Assassin, Starter),
    save!(ClassicLoD, "1.11b", LoD, Druid, Starter),
    // // 1.12a
    save!(ClassicLoD, "1.12a", Classic, Amazon, Starter),
    save!(ClassicLoD, "1.12a", Classic, Barbarian, Starter),
    save!(ClassicLoD, "1.12a", Classic, Necromancer, Starter),
    save!(ClassicLoD, "1.12a", Classic, Paladin, Starter),
    save!(ClassicLoD, "1.12a", Classic, Sorceress, Starter),
    save!(ClassicLoD, "1.12a", LoD, Amazon, Starter),
    save!(ClassicLoD, "1.12a", LoD, Barbarian, Starter),
    save!(ClassicLoD, "1.12a", LoD, Necromancer, Starter),
    save!(ClassicLoD, "1.12a", LoD, Paladin, Starter),
    save!(ClassicLoD, "1.12a", LoD, Sorceress, Starter),
    save!(ClassicLoD, "1.12a", LoD, Assassin, Starter),
    save!(ClassicLoD, "1.12a", LoD, Druid, Starter),
    // // 1.13d
    save!(ClassicLoD, "1.13d", Classic, Amazon, Starter),
    save!(ClassicLoD, "1.13d", Classic, Barbarian, Starter),
    save!(ClassicLoD, "1.13d", Classic, Necromancer, Starter),
    save!(ClassicLoD, "1.13d", Classic, Paladin, Starter),
    save!(ClassicLoD, "1.13d", Classic, Sorceress, Starter),
    save!(ClassicLoD, "1.13d", LoD, Amazon, Starter),
    save!(ClassicLoD, "1.13d", LoD, Barbarian, Starter),
    save!(ClassicLoD, "1.13d", LoD, Necromancer, Starter),
    save!(ClassicLoD, "1.13d", LoD, Paladin, Starter),
    save!(ClassicLoD, "1.13d", LoD, Sorceress, Starter),
    save!(ClassicLoD, "1.13d", LoD, Assassin, Starter),
    save!(ClassicLoD, "1.13d", LoD, Druid, Starter),
    // // 1.14b
    save!(ClassicLoD, "1.14b", Classic, Amazon, Starter),
    save!(ClassicLoD, "1.14b", Classic, Barbarian, Starter),
    save!(ClassicLoD, "1.14b", Classic, Necromancer, Starter),
    save!(ClassicLoD, "1.14b", Classic, Paladin, Starter),
    save!(ClassicLoD, "1.14b", Classic, Sorceress, Starter),
    save!(ClassicLoD, "1.14b", LoD, Amazon, Starter),
    save!(ClassicLoD, "1.14b", LoD, Barbarian, Starter),
    save!(ClassicLoD, "1.14b", LoD, Necromancer, Starter),
    save!(ClassicLoD, "1.14b", LoD, Paladin, Starter),
    save!(ClassicLoD, "1.14b", LoD, Sorceress, Starter),
    save!(ClassicLoD, "1.14b", LoD, Assassin, Starter),
    save!(ClassicLoD, "1.14b", LoD, Druid, Starter),
    // // 1.14d (latest Classic LoD)
    save!(ClassicLoD, "1.14d", Classic, Amazon, Starter),
    save!(ClassicLoD, "1.14d", Classic, Barbarian, Starter),
    save!(ClassicLoD, "1.14d", Classic, Necromancer, Starter),
    save!(ClassicLoD, "1.14d", Classic, Paladin, Starter),
    save!(ClassicLoD, "1.14d", Classic, Sorceress, Starter),
    save!(ClassicLoD, "1.14d", LoD, Amazon, Starter),
    save!(ClassicLoD, "1.14d", LoD, Barbarian, Starter),
    save!(ClassicLoD, "1.14d", LoD, Necromancer, Starter),
    save!(ClassicLoD, "1.14d", LoD, Paladin, Starter),
    save!(ClassicLoD, "1.14d", LoD, Sorceress, Starter),
    save!(ClassicLoD, "1.14d", LoD, Assassin, Starter),
    save!(ClassicLoD, "1.14d", LoD, Druid, Starter),
])]
fn starters_correctly_load_and_save(file: TestSave) -> Result<(), ReadCharacterSaveError> {
    verify_save(file)
}

#[cfg(not(feature = "strict"))]
#[test_matrix([
    // // PD2 2.13.0 (Season 12)
    save!(ProjectDiablo2, "2.13.0", LoD, Amazon, Starter),
    save!(ProjectDiablo2, "2.13.0", LoD, Barbarian, Starter),
    save!(ProjectDiablo2, "2.13.0", LoD, Necromancer, Starter),
    save!(ProjectDiablo2, "2.13.0", LoD, Paladin, Starter),
    save!(ProjectDiablo2, "2.13.0", LoD, Sorceress, Starter),
    save!(ProjectDiablo2, "2.13.0", LoD, Assassin, Starter),
    save!(ProjectDiablo2, "2.13.0", LoD, Druid, Starter),
])]
fn modded_starters_correctly_load_and_save(file: TestSave) -> Result<(), ReadCharacterSaveError> {
    verify_save(file)
}

fn verify_save(file: TestSave) -> Result<(), ReadCharacterSaveError> {
    let save = CharacterSave::read(file.bytes)?;

    assert_eq!(
        file.class, save.character.class,
        "{}: Character class mismatch (expected: {}, actual: {})",
        file.filename, file.class, save.character.class
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

    Ok(())
    // TODO:
    // let output = match save.write() {
    //     Ok(o) => o,
    //     Err(e) => panic!("{}: Cannot write save file ({})", file.filename, e),
    // };
    // assert_eq!(file.bytes, output.as_slice());
}
