use iced::{Element, Alignment, Length, theme};
use iced::widget::{button, row, text, container, column};
use crate::{Message};
use image::{RgbaImage, Rgba};
use iced::widget::image as img;
use imageproc;

pub fn modify(screen_result: Option<RgbaImage>) -> Element<'static, Message> {
    match screen_result {
        Some(screen) => {
            let controlRow:Element<'static, Message> = row![
                        button(text("← Home").width(Length::Fill).size(20)).style(theme::Button::Destructive).on_press(Message::HomeButton),
                        button(text("New Screenshot").width(Length::Fill).size(20)).style(theme::Button::Primary).on_press(Message::NewScreenshotButton),
                ]
                .spacing(20)
                .align_items(Alignment::Center)
                .into();
            let color = Rgba([255, 0, 0, 0]);
            let imageRow:Element<'static, Message> = row![
                       img(img::Handle::from_pixels(
                            screen.width(),
                            screen.height(),
                            imageproc::drawing::draw_hollow_circle(&screen, (370, 250), 100, color).as_raw().clone(),
                        ))
                ]
                .spacing(20)
                .align_items(Alignment::Center)
                .into();
            let content: Element<_> = column![ controlRow, imageRow ].spacing(20).into();
            return container(content).height(Length::Fill).center_y().center_x().into();
        }
        None => {
            let controlRow:Element<'static, Message> = row![
                        button(text("← Home").width(Length::Fill).size(20)).style(theme::Button::Destructive).on_press(Message::HomeButton),
                        button(text("New Screenshot").width(Length::Fill).size(20)).style(theme::Button::Primary).on_press(Message::NewScreenshotButton),
                ]
                .spacing(20)
                .align_items(Alignment::Center)
                .into();
            let imageRow:Element<'static, Message> = row![
                       text("Error! No Screenshot")
                ]
                .spacing(20)
                .align_items(Alignment::Center)
                .into();
            let content: Element<_> = column![ controlRow, imageRow ].spacing(20).into();
            return container(content).height(Length::Fill).center_y().center_x().into();
        }
    }
}
