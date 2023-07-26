use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author="Zi Hao L.", version="0.1.0", about="Proprietary Workbook Data Integrity Check.", long_about = None)]
pub struct DIFloraArgs {
    /// Print duplicates or conflicts
    #[arg(short, long, default_value_t = false)]
    pub verbose: bool,

    /// Select any number of workbooks
    #[arg(required = true)]
    pub input_sheet: Vec<String>,
}