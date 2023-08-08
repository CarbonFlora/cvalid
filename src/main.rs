use iced::{Settings, Application};
use cvalid::args::DIFloraArgs;
use anyhow::Result;
use cvalid::features::verify_sheets_cli;
use clap::Parser;
use cvalid::gui::CValid;

fn main() -> Result<()> {
    let args = DIFloraArgs::try_parse();
    match args {
        Ok(w) => verify_sheets_cli(w)?,
        Err(_e) => CValid::run(Settings::default())?,
    };

    Ok(())
}
