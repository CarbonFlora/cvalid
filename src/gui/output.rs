use anyhow::{anyhow, Error, Result};
use iced::widget::{column, text, Column};

use crate::gui::*;
use calamine::{open_workbook, DataType, Reader, Xlsx};
use std::collections::HashMap;

type ErrorPair = (String, String);

pub fn output_group(main_inputs: &MainInputs) -> Column<Message> {
    let mut column = column![subtitle("Summary")]
        .spacing(10)
        .width(Length::Fill)
        .padding(10);

    column = column.push(main_inputs.output_column());
    // match main_inputs.verify_sheets_gui() {
    //     Err(e) => {
    //         column = column.push(row![exclam_icon(), text(format!("{:?}", e))]);
    //     }
    //     Ok(w) => {
    //         column = column.push(text(w));
    //     }
    // }

    column
}

impl MainInputs {
    pub fn output_column(&self) -> Column<'_, Message> {
        let mut column = column![].spacing(10).width(Length::Fill).padding(10);
        let mut file_success = vec![];
        let mut file_failure = vec![];
        let mut job_number: HashMap<String, String> = HashMap::new(); //dupe information, dupe location

        for path in self.files.clone() {
            match self.audit_excel(&path, &mut job_number) {
                Ok(w) => file_success.push((path, w)),
                Err(e) => file_failure.push((path, e)),
            }
        }

        column = column
            .push(self.file_failure_block(&file_failure))
            .push(self.error_pair_block(&file_success))
            .push(self.summary_block(&file_failure, &file_success));

        column
    }

    fn file_failure_block(&self, file_failure: &Vec<(String, Error)>) -> Column<'_, Message> {
        let mut ff_column = column![];

        for file in file_failure {
            ff_column = ff_column.push(row![
                exclam_icon(),
                text(format!(" {} > {}", file.0, file.1))
            ]);
        }

        ff_column
    }

    fn error_pair_block(
        &self,
        file_success: &Vec<(String, Vec<ErrorPair>)>,
    ) -> Column<'_, Message> {
        let mut ep_column = column![];

        for file in file_success {
            for error_pair in &file.1 {
                ep_column = ep_column.push(row![
                    exclam_icon(),
                    text(format!(" {} && {}", error_pair.0, error_pair.1))
                ]);
            }
        }

        ep_column
    }

    fn summary_block(
        &self,
        file_failure: &Vec<(String, Error)>,
        file_success: &Vec<(String, Vec<ErrorPair>)>,
    ) -> Column<'_, Message> {
        let mut summary_column = column![];
        let total_errors = file_success.iter().fold(0, |acc, x| acc + x.1.len());

        if !file_failure.is_empty() {
            summary_column = summary_column.push(row![
                notification_icon(),
                text(format!(" Number of file failures: {}", file_failure.len()))
            ]);
        }
        summary_column = summary_column
            .push(row![
                notification_icon(),
                text(format!(" Number of files searched: {}", file_success.len()))
            ])
            .push(row![
                notification_icon(),
                text(format!(" Number of duplicates: {}", total_errors))
            ]);

        summary_column
    }

    fn audit_excel(
        &self,
        path: &String,
        job_number: &mut HashMap<String, String>,
    ) -> Result<Vec<ErrorPair>> {
        let mut workbook: Xlsx<_> = open_workbook(path)?;
        let mut error_pairs = vec![];
        let check_columns: Vec<usize> = vec![self.column_index.parse::<usize>()?];
        let binding = path.chars().rev().take(15).collect::<String>() + "..";
        let mut cut_path = binding.chars().rev().collect::<String>();
        if cut_path.len() == path.len() {
            cut_path = cut_path.trim_start_matches('.').to_string();
        }

        for (worksheet_name, worksheet) in workbook
            .worksheets()
            .iter()
            .skip(self.worksheet_skip.parse::<usize>()?)
        {
            for (row_index, row) in worksheet
                .rows()
                .enumerate()
                .skip(self.row_skip.parse::<usize>()?)
            {
                self.crawl(
                    row,
                    &cut_path,
                    job_number,
                    worksheet_name,
                    &row_index,
                    &mut error_pairs,
                    &check_columns,
                )?;
            }
        }

        Ok(error_pairs)
    }

    #[allow(clippy::too_many_arguments)]
    fn crawl(
        &self,
        row: &[DataType],
        path: &String,
        job_number: &mut HashMap<String, String>,
        worksheet_name: &String,
        row_index: &usize,
        error_pairs: &mut Vec<ErrorPair>,
        check_columns: &Vec<usize>,
    ) -> Result<()> {
        for col_index in check_columns {
            if let Some(next_entry) = row.get(*col_index) {
                let dir_text = format!(
                    "{path}/{worksheet_name}/Row #{}/{}",
                    row_index + 1,
                    next_entry.to_string().clone()
                );

                if let Some(dir_dupe) =
                    job_number.insert(next_entry.to_string().clone(), dir_text.clone())
                {
                    error_pairs.push((dir_dupe, dir_text));
                }
            } else {
                return Err(anyhow!(
                    "Specified column doesn't correspond to any information."
                ));
            }
        }
        Ok(())
    }
}
