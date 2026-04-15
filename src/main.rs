pub mod core;
pub mod gui;

use crate::gui::LumosGui;

fn main() {
    let res = iced::run(LumosGui::update, LumosGui::view);

    match res {
        Ok(_) => {
            println!("GUI ran successfully");
        }
        Err(_) => {
            println!("Error during GUI run")
        }
    }
}
