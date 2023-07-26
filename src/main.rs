use std::io::Write;
use anyhow::Result;

use diflora::features::verify_sheets;

fn main() -> Result<()> {
    Ok(verify_sheets().flush()?)
}