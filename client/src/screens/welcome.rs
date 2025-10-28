use cursive::views::Dialog;
use cursive::views::TextView;
use cursive::Cursive;

use crate::screens;

#[allow(non_snake_case)]
pub fn Welcome(app: &mut Cursive) {
    let content = &vec!["Eleventh", "Only 11 win!", "", "Press <Start> to begin."];
    let text = TextView::new(content.join("\n")).center();

    let view = Dialog::around(text)
        .title("Eleventh")
        .button("Start", |app| screens::AccountMenu(app));

    app.add_layer(view);
}
