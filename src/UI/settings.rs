use std::iter::Filter;
use std::path::Path;
use iced::{Element, Sandbox, Settings, Alignment, Length, alignment, theme, Color};
use iced::widget::{button, row, text, column, container};
use crate::{Message, ScreenState, PagesState};

pub fn settings(screenState: ScreenState) -> Element<'static, Message> {
    return column![
        row![
            button("Back").style(theme::Button::Destructive).on_press(Message::PagesState(PagesState::Home)),
            button("Save").style(theme::Button::Positive).on_press(Message::PagesState(PagesState::Home)),
        ]
        .spacing(20)
        .align_items(Alignment::Center),
        column![
            text("Screenshot Shortcut")
                    .width(Length::Fill)
                    .size(30)
                    .style(Color::from([0.5, 0.5, 0.5]))
                    .horizontal_alignment(alignment::Horizontal::Center),
        ]
        .spacing(20)
        .align_items(Alignment::Center),
        ]
        .into();
}
