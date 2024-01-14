use std::iter::Filter;
use std::path::Path;
use iced::{Element, Sandbox, Settings, Alignment, Length, alignment, theme};
use iced::Background::Color;
use iced::widget::{button, row, text, column, container};
use crate::{Message, ScreenState, PagesState};

pub fn modify() -> Element<'static, Message>{
    return column![
            row![
                        button("New Screenshot").style(theme::Button::Primary).on_press(Message::ScreenState(ScreenState::ScreenTrue)),
                        button("Settings").style(theme::Button::Secondary).on_press(Message::PagesState(PagesState::Settings)),
                ]
                    .spacing(20)
                    .align_items(Alignment::Center),
            ]
        .spacing(20)
        .align_items(Alignment::Center)
        .into();
}
