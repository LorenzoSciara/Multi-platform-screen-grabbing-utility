use iced::{Element, Alignment, Length, theme};
use iced::widget::{button, row, text};
use crate::{Message};

pub fn modify() -> Element<'static, Message>{
    let row = row![
                        button(text("← Home").width(Length::Fill).size(20)).style(theme::Button::Destructive).on_press(Message::HomeButton),
                        button(text("New Screenshot").width(Length::Fill).size(20)).style(theme::Button::Primary).on_press(Message::NewScreenshotButton),
                ]
        .spacing(20)
        .align_items(Alignment::Center)
        .into();
    return row;
}
