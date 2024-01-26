use iced::{Element, Length, theme, Alignment};
use iced::widget::{button, row, text, container,column};
use crate::{Message};
use iced::widget::{horizontal_space, scrollable, toggler, vertical_space, Radio, Container};
use multi_platform_screen_grabbing_utility::screenshot::Screenshot;

pub fn home(screen_result: Screenshot) -> Element<'static, Message>{
    let controlRow;
    let imageRow;

    if let Some(screenshot) = &screen_result {

        let image = &screenshot.screen;
        controlRow = row![
                        button(text("New Screenshot").width(Length::Fill).size(20)).style(theme::Button::Primary).on_press(Message::NewScreenshotButton),
                        button(text("Settings").width(Length::Fill).size(20)).style(theme::Button::Secondary).on_press(Message::SettingsButton),
                        button(text("Modify").width(Length::Fill).size(20)).style(theme::Button::Secondary).on_press(Message::ModifyButton)
                ]
            .spacing(20)
            .align_items(Alignment::Center)
            .into();
        imageRow =  row![
                        image(monitorasd.png)
        ].spacing(20)
            .align_items(Alignment::Center)
            .into();
    } else {
        controlRow = row![
                        button(text("New Screenshot").width(Length::Fill).size(20)).style(theme::Button::Primary).on_press(Message::NewScreenshotButton),
                        button(text("Settings").width(Length::Fill).size(20)).style(theme::Button::Secondary).on_press(Message::SettingsButton),
                ]
            .spacing(20)
            .align_items(Alignment::Center)
            .into();
        imageRow =  row![].spacing(20)
            .align_items(Alignment::Center)
            .into();
    }


    let spacev = vertical_space(Length::Fixed(20.0));


    let content: Element<_> = column![ controlRow, spacev, imageRow ]
        .spacing(20)
        .padding(20)
        .into();


    return container(content).height(Length::Fill).center_y().center_x().into();

}
