use cursive::views::{Dialog, TextView};
use cursive::Cursive;
use serde::Deserialize;

use crate::services::server_url;
use quickapi::Client;

use super::MainMenu;

#[derive(Debug, Clone, Deserialize)]
struct Player {
    name: String,
    position: String,
    attack: u32,
    defense: u32,
    passing: u32,
    stamina: u32,
}

#[derive(Debug, Clone, Deserialize)]
struct PowerUp {
    name: String,
    effect: String,
}

#[derive(Debug, Deserialize)]
struct RewardResponse {
    message: String,
    players: Option<Vec<Player>>,
    power_ups: Option<Vec<(PowerUp, u32)>>, // (powerup, amount)
    error: Option<String>,
}

fn reward_path(user: &str) -> String {
    format!("user/{}/deck/claim", user)
}

#[allow(non_snake_case)]
pub fn RewardScreen(app: &mut Cursive, auth: String) {
    app.pop_layer();

    let client = Client::new(&server_url());
    let path = reward_path(&auth);
    let response = client.get(&path);

    if response.status >= 400 {
        let error_msg = match serde_json::from_str::<serde_json::Value>(&response.body) {
            Ok(val) => val.get("error").and_then(|e| e.as_str()).unwrap_or("Unknown error").to_string(),
            Err(_) => "Unknown error".to_string(),
        };
        return ErrorDialog(app, auth, error_msg);
    }

    let reward = serde_json::from_str::<RewardResponse>(&response.body);

    if reward.is_err() {
        return ErrorDialog(app, auth.clone(), "Failed to parse server response".into());
    }

    let reward = reward.unwrap();

    if let Some(err) = reward.error {
        ErrorDialog(app, auth, format!("{}\n{}", reward.message, err));
    } else {
        let players = reward.players.unwrap_or_default();
        let powerups = reward.power_ups.unwrap_or_default();

        if !players.is_empty() {
            let mut iter = players.into_iter();
            if let Some(first) = iter.next() {
                EarnedCard(app, auth.clone(), first, iter.collect(), powerups);
            }
        } else if !powerups.is_empty() {
            let mut iter = powerups.into_iter();
            if let Some((first, amount)) = iter.next() {
                EarnedPowerUp(app, auth.clone(), first, amount, iter.collect());
            }
        } else {
            InfoDialog(app, auth, "Reward", reward.message);
        }
    }
}

#[allow(non_snake_case)]
fn EarnedCard(
    app: &mut Cursive,
    auth: String,
    player: Player,
    rest: Vec<Player>,
    powerups: Vec<(PowerUp, u32)>,
) {
    let info = format!(
        "{} ({})\nAttack: {}\nDefense: {}\nPassing: {}\nStamina: {}",
        player.name, player.position, player.attack, player.defense, player.passing, player.stamina
    );

    let dialog = Dialog::around(TextView::new(info)).title("New Player");

    let next_button = if rest.is_empty() {
        if powerups.is_empty() {
            dialog.button("Finish", move |s| MainMenu(s, auth.clone()))
        } else {
            dialog.button("Next", move |s| {
                let mut iter = powerups.clone().into_iter();
                let (first, amount) = iter.next().unwrap();
                EarnedPowerUp(s, auth.clone(), first, amount, iter.collect());
            })
        }
    } else {
        dialog.button("Next", move |s| {
            let mut rest = rest.clone();
            let next = rest.remove(0);
            EarnedCard(s, auth.clone(), next, rest, powerups.clone());
        })
    };

    app.pop_layer();
    app.add_layer(next_button);
}

#[allow(non_snake_case)]
fn EarnedPowerUp(
    app: &mut Cursive,
    auth: String,
    powerup: PowerUp,
    amount: u32,
    rest: Vec<(PowerUp, u32)>,
) {
    let info = format!(
        "{}\nEffect: {}\nAmount: {}",
        powerup.name, powerup.effect, amount
    );

    let dialog = Dialog::around(TextView::new(info)).title("New PowerUp");

    let next_button = if rest.is_empty() {
        dialog.button("Finish", move |s| MainMenu(s, auth.clone()))
    } else {
        dialog.button("Next", move |s| {
            let mut rest = rest.clone();
            let (next, amt) = rest.remove(0);
            EarnedPowerUp(s, auth.clone(), next, amt, rest);
        })
    };

    app.pop_layer();
    app.add_layer(next_button);
}

#[allow(non_snake_case)]
fn ErrorDialog(app: &mut Cursive, auth: String, msg: String) {
    let dialog = Dialog::text(msg)
        .title("Error")
        .button("Back", move |s| MainMenu(s, auth.clone()));
    app.add_layer(dialog);
}

#[allow(non_snake_case)]
fn InfoDialog(app: &mut Cursive, auth: String, title: &str, msg: String) {
    let dialog = Dialog::text(msg)
        .title(title)
        .button("Back", move |s| MainMenu(s, auth.clone()));
    app.add_layer(dialog);
}
