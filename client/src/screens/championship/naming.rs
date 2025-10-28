/// Name Players & Power-up to use in your next match
use cursive::views::{Dialog, ListView, ScrollView, SelectView};
use cursive::{traits::*, Cursive};
use std::sync::{Arc, Mutex};

use crate::schemas::deck::DeckResponse;
use crate::screens;
use crate::{schemas, services};

use super::Waiting;

/// Players naming window
#[allow(non_snake_case)]
#[allow(non_snake_case)]
pub fn NamePlayers(app: &mut Cursive, auth: String) {
    app.pop_layer();

    let deck = match deck_of(app, &auth) {
        Some(value) => value,
        None => return,
    };

    let chosen_players = Arc::new(Mutex::new(Vec::<usize>::new()));
    let chosen_powerup = Arc::new(Mutex::new(None::<usize>));

    let mut player_select = DeckSelection(&chosen_players);
    for (i, player) in deck.players.iter().enumerate() {
        player_select.add_item(format!("{}", player), i);
    }

    let mut powerup_select = PowerupSelect(&chosen_powerup);
    for (i, (power, count)) in deck.power_ups.iter().enumerate() {
        powerup_select.add_item(format!("x{} {}", count, power), i);
    }

    let confirm_auth = auth.clone();
    let confirm_players = chosen_players.clone();
    let confirm_powerup = chosen_powerup.clone();
    let confirm_deck = deck.clone();

    let list = ListView::new()
        .child("Choose 5 Players", player_select.fixed_height(10))
        .child("Choose 1 Power-up", powerup_select.fixed_height(5));

    let main_dialog = Dialog::around(ScrollView::new(list).scroll_y(true))
        .title("Select Your Team")
        .button("Confirm", move |app| {
            let players = confirm_players.lock().unwrap();
            let power = confirm_powerup.lock().unwrap();

            if players.len() == 5 && power.is_some() {
                let team = schemas::championship::Team {
                    named: players
                        .iter()
                        .map(|&idx| confirm_deck.players[idx].clone())
                        .collect::<Vec<schemas::deck::Player>>(),
                    helper: {
                        let idx = power.unwrap();
                        confirm_deck.power_ups[idx].0.clone()
                    },
                };

                match services::championship::name(&confirm_auth, team) {
                    Ok(resp) => {
                        Info(app, &format!("Team named successfully: {}", resp));
                        Waiting(app, confirm_auth.clone());
                    }
                    Err(e) => {
                        Info(app, &format!("Failed to send team: {}", e));
                    }
                }
            } else {
                Info(app, "You must pick exactly 5 players and 1 power-up!");
            }
        })
        .button("Back", move |app| {
            screens::MainMenu(app, auth.clone());
        });

    app.add_layer(main_dialog);
}

fn deck_of(app: &mut Cursive, auth: &String) -> Option<DeckResponse> {
    let deck: DeckResponse = match services::deck::list(auth) {
        Ok(resp) => match serde_json::from_str::<DeckResponse>(&resp.body) {
            Ok(json) => json,
            Err(_) => {
                Info(app, "Failed to parse deck JSON");
                return None;
            }
        },
        Err(_) => {
            Info(app, "Failed to fetch deck");
            return None;
        }
    };
    Some(deck)
}

#[allow(non_snake_case)]
fn PowerupSelect(chosen_powerup: &Arc<Mutex<Option<usize>>>) -> SelectView<usize> {
    SelectView::<usize>::new().on_submit({
        let chosen_powerup = chosen_powerup.clone();
        move |app, idx| {
            *chosen_powerup.lock().unwrap() = Some(*idx);
            Info(app, &format!("Selected power-up #{}", idx));
        }
    })
}

#[allow(non_snake_case)]
fn DeckSelection(chosen_players: &Arc<Mutex<Vec<usize>>>) -> SelectView<usize> {
    SelectView::<usize>::new().on_submit({
        let chosen_players = chosen_players.clone();
        move |siv, idx| {
            let mut chosen = chosen_players.lock().unwrap();
            if chosen.contains(idx) {
                Info(siv, "Already selected this player");
            } else if chosen.len() >= 5 {
                Info(siv, "You can only choose 5 players");
            } else {
                chosen.push(*idx);
                let text = format!("Added player #{} ({} chosen so far)", idx, chosen.len());
                Info(siv, &text);
            }
        }
    })
}

#[allow(non_snake_case)]
fn Info(siv: &mut Cursive, text: &str) {
    siv.add_layer(Dialog::info(text));
}
