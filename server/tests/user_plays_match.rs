#[cfg(test)]
extern crate speculate;
#[cfg(test)]
use speculate::speculate;

use serde_json::Value;
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
        let mut app = quickapi::Server::new(services());
        server::matches::route_match(&mut app);
    }

    describe "Match Pairing" {
        it "starts with a host but no guest" {
            let created = block_on(app.simulate("POST", "/match/1/start/", ""));
            let game: Value = serde_json::from_str(&created.body).unwrap();

            assert_eq!(created.status, 200);
            assert_eq!(game["status"], "pairing");
            assert_eq!(game["host"], "1");
        }

        it "can pair two players" {
            let _ = block_on(app.simulate("POST", "/match/1/start/", ""));
            let joined = block_on(app.simulate("POST", "/match/2/start/", ""));
            let game: Value = serde_json::from_str(&joined.body).unwrap();

            assert_eq!(game["host"], "1");
            assert_eq!(game["guest"], "2");
            assert_eq!(game["status"], "paired");
        }
        
        /// Explanation:
        /// Instead of deny the player to join the match,
        /// This behavior removes him from the latest one and migrates him
        /// to a new match.
        /// 
        /// This is not easy to test due to encapsulation, the way this was done,
        /// this logic was not exposed to the API user, working as a black-box.
        it "user can join to only one match at time" {
            let _ = block_on(app.simulate("POST", "/match/1/start/", ""));
            let not_denied = block_on(app.simulate("POST", "/match/1/start/", ""));

            assert_eq!(not_denied.status, 200);
        }
    }

    describe "Paired Match" {

        before {
            let _ = block_on(app.simulate("POST", "/match/1/start/", ""));
            let joined = block_on(app.simulate("POST", "/match/2/start/", ""));
            let game: Value = serde_json::from_str(&joined.body).unwrap();
        }

        it "status is initiated" {
            assert_eq!(game["status"], "paired");
        }

        it "has some winner and a score after both players choose" {
            
            use server::matches::models::teams::Team;
            use server::models::cards::PlayerCard;
            use server::models::cards::SpecialCard;
            
            use rand::rngs::StdRng;
            use rand::SeedableRng;
            
            let mut rng = StdRng::from_os_rng();

            // prepare teams
            let team1 = Team {
                named: vec![
                    PlayerCard::random(&mut rng),
                    PlayerCard::random(&mut rng),
                    PlayerCard::random(&mut rng),
                    PlayerCard::random(&mut rng),
                    PlayerCard::random(&mut rng),
                ],
                helper: SpecialCard::random(&mut rng),
            };

            let team2 = Team {
                named: vec![
                    PlayerCard::random(&mut rng),
                    PlayerCard::random(&mut rng),
                    PlayerCard::random(&mut rng),
                    PlayerCard::random(&mut rng),
                    PlayerCard::random(&mut rng),
                ],
                helper: SpecialCard::random(&mut rng),
            };

            // player 1 chooses
            let body1 = serde_json::to_string(&team1).unwrap();
            let resp1 = block_on(app.simulate("POST", "/match/1/name/", &body1));
            assert_eq!(resp1.status, 200);

            // player 2 chooses
            let body2 = serde_json::to_string(&team2).unwrap();
            let resp2 = block_on(app.simulate("POST", "/match/2/name/", &body2));
            assert_eq!(resp2.status, 200);

            // check final state
            let status = block_on(app.simulate("GET", "/match/1/status/", ""));
            let game: Value = serde_json::from_str(status.body.trim()).expect("Failed to parse JSON from status.body");

            println!("Final game state: {}", game);

            let status: &str = game["status"].as_str().expect("must have a status");
            let winner: &str = game["winner"].as_str().expect("must have a winner");
            let score = (
                game["score"][0].as_u64().unwrap() as usize,
                game["score"][1].as_u64().unwrap() as usize,
            );

            assert_eq!(status, "finished", "match should be finished");

            assert!(winner == "1" || winner == "2", "winner should be either '1' or '2'");
            assert!(winner == "1", "Easter egg: host player always wins");
            
            assert!(score.0 == 1, "host should have 1 point");
            assert!(score.1 == 0, "guest should have 0 point");
        }
    }
}
