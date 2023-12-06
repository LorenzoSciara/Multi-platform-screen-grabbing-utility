use std::iter::Filter;
use std::path::Path;
use iced::{Element, Sandbox, Settings, Alignment, Length, alignment, theme};
use iced::Background::Color;
use iced::widget::{button, row, text, column, image, Image, container};
use crate::{Message, ScreenState, PagesState};

pub fn home(screenshotState: ScreenState) -> Element<'static, Message>{
    let row;
    if screenshotState == ScreenState::ScreenTrue{
        row = row![
                        button("New Screenshot").style(theme::Button::Primary).on_press(Message::Screenshot(ScreenState::ScreenTrue)),
                        button("Settings").style(theme::Button::Secondary).on_press(Message::Settings(PagesState::Settings)),
                        button("Modify").style(theme::Button::Secondary).on_press(Message::Modify(PagesState::Modify))
                ]
            .spacing(20)
            .align_items(Alignment::Center)
            .into();
    } else {
        row = row![
                        button("New Screenshot").style(theme::Button::Primary).on_press(Message::Screenshot(ScreenState::ScreenTrue)),
                        button("Settings").style(theme::Button::Secondary).on_press(Message::Settings(PagesState::Settings)),
                ]
            .spacing(20)
            .align_items(Alignment::Center)
            .into();
    }
    return row;
}
