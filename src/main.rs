mod UI{pub mod home; pub mod modify; pub mod settings;}
use crate::UI::home::home;
use crate::UI::modify::modify;
use crate::UI::settings::settings;

use std::iter::Filter;
use std::path::Path;
use iced::{Element, Sandbox, Settings, Alignment, Length, alignment, theme, window};
use iced::Background::Color;
use iced::widget::{button, row, text, column, image, Image, container};



pub fn main() -> iced::Result {
    let settings = Settings {
        window: window::Settings {
            size: (350, 80), // Imposta le dimensioni della finestra
            ..Default::default()
        },
        ..Settings::default()
    };
    Screenshot::run(settings)
}

struct Screenshot {
    name: String,
    pageState: PagesState,
    image: image::Handle,
}

#[derive(Debug, Clone)]
pub enum Message{
    PagesState(PagesState),
}

#[derive(
Debug, Clone, Copy, PartialEq, Eq
)]
pub enum PagesState{
    Home(ScreenState),
    Settings,
    Modify,
}

#[derive(
Debug, Clone, Copy, PartialEq, Eq, Default,
)]
pub enum ScreenState{
    #[default]
    ScreenTrue,
    ScreenFalse,
}

impl Sandbox for Screenshot {
    type Message = Message;
    fn new() -> Screenshot {
        Screenshot {
            name: "Empty".to_string(),
            pageState: PagesState::Home(ScreenState::ScreenFalse),
            image: image::Handle::from_path(Path::new("./resources/empty-image.png")),
        }
    }

    fn title(&self) -> String {
        String::from("Multi-platform Screen-grabbing Utility")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::PagesState(filter) => {
                    self.pageState=filter;
            }
        }
    }

    fn view(&self) -> Element<'static, Message> {
        // We use a column: a simple vertical layout
        return container(
            match self.pageState {
                PagesState::Home(ScreenState::ScreenTrue)=> home(ScreenState::ScreenTrue),
                PagesState::Home(ScreenState::ScreenFalse) => home(ScreenState::ScreenFalse),
                PagesState::Settings => settings(),
                PagesState::Modify => modify(),
            })
            .width(Length::Fill)
            .padding(25)
            .center_x()
            .center_y()
            .into();
    }
}
