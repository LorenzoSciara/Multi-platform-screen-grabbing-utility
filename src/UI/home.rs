use std::iter::Filter;
use std::path::Path;
use iced::{Element, Sandbox, Settings, Alignment, Length, alignment, theme};
use iced::Background::Color;
use iced::widget::{button, row, text, column, container};
use crate::{Message, ScreenState, PagesState};

pub fn home(screenshotState: ScreenState) -> Element<'static, Message>{
    let row;
    if screenshotState == ScreenState::ScreenTrue{
        row = row![
                        button("New Screenshot").style(theme::Button::Primary).on_press(Message::ScreenState(ScreenState::ScreenTrue)),
                        button("Settings").style(theme::Button::Secondary).on_press(Message::PagesState(PagesState::Settings)),
                        button("Modify").style(theme::Button::Secondary).on_press(Message::PagesState(PagesState::Modify))
                ]
            .spacing(20)
            .align_items(Alignment::Center)
            .into();
    } else {
        row = row![
                        button("New Screenshot").style(theme::Button::Primary).on_press(Message::ScreenState(ScreenState::ScreenTrue)),
                        button("Settings").style(theme::Button::Secondary).on_press(Message::PagesState(PagesState::Settings)),
                ]
            .spacing(20)
            .align_items(Alignment::Center)
            .into();
    }
    return row;
}
