use tyrael::character::CharacterClass;

#[allow(unused)]
pub struct TestSave {
    pub bytes: &'static [u8],
    pub filename: &'static str,
    pub game: TestSaveGame,
    pub version: &'static str,
    pub expansion: TestSaveCharacterExpansion,
    pub class: CharacterClass,
    pub stage: TestSaveCharacterStage,
    pub name: Option<&'static str>,
}

#[allow(unused)]
#[derive(PartialEq)]
pub enum TestSaveGame {
    // Official
    Classic,
    ClassicLoD,
    // Modded
    ProjectDiablo2,
}

#[allow(unused)]
#[derive(PartialEq)]
pub enum TestSaveCharacterExpansion {
    Classic,
    LoD,
}

#[allow(unused)]
#[derive(PartialEq)]
pub enum TestSaveCharacterStage {
    Starter,
    Normal,
}

#[macro_export]
macro_rules! save {
    ($game:ident, $version:literal, $expansion:ident, $class:ident, $stage:ident) => {
        $crate::common::TestSave {
            bytes: include_bytes!(concat!(
                "saves/",
                stringify!($game),
                "/",
                $version,
                "/",
                stringify!($expansion),
                "_",
                stringify!($class),
                "/",
                stringify!($stage),
                ".d2s"
            )),
            filename: concat!(
                stringify!($game),
                "/",
                $version,
                "/",
                stringify!($expansion),
                "_",
                stringify!($class),
                "/",
                stringify!($stage),
                ".d2s"
            ),
            game: $crate::common::TestSaveGame::$game,
            version: $version,
            expansion: $crate::common::TestSaveCharacterExpansion::$expansion,
            class: tyrael::character::CharacterClass::$class,
            stage: $crate::common::TestSaveCharacterStage::$stage,
            name: None,
        }
    };

    ($game:ident, $version:literal, $expansion:ident, $class:ident, $stage:ident, $name:literal) => {
        $crate::common::TestSave {
            bytes: include_bytes!(concat!(
                "saves/",
                stringify!($game),
                "/",
                $version,
                "/",
                stringify!($expansion),
                "_",
                stringify!($class),
                "/",
                stringify!($stage),
                "_",
                $name,
                ".d2s"
            )),
            filename: concat!(
                stringify!($game),
                "/",
                $version,
                "/",
                stringify!($expansion),
                "_",
                stringify!($class),
                "/",
                stringify!($stage),
                "_",
                $name,
                ".d2s"
            ),
            game: $crate::common::TestSaveGame::$game,
            version: $version,
            expansion: $crate::common::TestSaveCharacterExpansion::$expansion,
            class: tyrael::character::CharacterClass::$class,
            stage: $crate::common::TestSaveCharacterStage::$stage,
            name: Some($name),
        }
    };
}
