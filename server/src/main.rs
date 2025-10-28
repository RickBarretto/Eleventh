use rand::rngs::StdRng;
use rand::SeedableRng;

// QuickAPI
use quickapi::Server;

// Routes
use server::account::route_account;
use server::deck::route_decks;
use server::matches::route_match;
use server::ping::route_ping;

// Services
use server::account::Accounts;
use server::deck::Inventories;
use server::deck::Rewarding;
use server::matches::Matches;
use server::services::inject;
use server::services::Services;

#[tokio::main]
async fn main() {
    let url: String = std::env::var("ELEVENTH_ADDRESS")
        .unwrap_or("127.0.0.1:8080".into());

        let rng = StdRng::from_os_rng();

    let services = Services {
        accounts: inject(Accounts::new()),
        inventories: inject(Inventories::new()),
        rewarding: inject(Rewarding::new(rng)),
        matches: inject(Matches::new()),
    };

    let mut app: Server<Services> = Server::new(services);

    route_account(&mut app);
    route_decks(&mut app);
    route_match(&mut app);
    route_ping(&mut app);

    app.run(&url).await;
}
