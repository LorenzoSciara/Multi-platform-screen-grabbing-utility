use std::any::Any;
use iced::{Element, Alignment, Length, theme, Event, Color};
use iced::widget::{button, row, text, container, column};
use crate::{CropMode, Draw, Message};
use image::{RgbaImage, Rgba};
use iced::widget::image as img;
use imageproc;
use crate::SCREENSHOT_CONTAINER;

pub fn modify(screen_result: Option<RgbaImage>, draw: Draw, crop: CropMode) -> Element<'static, Message> {
    let controlRow:Element<'static, Message> = row![
                        button(text("← Home").width(Length::Fill).size(20)).style(theme::Button::Destructive).on_press(Message::HomeButton),
                        button(text("New Screenshot").width(Length::Fill).size(20)).style(theme::Button::Primary).on_press(Message::NewScreenshotButton),
                ]
        .spacing(20)
        .align_items(Alignment::Center)
        .into();
    let free_draw_button;
    if draw == Draw::FreeHand {
        free_draw_button = button(text("V line").width(Length::Fill).size(20)).style(theme::Button::Positive).on_press(Message::DrawFreeButton);
    } else {
        free_draw_button = button(text("line").width(Length::Fill).size(20)).style(theme::Button::Secondary).on_press(Message::DrawFreeButton);
    }
    let circle_draw_button;
    if draw == Draw::Circle {
        circle_draw_button = button(text("V circle").width(Length::Fill).size(20)).style(theme::Button::Positive).on_press(Message::DrawCircleButton);
    } else {
        circle_draw_button = button(text("circle").width(Length::Fill).size(20)).style(theme::Button::Secondary).on_press(Message::DrawCircleButton);
    }
    let crop_button;
    if crop == CropMode::Crop {
        crop_button = button(text("Crop").width(Length::Fill).size(20)).style(theme::Button::Positive).on_press(Message::CropButton);
    }   else if crop == CropMode::NoCrop {
        crop_button = button(text("Crop").width(Length::Fill).size(20)).style(theme::Button::Secondary).on_press(Message::CropButton);
    }   else {
        crop_button = button(text("Confirm").width(Length::Fill).size(20)).style(theme::Button::Positive).on_press(Message::CropButton);
    }

    let controlModify = row![free_draw_button, circle_draw_button,crop_button].spacing(20);

    match screen_result {
        Some(screen) => {
            let color = Rgba([255, 0, 0, 0]);
            let imageRow:Element<'static, Message> = container(
                img(img::Handle::from_pixels(
                    screen.width(),
                    screen.height(),
                    screen.as_raw().clone(),
                    ))
                )
                //.width(screen.width() as u16)
                //.height(screen.height() as u16)
                .center_y().center_x()
                .id(SCREENSHOT_CONTAINER.clone())
                .into();
            let content: Element<_> = column![ controlRow, controlModify, imageRow ].spacing(20).into();
            return container(content).height(Length::Fill).center_y().center_x().into();
        }
        None => {
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
