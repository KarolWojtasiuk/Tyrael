macro_rules! save {
    ($game:ident, $version:literal, $expansion:ident, $class:ident, $name:literal) => {
        paste::paste! {
            pub const [<$expansion:upper _ $class:upper _ $name:upper>]: $crate::tests::TestSave = $crate::tests::TestSave {
                bytes: include_bytes!(concat!(
                    stringify!($game),
                    "/",
                    $version,
                    "/",
                    stringify!($expansion),
                    "_",
                    stringify!($class),
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
                    "/",
                    stringify!($class),
                    "_",
                    $name,
                    ".d2s"
                ),
                game: $crate::tests::TestSaveGame::$game,
                version: $version,
                expansion: $crate::tests::TestSaveExpansion::$expansion,
                class: $crate::tests::TestSaveClass::$class,
                name: $name,
            };
        }
    };
}

/// Diablo 2 v1.00 - v1.06b
pub mod classic {
    /// - Game: Diablo 2 v1.00
    /// - Save: v71
    pub mod v1_00 {
        save!(Classic, "1.00", Classic, Amazon, "Starter");
        save!(Classic, "1.00", Classic, Barbarian, "Starter");
        save!(Classic, "1.00", Classic, Necromancer, "Starter");
        save!(Classic, "1.00", Classic, Paladin, "Starter");
        save!(Classic, "1.00", Classic, Sorceress, "Starter");
    }

    /// - Game: Diablo 2 v1.01
    /// - Save: v71
    pub mod v1_01 {
        save!(Classic, "1.01", Classic, Amazon, "Starter");
        save!(Classic, "1.01", Classic, Barbarian, "Starter");
        save!(Classic, "1.01", Classic, Necromancer, "Starter");
        save!(Classic, "1.01", Classic, Paladin, "Starter");
        save!(Classic, "1.01", Classic, Sorceress, "Starter");
    }

    /// - Game: Diablo 2 v1.02
    /// - Save: v71
    pub mod v1_02 {
        save!(Classic, "1.02", Classic, Amazon, "Starter");
        save!(Classic, "1.02", Classic, Barbarian, "Starter");
        save!(Classic, "1.02", Classic, Necromancer, "Starter");
        save!(Classic, "1.02", Classic, Paladin, "Starter");
        save!(Classic, "1.02", Classic, Sorceress, "Starter");
    }

    /// - Game: Diablo 2 v1.03
    /// - Save: v71
    pub mod v1_03 {
        save!(Classic, "1.03", Classic, Amazon, "Starter");
        save!(Classic, "1.03", Classic, Barbarian, "Starter");
        save!(Classic, "1.03", Classic, Necromancer, "Starter");
        save!(Classic, "1.03", Classic, Paladin, "Starter");
        save!(Classic, "1.03", Classic, Sorceress, "Starter");
    }

    /// - Game: Diablo 2 v1.04c
    /// - Save: v71
    pub mod v1_04c {
        save!(Classic, "1.04c", Classic, Amazon, "Starter");
        save!(Classic, "1.04c", Classic, Barbarian, "Starter");
        save!(Classic, "1.04c", Classic, Necromancer, "Starter");
        save!(Classic, "1.04c", Classic, Paladin, "Starter");
        save!(Classic, "1.04c", Classic, Sorceress, "Starter");
    }

    /// - Game: Diablo 2 v1.05b
    /// - Save: v71
    pub mod v1_05b {
        save!(Classic, "1.05b", Classic, Amazon, "Starter");
        save!(Classic, "1.05b", Classic, Barbarian, "Starter");
        save!(Classic, "1.05b", Classic, Necromancer, "Starter");
        save!(Classic, "1.05b", Classic, Paladin, "Starter");
        save!(Classic, "1.05b", Classic, Sorceress, "Starter");
    }

    /// - Game: Diablo 2 v1.06b (latest)
    /// - Save: v71
    pub mod v1_06b {
        save!(Classic, "1.06b", Classic, Amazon, "Starter");
        save!(Classic, "1.06b", Classic, Barbarian, "Starter");
        save!(Classic, "1.06b", Classic, Necromancer, "Starter");
        save!(Classic, "1.06b", Classic, Paladin, "Starter");
        save!(Classic, "1.06b", Classic, Sorceress, "Starter");
    }
}

/// Diablo 2 LoD v1.07 - v1.14d
pub mod classic_lod {
    /// - Game: Diablo 2 v1.07
    /// - Save: v87
    pub mod v1_07 {
        save!(ClassicLoD, "1.07", Classic, Amazon, "Starter");
        save!(ClassicLoD, "1.07", Classic, Barbarian, "Starter");
        save!(ClassicLoD, "1.07", Classic, Necromancer, "Starter");
        save!(ClassicLoD, "1.07", Classic, Paladin, "Starter");
        save!(ClassicLoD, "1.07", Classic, Sorceress, "Starter");
        save!(ClassicLoD, "1.07", LoD, Amazon, "Starter");
        save!(ClassicLoD, "1.07", LoD, Barbarian, "Starter");
        save!(ClassicLoD, "1.07", LoD, Necromancer, "Starter");
        save!(ClassicLoD, "1.07", LoD, Paladin, "Starter");
        save!(ClassicLoD, "1.07", LoD, Sorceress, "Starter");
        save!(ClassicLoD, "1.07", LoD, Assassin, "Starter");
        save!(ClassicLoD, "1.07", LoD, Druid, "Starter");
    }

    /// - Game: Diablo 2 v1.08
    /// - Save: v89
    pub mod v1_08 {
        save!(ClassicLoD, "1.08", Classic, Amazon, "Starter");
        save!(ClassicLoD, "1.08", Classic, Barbarian, "Starter");
        save!(ClassicLoD, "1.08", Classic, Necromancer, "Starter");
        save!(ClassicLoD, "1.08", Classic, Paladin, "Starter");
        save!(ClassicLoD, "1.08", Classic, Sorceress, "Starter");
        save!(ClassicLoD, "1.08", LoD, Amazon, "Starter");
        save!(ClassicLoD, "1.08", LoD, Barbarian, "Starter");
        save!(ClassicLoD, "1.08", LoD, Necromancer, "Starter");
        save!(ClassicLoD, "1.08", LoD, Paladin, "Starter");
        save!(ClassicLoD, "1.08", LoD, Sorceress, "Starter");
        save!(ClassicLoD, "1.08", LoD, Assassin, "Starter");
        save!(ClassicLoD, "1.08", LoD, Druid, "Starter");
    }

    /// - Game: Diablo 2 v1.09d
    /// - Save: v92
    pub mod v1_09d {
        save!(ClassicLoD, "1.09d", Classic, Amazon, "Starter");
        save!(ClassicLoD, "1.09d", Classic, Barbarian, "Starter");
        save!(ClassicLoD, "1.09d", Classic, Necromancer, "Starter");
        save!(ClassicLoD, "1.09d", Classic, Paladin, "Starter");
        save!(ClassicLoD, "1.09d", Classic, Sorceress, "Starter");
        save!(ClassicLoD, "1.09d", LoD, Amazon, "Starter");
        save!(ClassicLoD, "1.09d", LoD, Barbarian, "Starter");
        save!(ClassicLoD, "1.09d", LoD, Necromancer, "Starter");
        save!(ClassicLoD, "1.09d", LoD, Paladin, "Starter");
        save!(ClassicLoD, "1.09d", LoD, Sorceress, "Starter");
        save!(ClassicLoD, "1.09d", LoD, Assassin, "Starter");
        save!(ClassicLoD, "1.09d", LoD, Druid, "Starter");
    }

    /// - Game: Diablo 2 v1.10
    /// - Save: v96
    pub mod v1_10 {
        save!(ClassicLoD, "1.10", Classic, Amazon, "Starter");
        save!(ClassicLoD, "1.10", Classic, Barbarian, "Starter");
        save!(ClassicLoD, "1.10", Classic, Necromancer, "Starter");
        save!(ClassicLoD, "1.10", Classic, Paladin, "Starter");
        save!(ClassicLoD, "1.10", Classic, Sorceress, "Starter");
        save!(ClassicLoD, "1.10", LoD, Amazon, "Starter");
        save!(ClassicLoD, "1.10", LoD, Barbarian, "Starter");
        save!(ClassicLoD, "1.10", LoD, Necromancer, "Starter");
        save!(ClassicLoD, "1.10", LoD, Paladin, "Starter");
        save!(ClassicLoD, "1.10", LoD, Sorceress, "Starter");
        save!(ClassicLoD, "1.10", LoD, Assassin, "Starter");
        save!(ClassicLoD, "1.10", LoD, Druid, "Starter");
    }

    /// - Game: Diablo 2 v1.11b
    /// - Save: v96
    pub mod v1_11b {
        save!(ClassicLoD, "1.11b", Classic, Amazon, "Starter");
        save!(ClassicLoD, "1.11b", Classic, Barbarian, "Starter");
        save!(ClassicLoD, "1.11b", Classic, Necromancer, "Starter");
        save!(ClassicLoD, "1.11b", Classic, Paladin, "Starter");
        save!(ClassicLoD, "1.11b", Classic, Sorceress, "Starter");
        save!(ClassicLoD, "1.11b", LoD, Amazon, "Starter");
        save!(ClassicLoD, "1.11b", LoD, Barbarian, "Starter");
        save!(ClassicLoD, "1.11b", LoD, Necromancer, "Starter");
        save!(ClassicLoD, "1.11b", LoD, Paladin, "Starter");
        save!(ClassicLoD, "1.11b", LoD, Sorceress, "Starter");
        save!(ClassicLoD, "1.11b", LoD, Assassin, "Starter");
        save!(ClassicLoD, "1.11b", LoD, Druid, "Starter");
    }

    /// - Game: Diablo 2 v1.12a
    /// - Save: v96
    pub mod v1_12a {
        save!(ClassicLoD, "1.12a", Classic, Amazon, "Starter");
        save!(ClassicLoD, "1.12a", Classic, Barbarian, "Starter");
        save!(ClassicLoD, "1.12a", Classic, Necromancer, "Starter");
        save!(ClassicLoD, "1.12a", Classic, Paladin, "Starter");
        save!(ClassicLoD, "1.12a", Classic, Sorceress, "Starter");
        save!(ClassicLoD, "1.12a", LoD, Amazon, "Starter");
        save!(ClassicLoD, "1.12a", LoD, Barbarian, "Starter");
        save!(ClassicLoD, "1.12a", LoD, Necromancer, "Starter");
        save!(ClassicLoD, "1.12a", LoD, Paladin, "Starter");
        save!(ClassicLoD, "1.12a", LoD, Sorceress, "Starter");
        save!(ClassicLoD, "1.12a", LoD, Assassin, "Starter");
        save!(ClassicLoD, "1.12a", LoD, Druid, "Starter");
    }

    /// - Game: Diablo 2 v1.13d
    /// - Save: v96
    pub mod v1_13d {
        save!(ClassicLoD, "1.13d", Classic, Amazon, "Starter");
        save!(ClassicLoD, "1.13d", Classic, Barbarian, "Starter");
        save!(ClassicLoD, "1.13d", Classic, Necromancer, "Starter");
        save!(ClassicLoD, "1.13d", Classic, Paladin, "Starter");
        save!(ClassicLoD, "1.13d", Classic, Sorceress, "Starter");
        save!(ClassicLoD, "1.13d", LoD, Amazon, "Starter");
        save!(ClassicLoD, "1.13d", LoD, Barbarian, "Starter");
        save!(ClassicLoD, "1.13d", LoD, Necromancer, "Starter");
        save!(ClassicLoD, "1.13d", LoD, Paladin, "Starter");
        save!(ClassicLoD, "1.13d", LoD, Sorceress, "Starter");
        save!(ClassicLoD, "1.13d", LoD, Assassin, "Starter");
        save!(ClassicLoD, "1.13d", LoD, Druid, "Starter");
    }

    /// - Game: Diablo 2 v1.14b
    /// - Save: v96
    pub mod v1_14b {
        save!(ClassicLoD, "1.14b", Classic, Amazon, "Starter");
        save!(ClassicLoD, "1.14b", Classic, Barbarian, "Starter");
        save!(ClassicLoD, "1.14b", Classic, Necromancer, "Starter");
        save!(ClassicLoD, "1.14b", Classic, Paladin, "Starter");
        save!(ClassicLoD, "1.14b", Classic, Sorceress, "Starter");
        save!(ClassicLoD, "1.14b", LoD, Amazon, "Starter");
        save!(ClassicLoD, "1.14b", LoD, Barbarian, "Starter");
        save!(ClassicLoD, "1.14b", LoD, Necromancer, "Starter");
        save!(ClassicLoD, "1.14b", LoD, Paladin, "Starter");
        save!(ClassicLoD, "1.14b", LoD, Sorceress, "Starter");
        save!(ClassicLoD, "1.14b", LoD, Assassin, "Starter");
        save!(ClassicLoD, "1.14b", LoD, Druid, "Starter");
    }

    /// - Game: Diablo 2 v1.14d
    /// - Save: v96
    pub mod v1_14d {
        save!(ClassicLoD, "1.14d", Classic, Amazon, "Starter");
        save!(ClassicLoD, "1.14d", Classic, Barbarian, "Starter");
        save!(ClassicLoD, "1.14d", Classic, Necromancer, "Starter");
        save!(ClassicLoD, "1.14d", Classic, Paladin, "Starter");
        save!(ClassicLoD, "1.14d", Classic, Sorceress, "Starter");
        save!(ClassicLoD, "1.14d", LoD, Amazon, "Starter");
        save!(ClassicLoD, "1.14d", LoD, Barbarian, "Starter");
        save!(ClassicLoD, "1.14d", LoD, Necromancer, "Starter");
        save!(ClassicLoD, "1.14d", LoD, Paladin, "Starter");
        save!(ClassicLoD, "1.14d", LoD, Sorceress, "Starter");
        save!(ClassicLoD, "1.14d", LoD, Assassin, "Starter");
        save!(ClassicLoD, "1.14d", LoD, Druid, "Starter");
    }
}
