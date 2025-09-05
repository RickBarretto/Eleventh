use std::sync::Arc;
use std::sync::Mutex;

use crate::account::Accounts;
use crate::deck::Inventories;
use crate::deck::Rewarding;

pub struct Services {
    pub accounts: Arc<Mutex<Accounts>>,
    pub inventories: Arc<Mutex<Inventories>>,
    pub rewarding: Arc<Mutex<Rewarding>>,
}

pub fn inject<T>(service: T) -> Arc<Mutex<T>> {
    Arc::new(Mutex::new(service))
}

impl Services {
    pub fn accounts(&self) -> Arc<Mutex<Accounts>> {
        Arc::clone(&self.accounts)
    }

    pub fn inventories(&self) -> Arc<Mutex<Inventories>> {
        Arc::clone(&self.inventories)
    }

    pub fn rewarding(&self) -> Arc<Mutex<Rewarding>> {
        Arc::clone(&self.rewarding)
    }
}
