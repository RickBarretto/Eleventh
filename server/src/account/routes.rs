use std::sync::Arc;

use serde::Deserialize;
use serde_json::json;
use tokio::sync::Mutex;

use quickapi::{Request, Response, Server};

use crate::account::models::Account;
use crate::account::repository::Accounts;
use crate::parse_json;
use crate::services::Services;

#[derive(Debug, Deserialize)]
pub struct Signup {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct Login {
    pub username: String,
    pub password: String,
}

async fn signup(accounts: Arc<Mutex<Accounts>>, request: Request) -> Response {
    let forms: Signup =
        serde_json::from_str(&request.body).expect("It should have the correct format.");

    let new_account = Account::new(forms.username, forms.password);

    match accounts.lock().await.store(new_account.clone()).await {
        Ok(_) => Response::ok().json(&json!({
            "message": "Account created successfully",
            "username": &new_account.username,
            "auth": new_account.auth,
        })),
        Err(_) => Response::unauthorized().json(&json!({
            "message": "Username already exists"
        })),
    }
}

async fn login(accounts: Arc<Mutex<Accounts>>, request: Request) -> Response {
    let data: Login = match parse_json(&request.body) {
        Ok(d) => d,
        Err(resp) => return resp,
    };

    match accounts
        .lock()
        .await
        .by_credentials(&data.username, &data.password)
        .await
    {
        Some(account) => Response::ok().json(&json!({
            "message": "Login successful",
            "username": account.username,
            "auth": account.auth,
        })),
        None => Response::unauthorized().json(&json!({
            "message": "Invalid username or password",
        })),
    }
}

pub fn route_account(app: &mut Server<Services>) {
    let services = app.services.clone();
    app.post("/accounts/create/", move |req, _| {
        let accounts = services.accounts();
        signup(accounts.clone(), req)
    });

    let services = app.services.clone();
    app.post("/accounts/login/", move |req, _| {
        let accounts = services.accounts();
        login(accounts.clone(), req)
    });
}
