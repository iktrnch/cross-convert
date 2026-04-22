use iced::widget::button::{Status, Style};
use iced::widget::{Button, Text};
use iced::{Background, Border, Color, Length, Theme};

pub fn button<'a, Message>(content: &'a str) -> Button<'a, Message> {
    Button::new(
        Text::new(content)
            .width(Length::Fill)
            .align_x(iced::Alignment::Center)
            .align_y(iced::Alignment::Center),
    )
    .style(|theme: &Theme, status: Status| Style {
        border: Border {
            color: Color::TRANSPARENT,
            width: 0.0,
            radius: iced::border::Radius::new(0.0),
        },
        background: match status {
            Status::Pressed => {
                let color = theme.palette().primary;
                Some(Background::Color(Color::from_rgba(
                    color.r, color.g, color.b, 0.5,
                )))
            }
            Status::Hovered => {
                let color = theme.palette().primary;
                Some(Background::Color(Color::from_rgba(
                    color.r, color.g, color.b, 0.8,
                )))
            }
            _ => Some(Background::Color(theme.palette().primary)),
        },
        ..Default::default()
    })
}
