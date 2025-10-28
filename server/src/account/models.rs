use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Account {
    pub username: String,
    pub password: String,
    pub created_at: String,
    pub auth: String,
}

impl Account {
    pub fn new(username: String, password: String) -> Account {
        use std::time;

        let created_at = time::SystemTime::now()
            .duration_since(time::UNIX_EPOCH)
            .map(|d| d.as_secs().to_string())
            .unwrap_or_else(|_| "0".to_string());

        let auth = Uuid::new_v4().to_string();

        Account {
            username,
            password,
            created_at,
            auth,
        }
    }
}
