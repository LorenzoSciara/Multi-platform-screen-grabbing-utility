mod UI{pub mod home; pub mod modify; pub mod settings;}
use crate::UI::home::home;
use crate::UI::modify::modify;
use crate::UI::settings::settings;
use iced::{executor, Size};
use iced::widget::{button, column, container};
use iced::window;
use iced::{Alignment, Application, Command, Element, Length, Settings, Theme};
use std::thread;
use std::time::Duration;

pub fn main() /*-> iced::Result*/ { //Il main non ritorna per permettere la programmazione multithread
    let handle = thread::spawn(|| {
        // Il thread dorme per 10 secondi
        thread::sleep(Duration::from_secs(10));
        println!("Il thread è stato in pausa per 10 secondi.");
    });

    let settings = Settings {
        window: window::Settings {
            size: (350, 80), // Imposta le dimensioni della finestra
            ..Default::default()
        },
        ..Settings::default()
    };
    Screenshot::run(settings);
    handle.join().unwrap();
}

struct Screenshot {
    pageState: PagesState,
    screenState: ScreenState,
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

#[derive(Debug, Clone, Copy)]
pub enum Message {
    PagesState(PagesState),
    ScreenState(ScreenState),
}

impl Application for Screenshot {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        return (Screenshot {
            pageState: PagesState::Home,
            screenState: ScreenState::ScreenFalse,
        }, Command::none());
    }

    fn title(&self) -> String {
        String::from("Exit - Iced")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::PagesState(filter) => {
                self.pageState=filter;
                return Command::none();
            },
            Message::ScreenState(filter) => {
                self.screenState=filter;
                return window::minimize(true);
            }
        }
    }

    fn view(&self) -> Element<Message> {
        return container(
            match self.pageState {
                PagesState::Home => match self.screenState {
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
