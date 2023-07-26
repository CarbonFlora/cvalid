use crate::args::DIFloraArgs;
use anyhow::{anyhow, Result};
use clap::Parser;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write, BufWriter, Stdout};

pub fn verify_sheets() -> BufWriter<Stdout> {
    let args = DIFloraArgs::parse();
    let mut deck: Vec<String> = vec![];
    let mut cli_output = BufWriter::new(io::stdout());

    for path in args.input_sheet {
        writeln!(cli_output, "{path}").unwrap();
    }

    cli_output
}