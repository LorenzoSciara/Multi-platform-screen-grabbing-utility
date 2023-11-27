mod UI{pub mod home; pub mod modify; pub mod settings;}
use crate::UI::home::home;
use crate::UI::modify::modify;
use crate::UI::settings::settings;

use std::iter::Filter;
use std::path::Path;
use iced::{Element, Sandbox, Settings, Alignment, Length, alignment, theme, window};
use iced::Background::Color;
use iced::widget::{button, row, text, column, image, Image, container};


struct Screenshot {
    name: String,
    pageState: PagesState,
    screenState: ScreenState,
    image: image::Handle,
}

#[derive(Debug, Clone)]
pub enum Message{
    PagesState(PagesState),
    ScreenState(ScreenState),
}

#[derive(
Debug, Clone, Copy, PartialEq, Eq
)]
pub enum PagesState{
    Home,
    Settings,
    Modify,
}

#[derive(
Debug, Clone, Copy, PartialEq, Eq
)]
pub enum ScreenState{
    ScreenTrue,
    ScreenFalse,
}


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

impl Sandbox for Screenshot {
    type Message = Message;
    fn new() -> Screenshot {
        Screenshot {
            name: "Empty".to_string(),
            pageState: PagesState::Home,
            screenState: ScreenState::ScreenFalse,
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
            },
            Message::ScreenState(filter) => {
                self.screenState=filter;
            }
        }
    }

    fn view(&self) -> Element<'static, Message> {
        // We use a column: a simple vertical layout
        return container(
            match self.pageState {
                PagesState::Home=> match self.screenState {
                    ScreenState::ScreenTrue => home(ScreenState::ScreenTrue),
                    ScreenState::ScreenFalse => home(ScreenState::ScreenFalse),
                },
                PagesState::Settings => settings(self.screenState),
                PagesState::Modify => modify(),
            })
            .width(Length::Fill)
            .padding(25)
            .center_x()
            .center_y()
            .into();
    }
}
