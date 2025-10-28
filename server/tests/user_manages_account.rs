#[cfg(test)]
extern crate speculate;
#[cfg(test)]
use speculate::speculate;

use serde_json::json;

use quickapi::server::Server;
use server::account::route_account;
use server::matches::Matches;
use server::services::Services;

fn block_on<F: std::future::Future>(future: F) -> F::Output {
    tokio::runtime::Runtime::new().unwrap().block_on(future)
}

pub fn services() -> Services {
    use rand::rngs::StdRng;
    use rand::SeedableRng;

    use server::account::Accounts;
    use server::deck::{Inventories, Rewarding};
    use server::services::inject;

    let rng = StdRng::from_os_rng();

    Services {
        accounts: inject(Accounts::new()),
        inventories: inject(Inventories::new()),
        rewarding: inject(Rewarding::new(rng)),
        matches: inject(Matches::new()),
    }
}

speculate! {

    describe "Accounts Route" {

        before {
            let mut app = Server::new(services());
            route_account(&mut app);
        }

        it "creates account" {
            let payload = json!({
                "username": "alice",
                "password": "secret"
            }).to_string();

            let res = block_on(app.simulate("POST", "/accounts/create/", &payload));
            assert_eq!(res.status, 200);

            let body: serde_json::Value = serde_json::from_str(&res.body).unwrap();
            assert_eq!(body["message"], "Account created successfully");
            assert_eq!(body["username"], "alice");
            assert!(body.get("auth").is_some());
        }

        it "doesn't duplicate" {
            let payload = json!({
                "username": "bob",
                "password": "secret"
            }).to_string();

            let create_new = block_on(app.simulate("POST", "/accounts/create/", &payload));
            assert_eq!(create_new.status, 200);

            let recreate = block_on(app.simulate("POST", "/accounts/create/", &payload));
            assert_eq!(recreate.status, 401);

            let body: serde_json::Value = serde_json::from_str(&recreate.body).unwrap();
            assert_eq!(body["message"], "Username already exists");
        }
    }

    describe "New user Charlie" {

        before {
            let mut app = Server::new(services());
            route_account(&mut app);
        }

        it "creates a new account" {
            let signup = json!({
                "username": "charlie",
                "password": "12345"
            }).to_string();

            let response = block_on(app.simulate("POST", "/accounts/create/", &signup));
            assert_eq!(response.status, 200);
        }

        it "insert wrong credentials" {
            block_on(
                app.simulate("POST", "/accounts/create/", &json!({
                    "username": "charlie",
                    "password": "right"
                }).to_string())
            );

            let wrong_login = json!({
                "username": "charlie",
                "password": "wrong"
            }).to_string();

            let response = block_on(app.simulate("POST", "/accounts/login/", &wrong_login));
            assert_eq!(response.status, 401);

            let body: serde_json::Value = serde_json::from_str(&response.body).unwrap();
            assert_eq!(body["message"], "Invalid username or password");
        }

        it "insert correct credentials" {
            block_on(
                app.simulate("POST", "/accounts/create/", &json!({
                    "username": "charlie",
                    "password": "right"
                }).to_string())
            );

            let login = json!({
                "username": "charlie",
                "password": "right"
            }).to_string();

            let response = block_on(app.simulate("POST", "/accounts/login/", &login));
            assert_eq!(response.status, 200);

            let body: serde_json::Value = serde_json::from_str(&response.body).unwrap();
            assert_eq!(body["message"], "Login successful");
            assert_eq!(body["username"], "charlie");
        }
    }
}
