use quickapi::Client;

use crate::services::server_url;

fn deck_path(user: &str, action: &str) -> String {
    format!("user/{}/deck/{}", user, action)
}

pub fn list(auth: &String) -> Result<quickapi::Response, String> {
    let client = Client::new(&server_url());
    let path = deck_path(auth, "");
    let res = client.get(&path);
    if res.status >= 400 {
        parse_error(&res)
    } else {
        Ok(res)
    }
}

pub fn fire_player(i: usize, auth_clone: &String) -> Result<(), String> {
    let client = Client::new(&server_url());
    let path = deck_path(&auth_clone, &format!("fire/{}", i));
    let res = client.delete(&path);
    if res.status >= 400 {
        match parse_error(&res) {
            Err(e) => Err(format!("Failed to fire player: {}", e)),
            Ok(_) => Err("Failed to fire player: Unknown error".to_string()),
        }
    } else {
        Ok(())
    }
}

fn parse_error(response: &quickapi::Response) -> Result<quickapi::Response, String> {
    Err(serde_json::from_str::<serde_json::Value>(&response.body)
        .ok()
        .and_then(|j| {
            j.get("message")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string())
        })
        .unwrap_or_else(|| format!("Server returned an error (status {})", response.status)))
}
