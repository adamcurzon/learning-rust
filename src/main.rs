use arboard::Clipboard;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

use slint::{Timer, TimerMode};
use std::time::Duration;

slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;
    let timer = Timer::default();

    ui.on_generate_password({
        let ui_handle = ui.as_weak();
        move || {
            let ui = ui_handle.unwrap();

            let rand_string: String = thread_rng()
                .sample_iter(&Alphanumeric)
                .take(20)
                .map(char::from)
                .collect();

            let rand_string_shared = rand_string.clone().into();
            ui.set_password(rand_string_shared);

            let mut clipboard = Clipboard::new().unwrap();
            clipboard.set_text(rand_string).unwrap();
        }
    });

    ui.on_copy_password({
        let ui_handle = ui.as_weak();
        move || {
            let ui = ui_handle.unwrap();

            let password = ui.get_password().to_string();

            let mut clipboard = Clipboard::new().unwrap();
            clipboard.set_text(password).unwrap();

            ui.set_clipboard_text("Copied".into());

            timer.start(TimerMode::SingleShot, Duration::from_millis(1000), move || {
                ui.set_clipboard_text("Copy to clipboard".into());
            });
        }
    });
    ui.run()
}
