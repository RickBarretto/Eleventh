use cursive::views::Dialog;
use cursive::Cursive;

use crate::services;

#[allow(non_snake_case)]
pub fn Ping(app: &mut Cursive) {
    let message: String = match services::ping::ping(10) {
        Ok(duration) => {
            let ms = duration.as_millis();
            if ms < 1000 {
                format!("Ping successful: {} ms", ms)
            } else {
                let secs = duration.as_secs_f64();
                format!("Ping successful: {:.2} s", secs)
            }
        }
        Err(err) => format!("Ping failed: {}", err),
    };
    
    let view = Dialog::info(message);
    app.add_layer(view);
}
