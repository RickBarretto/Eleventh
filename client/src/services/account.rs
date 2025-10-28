use quickapi::Client;

use crate::services::server_url;

fn accounts_path(path: &str) -> String {
    format!("accounts/{}", path)
}

pub fn signup(username: &str, password: &str) -> Result<String, String> {
    let client = Client::new(&server_url());
    let path = accounts_path("create");
    let res = request_at(&client, &path, username, password)?;
    let json = humanized_result(res)?;
    auth_or_error(json, "Signup failed")
}

pub fn login(username: &str, password: &str) -> Result<String, String> {
    let client = Client::new(&server_url());
    let path = accounts_path("login");
    let res = request_at(&client, &path, username, password)?;
    let json = humanized_result(res)?;
    auth_or_error(json, "Login failed")
}

fn request_at(
    client: &Client,
    path: &str,
    username: &str,
    password: &str,
) -> Result<quickapi::Response, String> {
    let body = serde_json::json!({ "username": username, "password": password }).to_string();
    let res = client.post(path, &body);
    Ok(res)
}

fn humanized_result(res: quickapi::Response) -> Result<serde_json::Value, String> {
    if res.status >= 400 {
        return parse_error(&res);
    }

    serde_json::from_str(&res.body).map_err(|_| "Failed to parse response".to_string())
}

fn parse_error(res: &quickapi::Response) -> Result<serde_json::Value, String> {
    let json: Result<serde_json::Value, _> = serde_json::from_str(&res.body);
    Err(json.ok()
        .and_then(|j| {
            j.get("message")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string())
    }).unwrap_or_else(|| "Server returned an error".to_string()))
}

fn auth_or_error(json: serde_json::Value, error_message: &'static str) -> Result<String, String> {
    match json.get("auth").and_then(|v| v.as_str()) {
        Some(auth) => Ok(auth.to_string()),
        None => Err(json
            .get("message")
            .and_then(|v| v.as_str())
            .unwrap_or(error_message)
            .to_owned()),
    }
}
