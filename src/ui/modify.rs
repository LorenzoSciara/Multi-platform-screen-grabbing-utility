use std::any::Any;
use crate::{CropMode, Draw, Message};
use iced::theme::TextInput;
use iced::{Element, Alignment, Length, theme, Event, Color, Theme, Background};
use iced::widget::{button, row, text, column, text_input, Row, container, vertical_slider, Container};
use image::{RgbaImage, Rgba};
use iced::widget::image as img;
use imageproc;
use crate::SCREENSHOT_CONTAINER;

pub fn modify(screen_result: Option<RgbaImage>, draw: Draw, draw_text: String, screen_result_backup: Option<RgbaImage>, color_slider_value: u8, crop: CropMode) -> Element<'static, Message> {

static mut R: f32 = 0.0;
static mut G: f32 = 0.0;
static mut B: f32 = 0.0;
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
    if draw == Draw::Crop && crop == CropMode::Crop {
            crop_button = button(text("Crop").width(Length::Fill).size(20)).style(theme::Button::Positive).on_press(Message::CropButton);
    } else if draw == Draw::Crop && crop == CropMode::CropConfirm {
            crop_button = button(text("Confirm").width(Length::Fill).size(20)).style(theme::Button::Positive).on_press(Message::CropButton);
    } else {
        crop_button = button(text("Crop").width(Length::Fill).size(20)).style(theme::Button::Secondary).on_press(Message::CropButton);
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
            let color_container;
            match color_slider_value.clone() {
                0..=9 => {color_container = Container::new(row![]).style(style::black_container).height(50).width(50);}
                10..=19 => {color_container = Container::new(row![]).style(style::red_container).height(50).width(50);}
                20..=29 => {color_container = Container::new(row![]).style(style::orange_container).height(50).width(50);}
                30..=39 => {color_container = Container::new(row![]).style(style::yellow_container).height(50).width(50);}
                40..=49 => {color_container = Container::new(row![]).style(style::green_container).height(50).width(50);}
                50..=59 => {color_container = Container::new(row![]).style(style::blue_container).height(50).width(50);}
                60..=69 => {color_container = Container::new(row![]).style(style::indigo_container).height(50).width(50);}
                70..=79 => {color_container = Container::new(row![]).style(style::violet_container).height(50).width(50);}
                    _ => {color_container = Container::new(row![]).style(style::white_container).height(50).width(50);}
            }
            let color_selector:Element<'static, Message> = row![
                vertical_slider(0..=100, color_slider_value.clone(), Message::DrawColorSlider).step(1),
                column![text("Color Selector").width(Length::Fill).size(20), color_container ]
            ].spacing(20).into();
            let imageRow:Element<'static, Message> = row![ imageContainer, color_selector, ].spacing(20).into();
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

mod style {
    use iced::widget::container;
    use iced::{BorderRadius, Color, Theme};

    pub fn white_container(theme: &Theme) -> container::Appearance {
        let palette = theme.extended_palette();

        container::Appearance {
            text_color: Some(palette.background.strong.text),
            background: Some(Color::from_rgb(255.0 / 255.0, 255.0 / 255.0, 255.0 / 255.0).into()),
            border_radius: BorderRadius::from(15.0),
            border_width: 2.0,
            border_color: Color::BLACK,
            ..Default::default()
        }
    }
    pub fn red_container(theme: &Theme) -> container::Appearance {
        let palette = theme.extended_palette();

        container::Appearance {
            text_color: Some(palette.background.strong.text),
            background: Some(Color::from_rgb(255.0 / 255.0, 0.0 / 255.0, 0.0 / 255.0).into()),
            border_radius: BorderRadius::from(15.0),
            border_width: 3.0,
            border_color: Color::BLACK,
            ..Default::default()
        }
    }
    pub fn orange_container(theme: &Theme) -> container::Appearance {
        let palette = theme.extended_palette();

        container::Appearance {
            text_color: Some(palette.background.strong.text),
            background: Some(Color::from_rgb(255.0 / 255.0, 165.0 / 255.0, 0.0 / 255.0).into()),
            border_radius: BorderRadius::from(15.0),
            border_width: 3.0,
            border_color: Color::BLACK,
            ..Default::default()
        }
    }
    pub fn yellow_container(theme: &Theme) -> container::Appearance {
        let palette = theme.extended_palette();

        container::Appearance {
            text_color: Some(palette.background.strong.text),
            background: Some(Color::from_rgb(255.0 / 255.0, 255.0 / 255.0, 51.0 / 255.0).into()),
            border_radius: BorderRadius::from(15.0),
            border_width: 3.0,
            border_color: Color::BLACK,
            ..Default::default()
        }
    }
    pub fn green_container(theme: &Theme) -> container::Appearance {
        let palette = theme.extended_palette();

        container::Appearance {
            text_color: Some(palette.background.strong.text),
            background: Some(Color::from_rgb(34.0 / 255.0, 139.0 / 255.0, 34.0 / 255.0).into()),
            border_radius: BorderRadius::from(15.0),
            border_width: 3.0,
            border_color: Color::BLACK,
            ..Default::default()
        }
    }
    pub fn blue_container(theme: &Theme) -> container::Appearance {
        let palette = theme.extended_palette();

        container::Appearance {
            text_color: Some(palette.background.strong.text),
            background: Some(Color::from_rgb(0.0 / 255.0, 0.0 / 255.0, 255.0 / 255.0).into()),
            border_radius: BorderRadius::from(15.0),
            border_width: 3.0,
            border_color: Color::BLACK,
            ..Default::default()
        }
    }
    pub fn indigo_container(theme: &Theme) -> container::Appearance {
        let palette = theme.extended_palette();

        container::Appearance {
            text_color: Some(palette.background.strong.text),
            background: Some(Color::from_rgb(73.0 / 255.0, 0.0 / 255.0, 130.0 / 255.0).into()),
            border_radius: BorderRadius::from(15.0),
            border_width: 3.0,
            border_color: Color::BLACK,
            ..Default::default()
        }
    }
    pub fn violet_container(theme: &Theme) -> container::Appearance {
        let palette = theme.extended_palette();

        container::Appearance {
            text_color: Some(palette.background.strong.text),
            background: Some(Color::from_rgb(218.0 / 255.0, 112.0 / 255.0, 238.0 / 255.0).into()),
            border_radius: BorderRadius::from(15.0),
            border_width: 3.0,
            border_color: Color::BLACK,
            ..Default::default()
        }
    }
    pub fn black_container(theme: &Theme) -> container::Appearance {
        let palette = theme.extended_palette();

        container::Appearance {
            text_color: Some(palette.background.strong.text),
            background: Some(Color::from_rgb(0.0 / 255.0, 0.0 / 255.0, 0.0 / 255.0).into()),
            border_radius: BorderRadius::from(15.0),
            border_width: 3.0,
            border_color: Color::BLACK,
            ..Default::default()
        }
    }
}