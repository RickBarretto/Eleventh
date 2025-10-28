use std::sync::Arc;

use cursive::views::Dialog;
use cursive::Cursive;

use crate::screens;

#[allow(non_snake_case)]
pub fn MainMenu(app: &mut Cursive, auth: String) {
    app.pop_layer();

    let auth = Arc::new(auth);

    let options = Dialog::text("Main Menu")
        .title("Eleventh")
        .button("Match", {
            let auth = auth.clone();
            move |app| screens::ChampionshipMenu(app, (*auth).clone())
        })
        .button("Team", {
            let auth = auth.clone();
            move |app| screens::TeamScreen(app, (*auth).clone())
        })
        .button("Reward", {
            let auth = auth.clone();
            move |app| screens::RewardScreen(app, (*auth).clone())
        })
        .button("Ping", screens::Ping)
        .button("Quit", |app| app.quit());

    app.add_layer(options);
}
