use iced::{Settings, Application};

use cvalid::gui::CValid;
use cvalid::features::verify_sheets;

fn main() -> Result<(), iced::Error> {
    if verify_sheets().is_err() {
        return CValid::run(Settings::default())
    }

    Ok(())
}
