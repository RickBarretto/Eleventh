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

    before {
        let mut app = Server::new(services());
        route_decks(&mut app);
    }

    it "deck should be initially empty" {
        let response = block_on(app.simulate("GET", "/user/123/deck/", ""));
        assert_eq!(response.status, 200);

        let body: serde_json::Value = serde_json::from_str(&response.body).unwrap();
        assert!(body["players"].as_array().unwrap().is_empty());
        assert!(body["power_ups"].as_array().unwrap().is_empty());
    }

    it "can claim new cards" {
        let claim = block_on(app.simulate("GET", "/user/123/deck/claim", ""));
        assert_eq!(claim.status, 200);

        let claim_body: serde_json::Value = serde_json::from_str(&claim.body).unwrap();
        assert_eq!(claim_body["message"], "You got new cards!");
        assert!(claim_body["players"].as_array().unwrap().len() > 0);
    }

    it "rewards only once in 24h" {
        block_on(app.simulate("GET", "/user/123/deck/claim", ""));
        let claim_again = block_on(app.simulate("GET", "/user/123/deck/claim", ""));
        assert_eq!(claim_again.status, 400);

        let claim_again_body: serde_json::Value = serde_json::from_str(&claim_again.body).unwrap();
        assert_eq!(claim_again_body["message"], "Could not claim reward!");
        assert_eq!(claim_again_body["error"], "Reward already claimed in the last 24h");
    }

    it "can fire players" {
        let claim = block_on(app.simulate("GET", "/user/123/deck/claim", ""));
        let fire = block_on(app.simulate("DELETE", "/user/123/deck/fire/0/", ""));
        let final_deck = block_on(app.simulate("GET", "/user/123/deck/", ""));

        let fire_body: serde_json::Value = serde_json::from_str(&fire.body).unwrap();
        let claim_body: serde_json::Value = serde_json::from_str(&claim.body).unwrap();
        let final_body: serde_json::Value = serde_json::from_str(&final_deck.body).unwrap();

        assert_eq!(claim.status, 200);
        assert_eq!(fire.status, 200);

        assert_eq!(claim_body["message"], "You got new cards!");
        assert!(claim_body["players"].as_array().unwrap().len() > 0);

        assert_eq!(fire_body["message"], "Card removed from deck");

        assert_eq!(final_body["message"], "Your deck");
        assert_eq!(final_body["players"].as_array().unwrap().len(), claim_body["players"].as_array().unwrap().len() - 1);
    }

}
