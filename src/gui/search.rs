use native_dialog::FileDialog;
use crate::gui::MainInputs;

pub fn search_files() -> Vec<String> {
    let result = FileDialog::new()
        // .add_filter("Rust Source", &["rs"])
        .add_filter("Workbook", &["xls", "xlsx", "xlsm","xlsb", "xla", "xlam","ods"])
        .show_open_multiple_file();
    
    match result {
        Ok(w) => w.iter().map(|x| x.to_str().unwrap_or_default().to_string()).collect::<Vec<String>>(),
        Err(_e) => vec![],
    }
}

pub fn display_files(main_inputs: &MainInputs) -> String {
    main_inputs.files.iter().fold("".to_string(), |acc, x| acc+"\n"+x)
}