use std::any::Any;
use crate::{CropMode, Draw, Message};
use iced::{Element, Alignment, Length, theme, Event, Color, Theme};
use iced::theme::TextInput;
use iced::widget::{button, row, text, container, column, text_input, Row, vertical_slider};
use image::{RgbaImage, Rgba};
use iced::widget::image as img;
use imageproc;
use crate::SCREENSHOT_CONTAINER;

pub fn modify(screen_result: Option<RgbaImage>, draw: Draw, draw_text: String, screen_result_backup: Option<RgbaImage>, color_slider_value: u8, crop: CropMode) -> Element<'static, Message> {
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

    let arrow_draw_button;
    if draw == Draw::Arrow {
        arrow_draw_button = button(text("V arrow").width(Length::Fill).size(20)).style(theme::Button::Positive).on_press(Message::DrawArrowButton);
    } else {
        arrow_draw_button = button(text("arrow").width(Length::Fill).size(20)).style(theme::Button::Secondary).on_press(Message::DrawArrowButton);
    }
    let text_draw_button;
    let text_draw_text = text_input("Enter text and place it in the image", draw_text.as_str()).width(Length::Fixed(350.0)).size(20).on_input(Message::DrawTextInput);
    if draw == Draw::Text {
        text_draw_button = button(text("V text").width(Length::Fill).size(20)).style(theme::Button::Positive).on_press(Message::DrawTextButton);
    } else {
        text_draw_button = button(text("text").width(Length::Fill).size(20)).style(theme::Button::Secondary).on_press(Message::DrawTextButton);
    }
    let clear_button = button(text("Clear").width(Length::Fill).size(20)).style(theme::Button::Destructive).on_press(Message::DrawClearButton);
    let mut controlModify = Row::new();
    if draw == Draw::Text && screen_result == screen_result_backup{
        controlModify = row![crop_button, free_draw_button, circle_draw_button, arrow_draw_button, text_draw_button, text_draw_text].spacing(20);
    }
    else if draw == Draw::Text && screen_result != screen_result_backup{
        controlModify = row![crop_button, free_draw_button, circle_draw_button, arrow_draw_button, text_draw_button, text_draw_text, clear_button].spacing(20);
    }
    else if draw != Draw::Text && screen_result != screen_result_backup{
        controlModify = row![crop_button, free_draw_button, circle_draw_button, arrow_draw_button, text_draw_button, clear_button].spacing(20);
    }
    else{
        controlModify = row![crop_button, free_draw_button, circle_draw_button, arrow_draw_button, text_draw_button].spacing(20);
    }

    match screen_result {
        Some(screen) => {
            let color = Rgba([255, 0, 0, 0]);
            let imageContainer = container(
                img(img::Handle::from_pixels(
                    screen.width(),
                    screen.height(),
                    screen.as_raw().clone(),
                    ))
                )
                .center_y().center_x()
                .id(SCREENSHOT_CONTAINER.clone())
                .style(theme::Container::Box);
            let verticalSlider = vertical_slider(0..=255, color_slider_value.clone(), Message::DrawColorSlider)
                .step(1);
            let imageRow:Element<'static, Message> = row![imageContainer, verticalSlider].spacing(20).into();
            //println!("container width -> {0:?}", imageRow.as_widget().width());
            //println!("container height -> {0:?}", imageRow.as_widget().height());
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
