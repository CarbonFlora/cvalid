use crate::args::DIFloraArgs;
use anyhow::{Result, Error};
use clap::Parser;
use std::collections::HashMap;
// use std::fs::File;
use std::io::{self, BufWriter, Stdout, Write};
use calamine::{Reader, open_workbook, Xlsx, DataType};

type ErrorPair = (String, String);

pub fn verify_sheets_cli() -> Result<()> {
    let args = DIFloraArgs::parse();
    let mut buffer = BufWriter::new(io::stdout());
    let mut file_success = vec![];
    let mut file_failure = vec![];
    let mut job_number = HashMap::new(); //dupe information, dupe location

    // let paths = args.input_sheet.iter().map(|x| x.to_uppercase()).collect::<Vec<String>>();
    for path in args.input_sheet.clone() {
        match check_excel(&args, &path, &mut job_number) {
            Ok(w) => file_success.push((path, w)),
            Err(e) => file_failure.push((path, e)),
        }
    }

    let skipped = write_to_buffer_file_failure(file_failure, &mut buffer)?; 
    let (success, dupe) = write_to_buffer_error_pair(file_success, &mut buffer)?; 
    write_to_buffer_summary(skipped, (success, dupe), &mut buffer)?;
    
    buffer.flush()?;
    Ok(())
}

pub fn check_excel(args: &DIFloraArgs, path: &String, job_number: &mut HashMap<String, String>) -> Result<Vec<ErrorPair>> {
    let mut workbook: Xlsx<_> = open_workbook(path)?;
    let mut error_pairs = vec![];
    let check_columns: Vec<usize> = vec![args.column];
    // let mut invoice_number = HashMap::new();
    // let mut ref_number = HashMap::new();

    for (worksheet_name, worksheet) in workbook.worksheets().iter().skip(args.wskip) {
        for (row_index, row) in worksheet.rows().enumerate().skip(args.rskip) { //skip 2 -> skip headers
            crawl(row, path, job_number, &worksheet_name, &row_index, &mut error_pairs, &check_columns);
        } 
    }

    Ok(error_pairs)
}

fn crawl(row: &[DataType], path: &String, job_number: &mut HashMap<String, String>, worksheet_name: &String, row_index: &usize, error_pairs: &mut Vec<ErrorPair>, check_columns: &Vec<usize>) {
    for col_index in check_columns {
        if let Some(next_entry) = row[*col_index].as_string() {
            let dir_text = format!("{path}/{worksheet_name}/Row #{row_index}/{}", next_entry.clone());
    
            if let Some(dir_dupe) = job_number.insert(next_entry.clone(), dir_text.clone()) { 
                error_pairs.push((dir_dupe, dir_text));
            }
        } 
    }
}

fn write_to_buffer_file_failure(file_failure: Vec<(String, Error)>, buffer: &mut BufWriter<Stdout>) -> Result<usize> {
    for file in &file_failure {
        writeln!(buffer, "X: {} > {}", file.0, file.1).unwrap();
    }

    Ok(file_failure.len())
}

fn write_to_buffer_error_pair(file_success: Vec<(String, Vec<ErrorPair>)>, buffer: &mut BufWriter<Stdout>) -> Result<(usize, usize)> {
    for file in &file_success {
        for error_pair in &file.1 {
            writeln!(buffer, "X: {} && {}", error_pair.0, error_pair.1)?;        
        }
    }
    let total_errors = file_success.iter().fold(0, |acc, x| acc+x.1.len());
    Ok((file_success.len(), total_errors))
}

fn write_to_buffer_summary(f: usize, s: (usize, usize), buffer: &mut BufWriter<Stdout>) -> Result<()> {
    if f > 0 {
        writeln!(buffer, "?: Number of Skipped Files = {}", f).unwrap();
    }
    if s.0 > 0 {
        writeln!(buffer, "?: Number of Read Files = {}", s.0)?;
        writeln!(buffer, "?: Number of Duplicates = {}", s.1)?;
    }

    Ok(())
}