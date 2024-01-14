mod UI{pub mod home; pub mod modify; pub mod settings;}
use crate::UI::home::home;
use crate::UI::modify::modify;
use crate::UI::settings::settings;
use iced::{executor, settings};
use iced::widget::{button, column, container};
use iced::window;
use iced::{Alignment, Application, Command, Subscription, Element, Length, Settings, Theme, Size};
use std::thread;
//use std::sync::mpsc;
use tokio::sync::mpsc;
use std::cell::RefCell;
use std::time::Duration;

pub fn main() -> iced::Result { //Il main non ritorna per permettere la programmazione multithread

    let settings = Settings {
        window: window::Settings {
            size: (350, 80), // Imposta le dimensioni della finestra
            ..Default::default()
        },
        ..Settings::default()
    };
    return Screenshot::run(settings);
}

struct Screenshot {
    pageState: PagesState,
    screenState: ScreenState,
    sender: RefCell<Option<mpsc::UnboundedSender<i32>>>,
    receiver: RefCell<Option<mpsc::UnboundedReceiver<i32>>>,
    message: i32,
    toggler_value_clipboard: bool,
    toggler_value_autosave: bool,
    radio_value_monitor: Choice,
    radio_value_format: Choice
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


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Choice {
    A,
    B,
    C,
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    PagesState(PagesState),
    ScreenState(ScreenState),
    ScreenDone,
    ScreenNotDone,
    TogglerToggledAutosave(bool),
    TogglerToggledClipboard(bool),
    RadioSelectedMonitor(Choice),
    RadioSelectedFormat(Choice),
}

impl Application for Screenshot {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(flags: ()) -> (Self, Command<Message>) {
        //let (tx, rx) = mpsc::channel();
        let (tx, rx) = mpsc::unbounded_channel::<i32>();
        return (Screenshot {
            pageState: PagesState::Home,
            screenState: ScreenState::ScreenFalse,
            sender: RefCell::new(Some(tx)),
            receiver: RefCell::new(Some(rx)),
            message: -1,
            toggler_value_clipboard: true,
            toggler_value_autosave: true,
            radio_value_monitor: Choice::A,
            radio_value_format: Choice::A,
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
                self.screenState = filter;
                let sender = self.sender.clone();
                thread::spawn(move|| {
                    thread::sleep(Duration::from_secs(5));
                    let i = 1;
                    sender.take().as_mut().unwrap().send(i).unwrap();
                    println!("{} Messaggio inviato", i);
                    thread::sleep(Duration::from_millis(200));
                });
                return window::minimize(true);
            },
            Message::ScreenDone => {
                return window::resize(Size::new(700, 500));
            }
            Message::ScreenNotDone => {
                return Command::none();
            }
            Message::TogglerToggledAutosave(value) => { self.toggler_value_autosave = value;
                return Command::none();}
            Message::TogglerToggledClipboard(value) => { self.toggler_value_clipboard = value;
                return Command::none();}
            Message::RadioSelectedMonitor(Choice) => { self.radio_value_monitor = Choice;
                return Command::none();}
            Message::RadioSelectedFormat(Choice) => { self.radio_value_format = Choice;
                return Command::none();}
        }
    }

    fn view(&self) -> Element<Message> {
        return container(
            match self.pageState {
                PagesState::Home => match self.screenState {
                    ScreenState::ScreenTrue => home(ScreenState::ScreenTrue),
                    ScreenState::ScreenFalse => home(ScreenState::ScreenFalse),
                },
                PagesState::Settings => settings(self.screenState, self.toggler_value_autosave, self.toggler_value_clipboard, self.radio_value_monitor, self.radio_value_format ),
                PagesState::Modify => modify(),
            })
            .width(Length::Fill)
            .padding(25)
            .center_x()
            .center_y()
            .into();
    }

    fn subscription(&self) -> Subscription<Message> {
        //let receiver = self.receiver.clone();
        iced::subscription::unfold(
            "led changes",
            self.receiver.take(),
            move |mut receiver| async move {
                /*if receiver == 1 {
                    return Message::ScreenDone;
                }
                else {
                    return Message::ScreenNotDone;
                }*/
                let num = receiver.as_mut().unwrap().recv().await.unwrap();
                return (Message::ScreenDone, receiver);
            },
        )
    }
}
