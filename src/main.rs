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
    /*sender: mpsc::Sender<i32>,
    receiver: mpsc::Receiver<i32>,*/
    sender: RefCell<Option<mpsc::UnboundedSender<i32>>>,
    receiver: RefCell<Option<mpsc::UnboundedReceiver<i32>>>,
    message: i32,
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
    ScreenDone,
    ScreenNotDone
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
