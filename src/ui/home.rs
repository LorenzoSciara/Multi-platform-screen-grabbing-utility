use iced::{Element, Length, theme, Alignment};
use iced::widget::{button, row, text, container, column};
use crate::{Message};
use iced::widget::{horizontal_space, scrollable, toggler, vertical_space, Radio, Container};
use iced::widget::image as img;
use multi_platform_screen_grabbing_utility::screenshot::Screenshot;
use image::RgbaImage;


pub fn home(screen_result: Option<RgbaImage>, toggler_value_autosave: bool) -> Element<'static, Message> {
    let mut controlRow:Element<'static, Message> = row![].into();
    let mut imageRow:Element<'static, Message> = row![].into();

    let screen_btn = button(text("New Screenshot").width(Length::Fill).size(20)).style(theme::Button::Primary).on_press(Message::NewScreenshotButton);
    let settings_btn = button(text("Settings").width(Length::Fill).size(20)).style(theme::Button::Secondary).on_press(Message::SettingsButton);
    let modify_btn = button(text("Modify").width(Length::Fill).size(20)).style(theme::Button::Secondary).on_press(Message::ModifyButton);
    let save_btn = button(text("Save").width(Length::Fill).size(20)).style(theme::Button::Positive).on_press(Message::SaveButton);


    match screen_result {
        Some(screen) => {
            if toggler_value_autosave{
                controlRow = row![screen_btn, settings_btn, modify_btn ]
                    .spacing(20)
                    .align_items(Alignment::Center)
                    .into();
            }
            else{
                controlRow = row![screen_btn, settings_btn, modify_btn, save_btn]
                    .spacing(20)
                    .align_items(Alignment::Center)
                    .into();
            }
            imageRow = row![
                       img(img::Handle::from_pixels(
                            screen.width(),
                            screen.height(),
                            screen.as_raw().clone(),
                        ))
        ].spacing(20)
                .align_items(Alignment::Center)
                .into();

        }
        None => {
            controlRow = row![screen_btn, settings_btn]
                .spacing(20)
                .align_items(Alignment::Center)
                .into();
        }
    }

        let spacev = vertical_space(Length::Fixed(20.0));


        let content: Element < _ > = column![ controlRow, imageRow ]
        .spacing(20)
        .into();


        return container(content).height(Length::Fill).center_y().center_x().into();
}