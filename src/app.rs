#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Character {
    Mario,
    DonkeyKong,
    Link,
    Samus,
    Yoshi,
    Kirby,
    Fox,
    Pikachu,
}

impl Character {
    pub const ALL: [Self; 8] = [
        Self::Mario,
        Self::DonkeyKong,
        Self::Link,
        Self::Samus,
        Self::Yoshi,
        Self::Kirby,
        Self::Fox,
        Self::Pikachu,
    ];

    pub fn name(self) -> &'static str {
        match self {
            Self::Mario => "MARIO",
            Self::DonkeyKong => "DONKEY KONG",
            Self::Link => "LINK",
            Self::Samus => "SAMUS",
            Self::Yoshi => "YOSHI",
            Self::Kirby => "KIRBY",
            Self::Fox => "FOX",
            Self::Pikachu => "PIKACHU",
        }
    }

    pub fn accent(self) -> u32 {
        match self {
            Self::Mario => 0xD93636,
            Self::DonkeyKong => 0x8D4A2F,
            Self::Link => 0x4F9B62,
            Self::Samus => 0xC95B72,
            Self::Yoshi => 0x66B95D,
            Self::Kirby => 0xD97AC5,
            Self::Fox => 0xB78345,
            Self::Pikachu => 0xE4C832,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AppState {
    Title,
    CharacterSelect,
    Diorama(Character),
}
