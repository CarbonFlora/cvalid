use iced::{Settings, Application};

use cvalid::gui::CValid;

fn main() -> Result<(), iced::Error> {
    CValid::run(Settings::default())
}
