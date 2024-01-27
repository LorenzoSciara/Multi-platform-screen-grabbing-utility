use iced::{Element, Length, theme, Alignment};
use iced::widget::{button, row, text};
use crate::{Message, ScreenState};

pub fn home(screenshot_state: ScreenState) -> Element<'static, Message>{
    let row;
    if screenshot_state == ScreenState::ScreenTrue{
        row = row![
                        button(text("New Screenshot").width(Length::Fill).size(20)).style(theme::Button::Primary).on_press(Message::NewScreenshotButton),
                        button(text("Settings").width(Length::Fill).size(20)).style(theme::Button::Secondary).on_press(Message::SettingsButton),
                        button(text("Modify").width(Length::Fill).size(20)).style(theme::Button::Secondary).on_press(Message::ModifyButton)
                ]
            .spacing(20)
            .align_items(Alignment::Center)
            .into();
    } else {
        row = row![
                        button(text("New Screenshot").width(Length::Fill).size(20)).style(theme::Button::Primary).on_press(Message::NewScreenshotButton),
                        button(text("Settings").width(Length::Fill).size(20)).style(theme::Button::Secondary).on_press(Message::SettingsButton),
                ]
            .spacing(20)
            .align_items(Alignment::Center)
            .into();
    }
    return row;
}

