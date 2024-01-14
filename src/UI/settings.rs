use std::iter::Filter;
use std::path::Path;
use iced::{Element, Sandbox, Settings, Alignment, Length, alignment, theme, Color};
use iced::widget::{button, row, text, column, container};
use crate::{Message, ScreenState, PagesState, Choice};

use iced::widget::{checkbox, horizontal_space, radio, scrollable, slider, text_input, toggler, vertical_space, Text, Radio};
use iced::widget::{Button, Column, Container, Slider};
use iced::{Font, Pixels, Renderer};

use iced::widget::container::{Appearance, StyleSheet};

//
const ICONS: Font = Font::with_name("Iced-Todos-Icons");

fn icon(unicode: char) -> Text<'static> {
    text(unicode.to_string())
        .font(ICONS)
        .width(20)
        .horizontal_alignment(alignment::Horizontal::Center)
}
fn delete_icon() -> Text<'static> {
    icon('\u{F1F8}')
}


pub fn settings(screenState: ScreenState, toggler_value: bool) -> Element<'static, Message> {
    let undobutton = button(row![delete_icon(), text("Back").width(Length::Fill).size(20) ]
        .spacing(10)
        .align_items(Alignment::Center))
        .on_press(Message::PagesState(PagesState::Home))
        .style(theme::Button::Destructive)
        .width(Length::Fixed(100.0))
        .height(Length::Fixed(50.0))
        .padding(10);

    let space = horizontal_space(Length::Fill);

    let savebutton = button(row![delete_icon(), text("Save").width(Length::Fill).size(20) ]
        .spacing(10)
        .align_items(Alignment::Center))
        .on_press(Message::PagesState(PagesState::Home))
        .style(theme::Button::Positive)
        .width(Length::Fixed(100.0))
        .height(Length::Fixed(50.0))
        .padding(10);

    let controls = row![undobutton, space, savebutton];

    let settingtext1 = text("Save the screenshot to the default location")
        .width(Length::Fill)
        .size(18)
        .horizontal_alignment(alignment::Horizontal::Center)
        .vertical_alignment(alignment::Vertical::Center);


    let space1 = horizontal_space(Length::Fill);

    let settinginput1 = toggler(
        String::from(""),
        toggler_value,
        |b|Message::TogglerToggled(b)
        ,
    )
        .width(Length::Shrink)
        .spacing(10);

    let setting1 = row![settingtext1, space1, settinginput1];

    let container1 = Container::new(setting1)
        .style(style::settings_container)
        .height(80)
        .width(Length::Fill)
        .padding(10)
        .align_y(alignment::Vertical::Center);



    let settingtext2 = text("Copy the screenshot into the clipdoard")
        .width(Length::Fill)
        .size(18)
        .horizontal_alignment(alignment::Horizontal::Center)
        .vertical_alignment(alignment::Vertical::Center);


    let space2 = horizontal_space(Length::Fill);

    let settinginput2 = toggler(
        String::from(""),
        toggler_value,
        |b|Message::TogglerToggled(b)
        ,
    )
        .width(Length::Shrink)
        .spacing(10);

    let setting2 = row![settingtext2, space2, settinginput2];

    let container2 = Container::new(setting2)
        .style(style::settings_container)
        .height(80)
        .width(Length::Fill)
        .padding(10)
        .align_y(alignment::Vertical::Center);



    let settingtext3 = text("Select the monitor in which to screenshot")
        .width(Length::Fill)
        .size(18)
        .horizontal_alignment(alignment::Horizontal::Center)
        .vertical_alignment(alignment::Vertical::Center);


    let space3 = horizontal_space(Length::Fill);

    let selected_choice = Some(Choice::A);

    let a = Radio::new(
        "Display 1",
        Choice::A,
        selected_choice.clone(),
        Message::RadioSelected,
    );

    let b = Radio::new(
        "Display 2",
        Choice::B,
        selected_choice.clone(),
        Message::RadioSelected,
    );

    let c = Radio::new(
        "All Dispaly",
        Choice::C,
        selected_choice.clone(),
        Message::RadioSelected,
    );

    let settinginput3 = row![a, b, c];

    let setting3 = row![settingtext3, space3, settinginput3];

    let container3 = Container::new(setting3)
        .style(style::settings_container)
        .height(80)
        .width(Length::Fill)
        .padding(10)
        .align_y(alignment::Vertical::Center);






    let spacev = vertical_space(Length::Fixed(30.0));
    let spacev2 = vertical_space(Length::Fixed(10.0));
    let spacev3 = vertical_space(Length::Fixed(10.0));


    let content: Element<_> = column![ controls, spacev, container1, container2, container3 ]
        .spacing(20)
        .padding(20)
        .into();

    let scrollable = scrollable(
        container(content)
            .width(Length::Fill)
            .center_x(),
    );

    return container(scrollable).height(Length::Fill).center_y().into();

}

mod style {
    use iced::widget::container;
    use iced::{BorderRadius, Color, Theme};

    pub fn settings_container(theme: &Theme) -> container::Appearance {
        let palette = theme.extended_palette();

        container::Appearance {
            text_color: Some(palette.background.strong.text),
            background: Some(Color::from_rgb(135.0 / 255.0, 206.0 / 255.0, 250.0 / 255.0).into()),
            border_radius: BorderRadius::from(15.0),
            border_width: 3.0,
            border_color: Color::BLACK,
            ..Default::default()
        }
    }
}