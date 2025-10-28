pub mod state;

use serde_json::json;
use state::{GameResponse, GameState};
use std::collections::HashMap;

use quickapi::{Request, Response, Server};

use super::service::Matches;
use crate::{
    matches::models::{matches::Match, teams::Team},
    services::Services,
    Shared,
};

async fn join(matches: Shared<Matches>, params: HashMap<String, String>) -> Response {
    let user = params.get("id").unwrap().clone();
    let mut matches = matches.lock().await;

    let to_remove: Vec<String> = matches
        .iter()
        .filter_map(|(k, s)| match s {
            GameState::Created(c) if c.by == user => Some(k.clone()),
            GameState::Paired(p) if p.host == user || p.guest == user => Some(k.clone()),
            GameState::Finished(_) => Some(k.clone()),
            _ => None,
        })
        .collect();

    for key in to_remove {
        matches.remove(&key);
    }

    let host_key = matches.iter().find_map(|(k, s)| match s {
        GameState::Created(_) => Some(k.clone()),
        _ => None,
    });

    if let Some(host_id) = host_key {
        if let GameState::Created(created) = matches.remove(&host_id).unwrap() {
            if let Some(paired) = created.join(user.clone()) {
                matches.insert(host_id.clone(), GameState::Paired(paired.clone()));
                let paired = json!(GameResponse::from(&GameState::Paired(paired)));
                return Response::ok().json(&paired);
            }
        }
    }

    let created = Match::new(user.clone());
    matches.insert(user.clone(), GameState::Created(created.clone()));
    Response::ok().json(&json!(GameResponse::from(&GameState::Created(created))))
}

async fn name(
    matches: Shared<Matches>,
    request: Request,
    params: HashMap<String, String>,
) -> Response {
    let mut matches = matches.lock().await;

    let user: String = params.get("id").expect("Have user ID").into();
    let team: Team = serde_json::from_str(&request.body).expect("Have right body format");

    let paired_key = matches.iter().find_map(|(k, s)| match s {
        GameState::Paired(p) if p.host == user || p.guest == user => Some(k.clone()),
        _ => None,
    });

    if let Some(key) = paired_key {
        if let GameState::Paired(mut paired) = matches.remove(&key).unwrap() {
            paired.name(user.clone(), team);

            if let Some(f) = paired.finish() {
                matches.insert(paired.host.clone(), GameState::Finished(f.clone()));
                let finished = GameResponse::from(&GameState::Finished(f));
                return Response::ok().json(&json!(finished));
            }

            matches.insert(key.clone(), GameState::Paired(paired.clone()));
            let keep_paired = GameResponse::from(&GameState::Paired(paired));
            return Response::ok().json(&json!(keep_paired));
        }
    }

    Response::not_found().json(&json!({
        "message": "no paired match"
    }))
}

async fn status(matches: Shared<Matches>, params: HashMap<String, String>) -> Response {
    let user = params.get("id").expect("To have user ID").clone();
    let matches = matches.lock().await;

    let state = matches.values().find(|s| match s {
        GameState::Created(created) => created.by == user,
        GameState::Paired(paired) => paired.host == user || paired.guest == user,
        GameState::Finished(_) => true,
    });

    match state {
        Some(current) => Response::ok().json(&json!(GameResponse::from(current))),
        None => Response::not_found().json(&json!({"message": "no match"})),
    }
}

pub fn route_match(app: &mut Server<Services>) {
    let services = app.services.clone();

    app.post("/match/{id}/start/", move |_, params| {
        let matches = services.matches();
        join(matches, params)
    });

    let services = app.services.clone();
    app.post("/match/{id}/name/", move |req, params| {
        let matches = services.matches();
        name(matches, req, params)
    });

    let services = app.services.clone();
    app.get("/match/{id}/status/", move |_, params| {
        let matches = services.matches();
        status(matches, params)
    });
}
