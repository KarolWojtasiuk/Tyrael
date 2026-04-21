mod saves;

use tyrael::CharacterSave;

use crate::saves::TestSaveGame;

#[test]
fn all_saves_are_correctly_load_and_saved() {
    // TODO: unfilter resurrected and pd2
    for file in saves::all()
        .iter()
        .filter(|s| s.game == TestSaveGame::Classic || s.game == TestSaveGame::ClassicLoD)
    {
        let save = match CharacterSave::read(file.bytes) {
            Ok(s) => s,
            Err(e) => panic!("{}: Cannot parse save file ({})", file.filename, e),
        };

        if file.class != save.character.class {
            panic!(
                "{}: Character class mismatch (expected: {}, actual: {})",
                file.filename, file.class, save.character.class
            );
        }
    }
}
