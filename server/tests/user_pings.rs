#[cfg(test)]
extern crate speculate;
#[cfg(test)]
use speculate::speculate;

use quickapi::server::Server;
use server::ping::route_ping;

use server::services::Services;

pub fn services() -> Services {
    use rand::rngs::StdRng;
    use rand::SeedableRng;
    use server::matches::Matches;

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
        route_ping(&mut app);
    }

    it "ping should respond with pong" {
        let response = block_on(app.simulate("GET", "/ping", ""));
        assert_eq!(response.status, 200);

        let body: serde_json::Value = serde_json::from_str(&response.body).unwrap();
        assert_eq!(body["message"], "pong");
    }
}