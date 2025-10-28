use std::fmt;
use std::fmt::Display;

use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Deserialize, Clone)]
pub struct DeckResponse {
    pub players: Vec<Player>,
    pub power_ups: Vec<(PowerUp, u32)>,

    #[allow(dead_code, reason = "needed for deserialization purposes.")]
    message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Player {
    name: String,
    position: String,
    attack: u32,
    defense: u32,
    passing: u32,
    stamina: u32,
}

impl Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} ({}) | ATK: {} | DEF: {} | PASS: {} | STA: {}",
            self.name, self.position, self.attack, self.defense, self.passing, self.stamina
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PowerUp {
    name: String,
    effect: String,
}

impl Display for PowerUp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.name, self.effect)
    }
}
