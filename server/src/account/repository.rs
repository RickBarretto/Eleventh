use std::collections::HashMap;

use super::models::Account;

#[derive(Clone, Debug)]
pub struct Accounts {
    map: HashMap<String, Account>,
}

impl Accounts {
    pub fn new() -> Self {
        Accounts {
            map: HashMap::new(),
        }
    }

    pub async fn store(&mut self, new_account: Account) -> Result<(), String> {
        if self.map.contains_key(&new_account.username) {
            return Err("Account already exists".into());
        }
        self.map.insert(new_account.username.clone(), new_account);
        Ok(())
    }

    pub async fn by_credentials(&self, username: &str, password: &str) -> Option<&Account> {
        self.map
            .get(username)
            .filter(|acc| acc.password == password)
    }
}
