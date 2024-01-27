mod ui {pub mod home; pub mod modify; pub mod settings;}
use crate::ui::home::home;
use crate::ui::modify::modify;
use crate::ui::settings::settings;
use iced::{executor};
use iced::widget::{container};
use iced::window;
use iced::{Application, Command, Subscription, Element, Length, Settings, Theme, Size};
use std::{io, thread, time};
use tokio::sync::mpsc;
use std::cell::RefCell;
use std::time::Duration;
use iced::window::{UserAttention};
use multi_platform_screen_grabbing_utility::hotkeys::{HotkeyListener,HotkeyConfig};

use multi_platform_screen_grabbing_utility::screenshot::Screenshot;
use image::RgbaImage;
use screenshots::{Screen};

pub fn main() -> iced::Result { //Il main non ritorna per permettere la programmazione multithread

    let settings = Settings {
        window: window::Settings {
            size: (350, 100), // Imposta le dimensioni della finestra
            ..Default::default()
        },
        ..Settings::default()
    };
    return ScreenshotGrabber::run(settings);
}

#[derive()]
struct ScreenshotGrabber {
    page_state: PagesState,
    sender: RefCell<Option<mpsc::UnboundedSender<i32>>>,
    receiver: RefCell<Option<mpsc::UnboundedReceiver<i32>>>,
    toggler_value_clipboard: bool,
    toggler_value_autosave: bool,
    radio_value_monitor: Choice,
    radio_value_format: Choice,
    timer_value: i32,
    shortcut_value: String,
    path_value: String,
    screen_result: Option<RgbaImage> //si puo cambiare anche tipo dell'option se riesci a covertire RgbaImage in un formato raw simile ad un file con https://docs.rs/image/latest/image/type.RgbaImage.html
}

#[derive(
Debug, Clone, Copy, PartialEq, Eq
)]
pub enum PagesState{
    Home,
    Settings,
    Modify,
}




#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Choice {
    A,
    B,
    C,
}

impl Choice {
    fn to_numeric(&self) -> u32 {
        match self {
            Choice::A => 1,
            Choice::B => 2,
            Choice::C => 3,
        }
    }
    fn to_format(&self) -> String {
        match self {
            Choice::A => "png".to_string(),
            Choice::B => "jpg".to_string(),
            Choice::C => "gif".to_string(),
        }
    }

}

#[derive(Debug, Clone)]
pub enum Message {
    SettingsButton,
    NewScreenshotButton,
    ModifyButton,
    HomeButton,
    ScreenDone,
    TogglerToggledAutosave(bool),
    TogglerToggledClipboard(bool),
    RadioSelectedMonitor(Choice),
    RadioSelectedFormat(Choice),
    TimerChange(i32),
    Shortcut(String),
    Path(String),
}

impl Application for ScreenshotGrabber {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        //let (tx, rx) = mpsc::channel();
        let (tx, rx) = mpsc::unbounded_channel::<i32>();
        return (ScreenshotGrabber {
            page_state: PagesState::Home,
            sender: RefCell::new(Some(tx)),
            receiver: RefCell::new(Some(rx)),
            toggler_value_clipboard: true,
            toggler_value_autosave: true,
            radio_value_monitor: Choice::A,
            radio_value_format: Choice::A,
            timer_value: 0,
            shortcut_value: String::new(),
            path_value: String::new(),
            screen_result: None, //l'ho fatto perchè non so inizilizzare un RgbaImage a "zero", alternativa inizializzare con un immagine con una scritta "screen not found/make a new screen"
        }, Command::none());
    }

    fn title(&self) -> String {
        String::from("Exit - Iced")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::NewScreenshotButton => {
                let monitor_value = self.radio_value_monitor.to_numeric();
                match Screenshot::capture_screen(monitor_value) {
                    Ok(res) => {
                        let img = res.convert().unwrap();
                        img.save(format!("screen_file.png")); //qua come lo usava fra il result di save() non è usato non va bene
                        self.screen_result = Some(img);
                    }
                    Err(err) => {
                        self.screen_result = None;
                        eprintln!("Error: {}", err); //gestire l'errore non puo essere un print in console, ma un banner rosso che appare a livello grafico con iced in home.rs
                    }
                }

                //metti tu il thread dove volesi, probabilmente nel Ok() per far chiudere la window
                let sender = self.sender.clone();
                thread::spawn(move|| {
                    let i = 1;
                    sender.take().as_mut().unwrap().send(i).unwrap();
                });
                return window::minimize(true);
            },
            Message::SettingsButton => {
                self.page_state =PagesState::Settings;
                return window::resize(Size::new(800, 800));
            },
            Message::ModifyButton => {
                self.page_state =PagesState::Modify;
                return window::resize(Size::new(700, 500));
            },
            Message::HomeButton => {
                self.page_state =PagesState::Home;
                if self.screen_result == true {
                    return Command::none();
                }
                else{
                    return window::resize(Size::new(350, 100));
                }
            },
            Message::ScreenDone => {
                println!("Screen done arrivato");
                return window::request_user_attention(Some(UserAttention::Informational));
                //return window::resize(Size::new(800, 800));
            },
            Message::TogglerToggledAutosave(value) => { self.toggler_value_autosave = value;
                return Command::none();
            },
            Message::TogglerToggledClipboard(value) => { self.toggler_value_clipboard = value;
                return Command::none();
            },
            Message::RadioSelectedMonitor(value) => { self.radio_value_monitor = value;
                return Command::none();
            },
            Message::RadioSelectedFormat(value) => { self.radio_value_format = value;
                return Command::none();
            },
            Message::TimerChange(value) => { self.timer_value = value;
                return Command::none();
            },
            Message::Shortcut(value) => { self.shortcut_value = value;
                return Command::none();
            },
            Message::Path(value) => { self.path_value = value;
                return Command::none();
            },
        }
    }

    fn view(&self) -> Element<Message> {
        return container(
            match self.page_state {
                PagesState::Home => home(self.screen_result.clone()),
                PagesState::Settings => settings(self.toggler_value_autosave, self.toggler_value_clipboard, self.radio_value_monitor, self.radio_value_format, self.timer_value, self.shortcut_value.clone(), self.path_value.clone() ),
                PagesState::Modify => modify(),
            })
            .width(Length::Fill)
            .padding(25)
            .center_x()
            .center_y()
            .into();
    }

    fn subscription(&self) -> Subscription<Message> {
        iced::subscription::unfold(
            "channel",
            self.receiver.take(),
            move |mut receiver| async move {
                let num = receiver.as_mut().unwrap().recv().await.unwrap();
                println!("{} messaggio arrivato", num);
                return (Message::ScreenDone, receiver);
            },
        )
    }
}
