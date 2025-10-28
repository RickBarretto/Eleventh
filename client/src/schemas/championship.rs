use serde::Deserialize;
use serde::Serialize;

use super::deck::Player;
use super::deck::PowerUp;

#[derive(Deserialize)]
pub struct MatchState {
    pub status: String,
    pub host: Option<String>,
    pub guest: Option<String>,
    pub score: Option<(usize, usize)>,
    pub winner: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct Team {
    pub named: Vec<Player>,
    pub helper: PowerUp,
}
