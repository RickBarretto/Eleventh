use quickapi::Client;

use crate::schemas::championship::MatchState;
use crate::schemas::championship::Team;
use crate::services::server_url;

fn match_path(user: &str, action: &str) -> String {
    format!("match/{}/{}", user, action)
}

/// Join a player to a match
pub fn join(auth: &String) -> Result<String, String> {
    let client = Client::new(&server_url());
    // Check current status first. If user is already pairing/paired, return that state
    if let Ok(state) = status(auth) {
        if state.status == "pairing" || state.status == "paired" {
            let json = serde_json::json!({
                "status": state.status,
                "host": state.host,
                "guest": state.guest,
                "score": state.score,
                "winner": state.winner,
            });
            return Ok(json.to_string());
        }
    }

    let path = match_path(auth, "start");
    let res = client.post(&path, "");

    if res.status >= 400 {
        return Err(parse_error(&res));
    }

    Ok(res.body)
}

/// Fetches the state of the current match of the player
pub fn status(auth: &String) -> Result<MatchState, String> {
    let client = Client::new(&server_url());
    let path = match_path(auth, "status");
    let res = client.get(&path);

    if res.status >= 400 {
        return Err(parse_error(&res));
    }

    serde_json::from_str::<MatchState>(&res.body)
        .map_err(|e| format!("Failed to parse JSON: {}", e))
}

/// Name a team for a Match
pub fn name(auth: &String, team: Team) -> Result<String, String> {
    let client = Client::new(&server_url());
    let path = match_path(auth, "name");
    let body =
        serde_json::to_string(&team).map_err(|e| format!("Failed to serialize team: {}", e))?;
    let res = client.post(&path, &body);

    if res.status >= 400 {
        return Err(parse_error(&res));
    }

    Ok(res.body)
}

fn parse_error(res: &quickapi::Response) -> String {
    serde_json::from_str::<serde_json::Value>(&res.body)
        .ok()
        .and_then(|j| {
            j.get("message")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string())
        })
        .unwrap_or_else(|| format!("Server returned an error (status {})", res.status))
}
