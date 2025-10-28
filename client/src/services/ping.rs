use std::time::{Duration, Instant};

use crate::services::server_url;
use quickapi::Client;

fn ping_path() -> String {
    "ping".into()
}

pub fn ping(timeout: usize) -> Result<Duration, String> {
    let client = Client::new(&server_url());

    let start = Instant::now();
    let res = client.get(&ping_path());
    let elapsed = start.elapsed();

    if elapsed > Duration::from_secs(timeout as u64) {
        return Err(format!("Timed out, took more than {}s", timeout));
    }

    if res.status >= 400 {
        return Err(serde_json::from_str::<serde_json::Value>(&res.body)
            .ok()
            .and_then(|j| {
                j.get("message")
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string())
            })
            .unwrap_or_else(|| format!("Server returned an error (status {})", res.status)));
    }

    Ok(elapsed)
}
