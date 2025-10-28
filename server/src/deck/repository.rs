use std::collections::HashMap;

use crate::deck::models::Inventory;

#[derive(Debug, Clone)]
pub struct Inventories {
    per_user: HashMap<String, Inventory>,
}

impl Inventories {
    pub fn new() -> Self {
        Self {
            per_user: HashMap::new(),
        }
    }

    /// Get or create an Inventory for a user
    pub async fn deck_of(&mut self, user_id: &str) -> &mut Inventory {
        self.per_user.entry(user_id.into()).or_default()
    }
}
