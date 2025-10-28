use cursive::views::Dialog;
use cursive::Cursive;
use std::thread;
use std::time::Duration;

use crate::services;

use super::NamePlayers;

fn fetch(auth: &String) -> bool {
    services::championship::status(&auth).is_ok_and(|status| status.status == "paired")
}

#[allow(non_snake_case)]
pub fn Pairing(app: &mut Cursive, auth: String) {
    let view = Dialog::new().title("Pairing...").button("Cancel", |app| {
        app.pop_layer();
    });

    app.add_layer(view);

    let sink = app.cb_sink().clone();
    let auth = auth.clone();

    thread::spawn(move || loop {
        thread::sleep(Duration::from_secs(1));
        if fetch(&auth) {
            let auth = auth.clone();
            sink.send(Box::new(move |app: &mut Cursive| {
                NamePlayers(app, auth.clone());
            }))
            .unwrap();
            break;
        }
    });
}
