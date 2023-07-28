use iced::widget::{column, text, Column};
use anyhow::{Result, Error, anyhow};

use crate::gui::*;
use std::collections::HashMap;
use calamine::{Reader, open_workbook, Xlsx, DataType};

type ErrorPair = (String, String);

pub fn output_group(main_inputs: &MainInputs) -> Column<Message> {
    let mut column = column![].spacing(10).width(Length::Fill).padding(10);
    
    match main_inputs.verify_sheets_gui() {
        Ok(w) => {
            column = column.push(text(format!("{}", w)));
        },
        Err(e) => {
            column = column.push(text(format!("{:?}", e)));
        },
    }

    column
}

impl MainInputs {
    pub fn verify_sheets_gui(&self) -> Result<String> {
        let mut buffer = String::new();
        let mut file_success = vec![];
        let mut file_failure = vec![];
        let mut job_number: HashMap<String, String> = HashMap::new(); //dupe information, dupe location
    
        for path in self.files.clone() {
            match self.check_excel(&path, &mut job_number) {
                Ok(w) => file_success.push((path, w)),
                Err(e) => file_failure.push((path, e)),
            }
        }
    
        let skipped = self.write_to_buffer_file_failure(file_failure, &mut buffer)?; 
        let (success, dupe) = self.write_to_buffer_error_pair(file_success, &mut buffer)?; 
        self.write_to_buffer_summary(skipped, (success, dupe), &mut buffer)?;
        
        Ok(buffer)
    }

    fn check_excel(&self, path: &String, job_number: &mut HashMap<String, String>) -> Result<Vec<ErrorPair>> {
        let mut workbook: Xlsx<_> = open_workbook(path)?;
        let mut error_pairs = vec![];
        let check_columns: Vec<usize> = vec![self.column_index.parse::<usize>()?];
        let binding = path.chars().rev().take(15).collect::<String>()+"..";
        let mut cut_path = binding.chars().rev().collect::<String>();
        if cut_path.len() == path.len() {
            cut_path = cut_path.trim_start_matches('.').to_string();
        }

        for (worksheet_name, worksheet) in workbook.worksheets().iter().skip(self.worksheet_skip.parse::<usize>()?) {
            for (row_index, row) in worksheet.rows().enumerate().skip(self.row_skip.parse::<usize>()?) {
                self.crawl(row, &cut_path, job_number, &worksheet_name, &row_index, &mut error_pairs, &check_columns)?;
            } 
        }
    
        Ok(error_pairs)
    }
    
    fn crawl(&self, row: &[DataType], path: &String, job_number: &mut HashMap<String, String>, worksheet_name: &String, row_index: &usize, error_pairs: &mut Vec<ErrorPair>, check_columns: &Vec<usize>) -> Result<()> {
        for col_index in check_columns {
            if let Some(next_entry) = row.get(*col_index) {
                let dir_text = format!("{path}/{worksheet_name}/Row #{row_index}/{}", next_entry.to_string().clone());
        
                if let Some(dir_dupe) = job_number.insert(next_entry.to_string().clone(), dir_text.clone()) { 
                    error_pairs.push((dir_dupe, dir_text));
                }
            } else {
                return Err(anyhow!("Specified column doesn't correspond to any information."))
            }
        }
        Ok(())
    }
    
    fn write_to_buffer_file_failure(&self, file_failure: Vec<(String, Error)>, buffer: &mut String) -> Result<usize> {
        for file in &file_failure {
            *buffer+=format!("\nX: {} > {}", file.0, file.1).as_str();
        }
    
        Ok(file_failure.len())
    }
    
    fn write_to_buffer_error_pair(&self, file_success: Vec<(String, Vec<ErrorPair>)>, buffer: &mut String) -> Result<(usize, usize)> {
        for file in &file_success {
            for error_pair in &file.1 {
                *buffer+=format!("\nX: {} && {}", error_pair.0, error_pair.1).as_str();
            }
        }
        let total_errors = file_success.iter().fold(0, |acc, x| acc+x.1.len());
        Ok((file_success.len(), total_errors))
    }
    
    fn write_to_buffer_summary(&self, f: usize, s: (usize, usize), buffer: &mut String) -> Result<()> {
        if f > 0 {
            *buffer+=format!("\n?: Number of Skipped Files = {}", f).as_str();
            // writeln!(buffer, "?: Number of Skipped Files = {}", f).unwrap();
        }
        if s.0 > 0 {
            // writeln!(buffer, "?: Number of Read Files = {}", s.0)?;
            *buffer+=format!("\n?: Number of Read Files = {}", s.0).as_str();
            *buffer+=format!("\n?: Number of Duplicates = {}", s.1).as_str();
            *buffer+="\n--";
        }
    
        Ok(())
    }
}

