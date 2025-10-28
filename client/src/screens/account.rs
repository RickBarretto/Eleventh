use cursive::view::Nameable;
use cursive::view::Resizable;
use cursive::views::{Button, Dialog, EditView, LinearLayout, TextView};
use cursive::Cursive;

use crate::screens;
use crate::services;

#[allow(non_snake_case)]
pub fn AccountMenu(app: &mut Cursive) {
    app.pop_layer();

    let login_button = Button::new("Login", |app| {
        let username = app
            .call_on_name("username", |field: &mut EditView| field.get_content())
            .unwrap()
            .to_string();
        let password = app
            .call_on_name("password", |field: &mut EditView| field.get_content())
            .unwrap()
            .to_string();

        let auth_result = services::account::login(&username, &password);

        match auth_result {
            Ok(auth) => {
                let auth = auth.clone();
                screens::MainMenu(app, auth);
            }
            Err(message) => {
                app.add_layer(Dialog::info(message));
            }
        }
    });

    let signup_button = Button::new("Signup", |app| {
        let username = app
            .call_on_name("username", |field: &mut EditView| field.get_content())
            .unwrap()
            .to_string();
        let password = app
            .call_on_name("password", |view: &mut EditView| view.get_content())
            .unwrap()
            .to_string();

        let auth_result = services::account::signup(&username, &password);

        match auth_result {
            Ok(auth) => {
                let auth = auth.clone();
                screens::MainMenu(app, auth);
            }
            Err(message) => {
                app.add_layer(Dialog::info(message));
            }
        }
    });

    let layout = LinearLayout::vertical()
        .child(TextView::new("Username"))
        .child(EditView::new().with_name("username").fixed_width(20))
        .child(TextView::new("Password"))
        .child(
            EditView::new()
                .secret()
                .with_name("password")
                .fixed_width(20),
        )
        .child(login_button)
        .child(signup_button);

    let dialog = Dialog::around(layout).title("Login / Signup");

    app.add_layer(dialog);
}
