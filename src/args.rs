use clap::Parser;

/// Simple program to verify workbook data integrity.
#[derive(Parser, Debug)]
#[command(author="Zi Hao L.", version="0.2.0", about="Workbook Data Integrity Check.", long_about = None)]
pub struct DIFloraArgs {
    // /// Print duplicates or conflicts
    // #[arg(short, long, default_value_t = false)]
    // pub verbose: bool,

    /// Specify a specific column
    #[arg(short, long, default_value_t = 5)]
    pub column: usize,

    /// Specify number of beginning rows to skip
    #[arg(short, long, default_value_t = 2)]
    pub rskip: usize,

    /// Specify number of beginning worksheets to skip
    #[arg(short, long, default_value_t = 0)]
    pub wskip: usize,

    /// Select any number of workbooks
    #[arg(required = true)]
    pub input_sheet: Vec<String>,
}