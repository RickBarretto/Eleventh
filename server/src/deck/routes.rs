use std::collections::HashMap;
use std::sync::Arc;

use rand::rngs::StdRng;
use rand::SeedableRng;
use serde::Deserialize;
use serde_json::json;
use tokio::sync::Mutex;

use quickapi::{Response, Server};

use super::repository::Inventories;
use super::services::claim::Rewarding;
use crate::services::Services;

#[derive(Debug, Deserialize)]
pub struct ClaimBody {
    pub auth: String,
}

#[derive(Debug, Deserialize)]
pub struct Login {
    pub username: String,
    pub password: String,
}

async fn claim(
    rewards: Arc<Mutex<Rewarding>>,
    inventories: Arc<Mutex<Inventories>>,
    params: HashMap<String, String>,
) -> Response {
    let id = params.get("id").expect("To have ID parameter");

    match rewards
        .lock()
        .await
        .claim_reward(id, StdRng::from_os_rng())
        .await
    {
        Err(e) => Response::bad_request().json(&json!({
            "message": "Could not claim reward!",
            "error": e,
        })),
        Ok(claimed) => {
            let mut inventories = inventories.lock().await;
            let inventory = inventories.deck_of(id).await;
            inventory.add_deck(claimed.clone()).await;

            Response::ok().json(&json!({
                "message": "You got new cards!",
                "players": &claimed.players().await,
                "power_ups": &claimed.power_ups().await,
            }))
        }
    }
}

async fn fire(inventories: Arc<Mutex<Inventories>>, params: HashMap<String, String>) -> Response {
    let mut inventories = inventories.lock().await;

    let user_id = params.get("id").expect("Have user ID as parameter.");
    let card_index: usize = params
        .get("index")
        .expect("Have index of the card to be fired.")
        .parse::<usize>()
        .expect("Expected to be a natural number");

    let inventory = inventories.deck_of(user_id).await;

    match inventory.fire(card_index).await {
        Some(_) => Response::ok().json(&json!({
            "message": "Card removed from deck",
        })),
        None => Response::bad_request().json(&json!({
            "message": "Could not remove from deck.",
            "error": "index out of bounds",
        })),
    }
}

async fn user_deck(
    inventories: Arc<Mutex<Inventories>>,
    params: HashMap<String, String>,
) -> Response {
    let mut inventories = inventories.lock().await;

    let user_id = params.get("id").expect("Have user ID as parameter.");

    let deck = inventories.deck_of(user_id).await;
    let players = deck.players().await;
    let power_ups = deck.power_ups().await;

    Response::ok().json(&json!({
        "message": "Your deck",
        "players": players,
        "power_ups": power_ups,
    }))
}

pub fn route_decks(app: &mut Server<Services>) {
    let services = app.services.clone();
    app.get("/user/{id}/deck/claim/", move |_, params| {
        let rewards = services.rewarding();
        let inventories = services.inventories();
        claim(rewards, inventories, params)
    });

    let services = app.services.clone();
    app.delete("/user/{id}/deck/fire/{index}", move |_, params| {
        let inventories = services.inventories();
        fire(inventories, params)
    });

    let services = app.services.clone();
    app.get("/user/{id}/deck/", move |_, params| {
        let inventories = services.inventories();
        user_deck(inventories, params)
    });
}
