use serde::Serialize;

use crate::matches::models::matches::{Created, Finished, Paired};

#[derive(Clone)]
pub enum GameState {
    Created(Created),
    Paired(Paired),
    Finished(Finished),
}

#[derive(Serialize)]
pub struct GameResponse {
    status: String,
    host: Option<String>,
    guest: Option<String>,
    score: Option<(usize, usize)>,
    winner: Option<String>,
}

impl From<&GameState> for GameResponse {
    fn from(state: &GameState) -> Self {
        match state {
            GameState::Created(created) => GameResponse {
                status: "pairing".into(),
                host: Some(created.by.clone()),
                guest: None,
                score: None,
                winner: None,
            },
            GameState::Paired(paired) => {
                if paired.both_named() {
                    GameResponse {
                        status: "finished".into(),
                        host: Some(paired.host.clone()),
                        guest: Some(paired.guest.clone()),
                        score: Some(paired.run_match()),
                        winner: Some(if paired.run_match().0 > paired.run_match().1 {
                            paired.host.clone()
                        } else {
                            paired.guest.clone()
                        }),
                    }
                } else {
                    GameResponse {
                        status: "paired".into(),
                        host: Some(paired.host.clone()),
                        guest: Some(paired.guest.clone()),
                        score: None,
                        winner: None,
                    }
                }
            }
            GameState::Finished(finished) => GameResponse {
                status: "finished".into(),
                host: None,
                guest: None,
                score: Some(finished.score),
                winner: Some(finished.winner.clone()),
            },
        }
    }
}
