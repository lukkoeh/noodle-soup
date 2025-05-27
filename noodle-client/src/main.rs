// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::error::Error;

slint::include_modules!();

fn main() -> Result<(), Box<dyn Error>> {
    let ui = AppWindow::new()?;

    // Add some dummy test data
    let user_lastname = "Müller";
    let user_firstname = "Max";

    ui.set_user_firstname(user_firstname.into());
    ui.on_search_bar_edited(move |text| {
        println!("Text changed: {}", text);
    });
    ui.on_button_outline_clicked(move || {
        println!("Button clicked:");
    });
    ui.run()?;

    Ok(())
}
