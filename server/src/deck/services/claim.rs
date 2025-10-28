use chrono::{DateTime, Duration, NaiveDate, Utc};
use rand::{rngs::StdRng, Rng};
use std::{collections::HashMap, sync::Arc};
use tokio::sync::Mutex;

use crate::deck::models::Deck;

#[derive(Debug, Clone)]
pub struct DailyDecks {
    decks: Vec<Deck>,
}

impl DailyDecks {
    pub fn new(mut rng: impl Rng) -> Self {
        Self {
            decks: Self::new_decks(&mut rng),
        }
    }

    pub async fn regerate(&mut self, mut rng: impl Rng) {
        self.decks = Self::new_decks(&mut rng)
    }

    pub async fn decks(self) -> Vec<Deck> {
        self.decks.clone()
    }

    pub async fn reward(&mut self) -> Option<Deck> {
        self.decks.pop()
    }

    fn new_decks(mut rng: impl Rng) -> Vec<Deck> {
        (0..256).map(|_| Deck::random(&mut rng)).collect()
    }
}

/// Tracks when a user last claimed a reward
#[derive(Debug, Clone)]
pub struct PlayerClaim {
    claimed_at: Option<DateTime<Utc>>,
}

/// The reward service manages daily decks and per-user rewards
#[derive(Debug, Clone)]
pub struct Rewarding {
    daily_decks: Arc<Mutex<DailyDecks>>,
    refreshed_at: NaiveDate,
    claims: HashMap<String, PlayerClaim>,
}

impl Rewarding {
    pub fn new(mut rng: StdRng) -> Self {
        let today = Utc::now().date_naive();
        Self {
            daily_decks: Mutex::new(DailyDecks::new(&mut rng)).into(),
            refreshed_at: today,
            claims: HashMap::new(),
        }
    }

    /// Claim a reward for a user (once per 24h).
    pub async fn claim_reward(
        &mut self,
        user_id: &str,
        mut rng: StdRng,
    ) -> Result<Deck, &'static str> {
        let now = Utc::now();
        let today = now.date_naive();

        // Refresh deck if midnight passed
        if today > self.refreshed_at {
            self.force_refresh(&mut rng).await;
        }

        let player_state = self
            .claims
            .entry(user_id.to_string())
            .or_insert(PlayerClaim { claimed_at: None });

        if let Some(last) = player_state.claimed_at {
            if now.signed_duration_since(last) < Duration::hours(24) {
                return Err("Reward already claimed in the last 24h");
            }
        }

        if let Some(deck) = self.daily_decks.lock().await.reward().await {
            player_state.claimed_at = Some(now);
            Ok(deck)
        } else {
            Err("No rewards available today. Try again in 24h!")
        }
    }

    /// Force deck refresh.
    pub async fn force_refresh(&mut self, mut rng: impl Rng) {
        self.daily_decks.lock().await.regerate(&mut rng).await;
        self.refreshed_at = Utc::now().date_naive();
    }
}
