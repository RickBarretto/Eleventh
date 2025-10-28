#[cfg(test)]
extern crate speculate;
#[cfg(test)]
use speculate::speculate;

use quickapi::server::Server;
use server::deck::route_decks;
use server::matches::Matches;
use server::services::Services;

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

fn block_on<F: std::future::Future>(future: F) -> F::Output {
    tokio::runtime::Runtime::new().unwrap().block_on(future)
}

speculate! {

    describe "User Claims Deck" {

        before {
            let mut app = Server::new(services());
            route_decks(&mut app);
        }

        it "should get a new deck" {
            let response = block_on(app.simulate("GET", "/user/123/deck/claim/", ""));
            assert_eq!(response.status, 200);

            let body: serde_json::Value = serde_json::from_str(&response.body).unwrap();
            assert_eq!(body["message"], "You got new cards!");
            assert!(body["players"].is_array());
            assert!(body["power_ups"].is_array());
        }

        it "should prevent claiming twice in 24h" {
            let _ = block_on(app.simulate("GET", "/user/123/deck/claim/", ""));
            let second = block_on(app.simulate("GET", "/user/123/deck/claim", ""));

            assert_eq!(second.status, 400);

            let body: serde_json::Value = serde_json::from_str(&second.body).unwrap();
            assert_eq!(body["message"], "Could not claim reward!");
            assert_eq!(body["error"], "Reward already claimed in the last 24h");
        }
    }
}
