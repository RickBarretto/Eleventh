use rand::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::models::cards::{PlayerCard, SpecialCard};

type Amount = usize;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Deck {
    players: Vec<PlayerCard>,
    power_ups: HashMap<SpecialCard, Amount>,
}

impl Deck {
    pub fn new(players: &[PlayerCard], power_ups: &[(SpecialCard, Amount)]) -> Self {
        Self {
            players: players.to_vec(),
            power_ups: power_ups.iter().cloned().collect(),
        }
    }

    pub async fn players(&self) -> Vec<PlayerCard> {
        self.players.clone()
    }

    pub async fn power_ups(&self) -> Vec<(SpecialCard, Amount)> {
        self.power_ups
            .iter()
            .map(|(card, amount)| (card.clone(), *amount))
            .collect()
    }

    pub fn random(mut rng: impl Rng) -> Deck {
        const GOALKEEPERS: std::ops::Range<i32> = 0..1;
        const DEFENDERS: std::ops::Range<i32> = 0..4;
        const MIDFIELDERS: std::ops::Range<i32> = 0..4;
        const FORWARDS: std::ops::Range<i32> = 0..2;
        const SPECIAL_CARDS: std::ops::Range<i32> = 0..3;

        let mut player_cards: Vec<PlayerCard> = Vec::new();
        player_cards.extend(GOALKEEPERS.map(|_| PlayerCard::random(&mut rng).is("GK")));
        player_cards.extend(DEFENDERS.map(|_| PlayerCard::random(&mut rng).is("DEF")));
        player_cards.extend(MIDFIELDERS.map(|_| PlayerCard::random(&mut rng).is("MID")));
        player_cards.extend(FORWARDS.map(|_| PlayerCard::random(&mut rng).is("FWD")));

        let mut power_ups: HashMap<SpecialCard, Amount> = HashMap::new();
        for _ in SPECIAL_CARDS {
            let card = SpecialCard::random(&mut rng);
            *power_ups.entry(card).or_insert(0) += 1;
        }

        Deck {
            players: player_cards,
            power_ups,
        }
    }

    pub fn join(&mut self, other: Self) {
        self.players.extend(other.players);
        for (card, amount) in other.power_ups {
            *self.power_ups.entry(card).or_insert(0) += amount;
        }
    }
}

#[derive(Debug, Clone)]
pub struct Inventory {
    deck: Deck,
    limit: Amount,
}

impl Default for Inventory {
    fn default() -> Self {
        Self {
            limit: 50,
            deck: Deck::new(&[], &[]),
        }
    }
}

impl Inventory {
    pub fn new(limit: usize) -> Self {
        Self {
            limit,
            ..Default::default()
        }
    }

    pub async fn add_deck(&mut self, deck: Deck) {
        self.deck.join(deck);
        if self.deck.players.len() > self.limit {
            let _ = self.deck.players.split_off(50);
        }
    }

    pub async fn players(&self) -> Vec<PlayerCard> {
        self.deck.players().await
    }

    pub async fn power_ups(&self) -> Vec<(SpecialCard, Amount)> {
        self.deck.power_ups().await
    }

    pub async fn fire(&mut self, index: usize) -> Option<PlayerCard> {
        if index < self.deck.players.len() {
            Some(self.deck.players.remove(index))
        } else {
            None
        }
    }
}
