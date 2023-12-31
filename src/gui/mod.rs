// use std::path::PathBuf;

use iced::alignment::{self};
use iced::font::{self, Font};
use iced::theme::Theme;
use iced::widget::{button, column, container, row, scrollable, text, text_input, Column, Text};
use iced::{Application, Element};
use iced::{Color, Command, Length};
// use once_cell::sync::Lazy;

pub mod output;
pub mod search;

use self::output::*;
use self::search::*;

const H_S: u16 = 5;
// static INPUT_ID: Lazy<text_input::Id> = Lazy::new(text_input::Id::unique);

#[derive(Debug)]
pub enum CValid {
    Main(MainInputs),
}

#[derive(Debug, Clone)]
pub struct MainInputs {
    pub files: Vec<String>,
    pub column_index: String,
    pub row_skip: String,
    pub worksheet_skip: String,
}

#[derive(Debug, Clone)]
pub enum Message {
    FontLoaded(Result<(), font::Error>),
    SelectFiles,
    InputColumnIndex(String),
    InputRowSkip(String),
    InputWorksheetSkip(String),
}

impl Application for CValid {
    type Message = Message;
    type Theme = Theme;
    type Executor = iced::executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (CValid, Command<Message>) {
        (
            CValid::Main(MainInputs {
                files: vec![],
                column_index: "6".to_string(),
                row_skip: "3".to_string(),
                worksheet_skip: "0".to_string(),
            }),
            Command::batch([font::load(
                include_bytes!("../../fonts/Byom-Icons-Trial.ttf").as_slice(),
            )
            .map(Message::FontLoaded)]),
        )
    }

    fn title(&self) -> String {
        String::from("Zi's Workbook Column Data Validation")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match self {
            CValid::Main(main_inputs) => {
                match message {
                    Message::SelectFiles => {
                        main_inputs.files = search_files();
                    }
                    Message::InputColumnIndex(raw_input) => {
                        main_inputs.column_index = raw_input;
                    }
                    Message::InputRowSkip(raw_input) => {
                        main_inputs.row_skip = raw_input;
                    }
                    Message::InputWorksheetSkip(raw_input) => {
                        main_inputs.worksheet_skip = raw_input;
                    }
                    _ => (),
                };
                Command::none()
            }
        }
    }

    fn view(&self) -> Element<Message> {
        match self {
            CValid::Main(main_inputs) => {
                let title = header_group();
                let body = column![input_group(main_inputs), output_group(main_inputs)]; //, output_group(main_inputs)

                scrollable(
                    container(column![title, body].spacing(10))
                        .width(Length::Fill)
                        .padding(40)
                        .center_x(),
                )
                .into()
            }
        }
    }
}

fn header_group<'a>() -> Column<'a, Message> {
    let title = text("Workbook Column Data Validation")
        .width(Length::Fill)
        .size(50)
        .style(Color::from([0.5, 0.5, 0.5]))
        .horizontal_alignment(alignment::Horizontal::Center);
    column![title].spacing(40).width(Length::Fill)
}

fn input_group(main_inputs: &MainInputs) -> Column<Message> {
    let choose_files = button(exit_icon().horizontal_alignment(alignment::Horizontal::Center))
        .width(42)
        .on_press(Message::SelectFiles);
    let display_files = text(format!("Selected Files: {}", display_files(main_inputs)));
    let column_index =
        text_input("#", &main_inputs.column_index).on_input(Message::InputColumnIndex);
    let row_skip = text_input("#", &main_inputs.row_skip).on_input(Message::InputRowSkip);
    let worksheet_skip =
        text_input("#", &main_inputs.worksheet_skip).on_input(Message::InputWorksheetSkip);

    let row_1 = row![choose_files, display_files].spacing(H_S);
    let row_2 = row![
        text("Column:"),
        column_index,
        text("# Rows Skipped:"),
        row_skip,
        text("# Worksheets Skipped:"),
        worksheet_skip
    ]
    .spacing(H_S);

    column![row_1, row_2]
        .spacing(10)
        .width(Length::Fill)
        .padding(10)
}

const ICONS2: Font = Font::with_name("Byom Icons");

fn icon2(unicode: char) -> Text<'static> {
    text(unicode.to_string())
        .font(ICONS2)
        .horizontal_alignment(alignment::Horizontal::Center)
        .width(20)
}

fn exit_icon() -> Text<'static> {
    icon2('\u{2e}')
}

fn exclam_icon() -> Text<'static> {
    icon2('\u{21}')
}

// fn good_check_icon() -> Text<'static> {
//     icon2('\u{56}')
// }

fn notification_icon() -> Text<'static> {
    icon2('\u{49}')
}

const SUBTITLE_SIZE: u16 = 22;

fn subtitle(str: &str) -> Text<'static> {
    text(str).size(SUBTITLE_SIZE)
}
