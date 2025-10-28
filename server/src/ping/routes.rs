use quickapi::{Response, Server};
use serde_json::json;

use crate::services::Services;


async fn pong() -> Response {
    Response::ok().json(&json!({
        "message": "pong"
    }))
}


pub fn route_ping(app: &mut Server<Services>) {
    app.get("/ping", |_, _| pong());
}