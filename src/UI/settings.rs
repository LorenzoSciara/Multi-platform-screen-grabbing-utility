use std::iter::Filter;
use std::path::Path;
use iced::{Element, Sandbox, Settings, Alignment, Length, alignment, theme, Color};
use iced::widget::{button, row, text, column, container};
use crate::{Message, ScreenState, PagesState, Choice};

use iced::widget::{checkbox, horizontal_space, radio, scrollable, slider, text_input, toggler, vertical_space, Text, Radio};
use iced::widget::{Button, Column, Container, Slider, Toggler};
use iced::{Font, Pixels, Renderer};

use iced::widget::container::{Appearance, StyleSheet};



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

fn radio_container(radio_value: Choice, radio_type: String) -> Container<'static, Message> {


    let selected_choice = Some(radio_value);

    let a = Radio::new(
        if radio_type=="monitor" {"Display 1"} else {".jpg"},
        Choice::A,
        selected_choice,
        if radio_type=="monitor" {Message::RadioSelectedMonitor} else {Message::RadioSelectedFormat},
    );

    let b = Radio::new(
        if radio_type=="monitor" {"Display 2"} else {".png"},
        Choice::B,
        selected_choice,
        if radio_type=="monitor" {Message::RadioSelectedMonitor} else {Message::RadioSelectedFormat},
    );

    let c = Radio::new(
        if radio_type=="monitor" {"All Display"} else {".gif"},
        Choice::C,
        selected_choice,
        if radio_type=="monitor" {Message::RadioSelectedMonitor} else {Message::RadioSelectedFormat},
    );

    let settinginput = row![a, b, c];
    let container = Container::new(settinginput);
    return container;

}


fn toggler_container(toggler_value: bool, toggler_type: String) -> Container<'static, Message> {
    let settinginput = toggler(
        String::from(""),
        toggler_value,
        move |b|{if toggler_type=="autosave" { Message::TogglerToggledAutosave(b) } else { Message::TogglerToggledClipboard(b) }}
        ,
    )
        .width(Length::Shrink)
        .spacing(10);

    let container = Container::new(settinginput);
    return container;

}
fn settings_box(settings_text: String, settings_container: Container<'static, Message>) -> Container<'static, Message> {
    let settingtext = text(settings_text)
        .width(Length::Fill)
        .size(18)
        .horizontal_alignment(alignment::Horizontal::Center)
        .vertical_alignment(alignment::Vertical::Center);

    let space = horizontal_space(Length::Fill);

    let setting = row![settingtext, space, settings_container];

    let container = Container::new(setting)
        .style(style::settings_container)
        .height(80)
        .width(Length::Fill)
        .padding(10)
        .align_y(alignment::Vertical::Center);

    return container;

}
pub fn settings(screenState: ScreenState, toggler_value_autosave: bool, toggler_value_clipboard: bool, radio_value_monitor: Choice, radio_value_format: Choice) -> Element<'static, Message> {
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
    let spacev = vertical_space(Length::Fixed(30.0));

    let container1 = settings_box("Save the screenshot to the default location".to_string(), toggler_container(toggler_value_autosave, "autosave".to_string()));
    let container2 = settings_box("Copy the screenshot into the clipdoard".to_string(), toggler_container(toggler_value_clipboard, "clipboard".to_string()));
    let container3 = settings_box("Select the monitor in which to screenshot".to_string(), radio_container(radio_value_monitor, "monitor".to_string()));
    let container4 = settings_box("Select the screenshot format".to_string(), radio_container(radio_value_format, "clipboard".to_string()));







    let content: Element<_> = column![ controls, spacev, container1, container2, container3, container4 ]
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