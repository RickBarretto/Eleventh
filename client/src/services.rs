pub mod account;
pub mod championship;
pub mod deck;
pub mod ping;

/// Returns the Eleventh's Server URL
/// 
/// This value may change according to the environment,
/// this allows the compatibility for locally deployed environments
/// which allow us to develop, prototype and build quickly, and also,
/// for production, which is used along side Docker.
pub fn server_url() -> String {
    std::env::var("ELEVENTH_ADDRESS")
        .unwrap_or("127.0.0.1:8080".into())
}