#![windows_subsystem = "windows"]

pub mod img_manager;
pub mod ui;

use img_manager::{convert_image, is_image_extension, Format};

use iced::widget::{container, text, Column, Row, Space};
use iced::{window, Theme};
use iced::{Length, Settings, Task};

use ui::button::button;

type Element<'a> = iced::Element<'a, Message, Theme, iced::Renderer>;

#[derive(Clone)]
enum Message {
    Format(Format),
    Close,
}

#[derive(Default, Debug, Clone)]
pub struct App {
    file: File,
    result: String,
}

#[derive(Default, Debug, Clone)]
pub struct File {
    path: String,
    ext: String,
}

impl File {
    pub fn new(path: &str) -> Self {
        let ext = std::path::Path::new(path)
            .extension()
            .and_then(std::ffi::OsStr::to_str)
            .unwrap_or("")
            .to_string();
        Self {
            path: String::from(path),
            ext: ext,
        }
    }
}

fn view(app: &App) -> Element<'_> {
    let close_btn = button("X")
        .on_press(Message::Close)
        .width(35)
        .height(35)
        .padding(5);

    let row = Row::new()
        .push(Space::new().width(iced::Length::Fill))
        .push(close_btn);

    let mut column = Column::new();

    let title = text("Choose a file extention");
    column = column.push(title);

    // Create a button for each format
    for format in Format::values() {
        if format == Format::None {
            continue;
        }

        let btn = button(format.as_str())
            .on_press(Message::Format(format))
            .width(Length::Fill)
            .padding(5);
        column = column.push(btn);
    }

    column = column.push(text(&app.result));
    column = column.align_x(iced::Alignment::Center).spacing(10);

    let content = Column::new()
        .push(row)
        .push(column)
        .padding(10)
        .align_x(iced::Alignment::Center);

    container(content)
        .width(iced::Length::Fill)
        .height(iced::Length::Fill)
        .center_x(iced::Length::Fill)
        .into()
}

fn update(app: &mut App, message: Message) -> Task<Message> {
    if !is_image_extension(&app.file.ext) {
        app.result = "Invalid file extension".to_string();
        return Task::none();
    }

    match message {
        Message::Format(format) => {
            if format == Format::None {
                app.result = "No format selected".to_string();
                return Task::none();
            }
            match convert_image(app.file.path.clone(), format.as_str()) {
                Ok(_) => {
                    app.result = "Image converted successfully".to_string();
                    std::process::exit(0);
                }
                Err(e) => {
                    app.result = format!("Error converting image: {}", e);
                }
            }
        }
        Message::Close => {
            std::process::exit(0);
        }
    }
    Task::none()
}

pub fn start_app(file: String) -> anyhow::Result<()> {
    iced::application(
        move || App {
            file: File::new(&file),
            result: String::new(),
        },
        update,
        view,
    )
    .settings(Settings {
        id: Some(String::from("cross-convert")),
        ..Default::default()
    })
    .window(window::Settings {
        size: iced::Size::new(300.0, 500.0),
        resizable: false,
        decorations: false,
        ..Default::default()
    })
    .title(|_: &App| String::from("CrossConvert"))
    .theme(|_: &App| Theme::CatppuccinMocha)
    .run()?;

    Ok(())
}
