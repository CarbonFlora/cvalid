use crate::args::DIFloraArgs;
use anyhow::{Result, Error};
use clap::Parser;
use std::collections::HashMap;
// use std::fs::File;
use std::io::{self, BufWriter, Stdout, Write};
use calamine::{Reader, open_workbook, Xlsx};

type ErrorPair = (String, String);

pub fn verify_sheets() -> Result<BufWriter<Stdout>> {
    let args = DIFloraArgs::parse();
    let mut buffer = BufWriter::new(io::stdout());
    let mut file_success = vec![];
    let mut file_failure = vec![];
    
    // let paths = args.input_sheet.iter().map(|x| x.to_uppercase()).collect::<Vec<String>>();
    for path in args.input_sheet {
        match check_excel(&path) {
            Ok(w) => file_success.push((path, w)),
            Err(e) => file_failure.push((path, e)),
        }
    }

    write_to_buffer_f(file_failure, &mut buffer)?; 
    write_to_buffer_s(file_success, &mut buffer)?; 

    

    Ok(buffer)
}

fn check_excel(path: &String) -> Result<Vec<ErrorPair>> {
    let mut workbook: Xlsx<_> = open_workbook(path)?;
    let mut error_pairs = vec![];
    let mut job_number = HashMap::new(); //dupe information, dupe location
    // let mut invoice_number = HashMap::new();
    // let mut ref_number = HashMap::new();

    for (worksheet_name, worksheet) in workbook.worksheets() {
        for (row_index, row) in worksheet.rows().enumerate().skip(2) { //skip 2 -> skip headers
            if let Some(next_job_number) = row[1].as_string() {
                let dir_text = format!("{path}/{worksheet_name}/Row #{row_index}/{}", next_job_number.clone());

                if let Some(dir_dupe) = job_number.insert(next_job_number.clone(), dir_text.clone()) { 
                    error_pairs.push((dir_dupe, dir_text));
                }
            }            
        } 
    }

    Ok(error_pairs)
}

fn write_to_buffer_f(file_failure: Vec<(String, Error)>, buffer: &mut BufWriter<Stdout>) -> Result<()> {
    for file in &file_failure {
        writeln!(buffer, "X: {} > {}", file.0, file.1).unwrap();
    }
    if file_failure.len() > 0 {
        writeln!(buffer, "?: Number of Skipped Files = {}", file_failure.len()).unwrap();
    }
    
    Ok(())
}

fn write_to_buffer_s(file_success: Vec<(String, Vec<ErrorPair>)>, buffer: &mut BufWriter<Stdout>) -> Result<()> {
    for file in &file_success {
        for error_pair in &file.1 {
            writeln!(buffer, "X: {} && {}", error_pair.0, error_pair.1)?;        
        }
    }
    if file_success.len() > 0 {
        writeln!(buffer, "?: Number of Read Files = {}", file_success.len())?;
        let total_errors = file_success.iter().fold(0, |acc, x| acc+x.1.len());
        writeln!(buffer, "?: Number of Duplicates = {}", total_errors)?;
    }
    Ok(())
}

