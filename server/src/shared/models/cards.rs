use rand::prelude::*;
use serde::{Deserialize, Serialize};

const PLAYER_NAMES: &[&str] = &[
    "Alex", "Jordan", "Chris", "Taylor", "Morgan", "Casey", "Drew", "Jamie", "Riley", "Cameron",
    "Sam", "Lee", "Pat", "Blake", "Dakota", "Quinn", "Avery", "Emerson", "Harper", "Reese",
];
const POSITIONS: &[&str] = &["GK", "DEF", "MID", "FWD"];
const SPECIAL_CARD_NAMES: &[(&str, &str)] = &[
    ("Counter-Attack", "Launch instant play if opponent fails"),
    ("Injury", "Reduce opponent player's stats"),
    ("Super Save", "Boost GK for one turn"),
    ("Set Piece", "Free Kick/Corner with bonus"),
    ("Adrenaline Rush", "Boost stamina"),
    ("Tactical Shift", "Change formation mid-turn"),
    ("Red Card", "Remove opponent player for turn"),
    ("Dribble Mastery", "Increase attack chance"),
];

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PlayerCard {
    pub name: String,
    pub position: String,
    pub attack: u8,
    pub defense: u8,
    pub passing: u8,
    pub stamina: u8,
}

impl PlayerCard {
    pub fn random<R: Rng>(rng: &mut R) -> Self {
        let name = PLAYER_NAMES.choose(rng).unwrap_or(&"John Doe").to_string();

        let position = POSITIONS.choose(rng).unwrap_or(&"MID").to_string();

        PlayerCard {
            name,
            position,
            attack: Self::random_stats(rng),
            defense: Self::random_stats(rng),
            passing: Self::random_stats(rng),
            stamina: Self::random_stats(rng),
        }
    }

    pub fn is(mut self, position: &str) -> Self {
        self.position = position.into();
        self
    }

    fn random_stats<R: Rng>(rng: &mut R) -> u8 {
        const MAXIMUM: u8 = 100;
        const MINIMUM: u8 = 50;
        rng.random_range(MINIMUM..=MAXIMUM)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct SpecialCard {
    pub name: String,
    pub effect: String,
}

impl SpecialCard {
    pub fn random<R: Rng>(rng: &mut R) -> Self {
        let (name, effect) = SPECIAL_CARD_NAMES
            .choose(rng)
            .unwrap_or(&SPECIAL_CARD_NAMES[0]);

        SpecialCard {
            name: name.to_string(),
            effect: effect.to_string(),
        }
    }
}
