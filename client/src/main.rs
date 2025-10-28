use cursive::Cursive;
use cursive::*;

use eleventh::screens;
use eleventh::theme;

fn main() {
    let mut app = Cursive::default();

    screens::Welcome(&mut app);

    app.set_theme(theme::theme());
    app.run();
}
