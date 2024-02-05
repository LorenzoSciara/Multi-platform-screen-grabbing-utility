mod ui {
    pub mod home;
    pub mod modify;
    pub mod settings;
}

use crate::ui::home::home;
use crate::ui::modify::modify;
use crate::ui::settings::settings;
use iced::{executor};
use iced::widget::{container};
use iced::window;
use iced::event;
use iced::{Application, Command, Subscription, Element, Length, Settings, Theme, Size, Event};
use std::{io, thread, time};
use std::alloc::System;
use tokio::sync::mpsc;
use std::cell::RefCell;
use std::time::{Duration};
use chrono::{Datelike, prelude::*};
use iced::window::{screenshot, UserAttention};
use multi_platform_screen_grabbing_utility::hotkeys::{HotkeyListener, HotkeyConfig};
use multi_platform_screen_grabbing_utility::screenshot::Screenshot;
use image::RgbaImage;
use multi_platform_screen_grabbing_utility::image_handler::ImageHandler;
pub fn main() -> iced::Result { //Il main non ritorna per permettere la programmazione multithread

    let settings = Settings {
        window: window::Settings {
            size: (350, 120), // Imposta le dimensioni della finestra
            ..Default::default()
        },
        ..Settings::default()
    };
    return ScreenshotGrabber::run(settings);
}

#[derive()]
struct ScreenshotGrabber {
    page_state: PagesState,
    sender: RefCell<Option<mpsc::UnboundedSender<Option<RgbaImage>>>>,
    receiver: RefCell<Option<mpsc::UnboundedReceiver<Option<RgbaImage>>>>,
    toggler_value_clipboard: bool,
    toggler_value_autosave: bool,
    radio_value_monitor: Choice,
    radio_value_format: Choice,
    timer_value: i32,
    input_value: String,
    input_state: text_input::State,
    screen_result: Option<RgbaImage>,
    subscription_state: SubscriptionState,
    total_monitor_number: usize,
}

#[derive(
Debug, Clone, Copy, PartialEq, Eq
)]
pub enum PagesState {
    Home,
    Settings,
    Modify,
}

#[derive(
Debug, Clone, Copy, PartialEq, Eq
)]
pub enum SubscriptionState {
    Screenshotting,
    None,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Choice {
    A,
    B,
    C,
    D,
    E,
    F,
}

impl Choice {
    fn to_numeric(&self) -> u32 {
        match self {
            Choice::A => 1,
            Choice::B => 2,
            Choice::C => 3,
            Choice::D => 4,
            Choice::E => 5,
            Choice::F => 6,
        }
    }
    fn to_format(&self) -> String {
        match self {
            Choice::A => ".jpg".to_string(),
            Choice::B => ".png".to_string(),
            Choice::C => ".gif".to_string(),
            _ => "".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    SettingsButton,
    NewScreenshotButton,
    ModifyButton,
    HomeButton,
    SaveButton,
    ScreenDone(Option<RgbaImage>),
    TogglerToggledAutosave(bool),
    TogglerToggledClipboard(bool),
    RadioSelectedMonitor(Choice),
    RadioSelectedFormat(Choice),
    TimerChange(i32),
    TextInputShortcut(String),
    TextInputPath(String),
    EventOccurred(Event),
}

impl Application for ScreenshotGrabber {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        let (tx, rx) = mpsc::unbounded_channel::<Option<RgbaImage>>();
        return (ScreenshotGrabber {
            page_state: PagesState::Home,
            sender: RefCell::new(Some(tx)),
            receiver: RefCell::new(Some(rx)),
            toggler_value_clipboard: true,
            toggler_value_autosave: false,
            radio_value_monitor: Choice::A,
            radio_value_format: Choice::A,
            timer_value: 0,
            shortcut_value: String::new(),
            path_value: "".to_string(),
            screen_result: None,
            subscription_state: SubscriptionState::None,
            total_monitor_number: Screenshot::monitors_num(),
        }, Command::none());
    }

    fn title(&self) -> String {
        String::from("Exit - Iced")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::NewScreenshotButton => {
                let sender = self.sender.clone();
                let screen_result = self.screen_result.clone();
                let timer_value = self.timer_value.clone();
                let radio_value_monitor = self.radio_value_monitor.to_numeric().clone();
                self.subscription_state = SubscriptionState::Screenshotting;
                thread::spawn(move || {
                    thread::sleep(time::Duration::from_millis((timer_value*1000 + 500) as u64)); //Aspetto che si chiuda l'applicazione e faccio lo screen
                    let screen_result ;
                    if radio_value_monitor != 6 {
                        match Screenshot::capture_screen(radio_value_monitor-1) {
                            Ok(res) => {
                                println!("Screen ok");
                                let img = res.convert().unwrap();
                                //img.save(format!("screen_file.png")); //qua come lo usava fra il result di save() non è usato non va bene
                                screen_result = Some(img);
                            }
                            Err(err) => {
                                screen_result = None;
                                eprintln!("Error: {}", err); //gestire l'errore non puo essere un print in console, ma un banner rosso che appare a livello grafico con iced in home.rs
                            }
                        }
                    } else {
                        match Screenshot::capture_all() { //TO DO: gestire piu monitor
                            Ok(res) => {
                                println!("Screen ok");
                                let img = res[0].convert().unwrap();
                                //img.save(format!("screen_file.png")); //qua come lo usava fra il result di save() non è usato non va bene
                                screen_result = Some(img);
                            }
                            Err(err) => {
                                screen_result = None;
                                eprintln!("Error: {}", err); //gestire l'errore non puo essere un print in console, ma un banner rosso che appare a livello grafico con iced in home.rs
                            }
                        }
                    }
                    sender.take().as_mut().unwrap().send(screen_result).unwrap();
                });
                return window::minimize(true);
            }
            Message::SettingsButton => {
                self.page_state = PagesState::Settings;
                return window::resize(Size::new(1000, 500));
            }
            Message::ModifyButton => {
                self.page_state = PagesState::Modify;
                return window::resize(Size::new(700, 500));
            }
            Message::HomeButton => {
                self.page_state = PagesState::Home;
                if self.screen_result == None {
                    return window::resize(Size::new(350, 120));
                } else {
                    return Command::none();
                }
            }

            Message::SaveButton => {
                //if self.toggler_value_clipboard {
                //    let img_clipboard : ImageHandler = img.clone().into();
                //    img_clipboard.to_clipboard();
                //}
                return Command::none();
            }
            Message::ScreenDone(image) => {
                self.screen_result = image;
                self.subscription_state=SubscriptionState::None;
                let (tx, rx) = mpsc::unbounded_channel::<Option<RgbaImage>>();
                self.sender= RefCell::new(Some(tx));
                self.receiver= RefCell::new(Some(rx));
                if self.toggler_value_autosave{
                    match self.screen_result.clone() {
                        Some(img) => {
                            let current_time = Utc::now();
                            let current_time_string = format!(
                                "Screenshot_{:04}_{:02}_{:02}_{:02}_{:02}_{:02}",
                                current_time.year(),
                                current_time.month(),
                                current_time.day(),
                                current_time.hour(),
                                current_time.minute(),
                                current_time.second()
                            );
                            let save_result = img.save(format!("{}{}{}", self.path_value, current_time_string, self.radio_value_format.to_format()));
                            println!("{}{}{}", self.path_value, current_time_string, self.radio_value_format.to_format());

                            if self.toggler_value_clipboard {
                                let img_clipboard : ImageHandler = img.clone().into();
                                img_clipboard.to_clipboard();
                            }

                            match save_result {
                                Ok(_) => (),
                                Err(e) => {println!("{}",e)}
                            };
                        }
                        None => ()
                    }
                }

                return window::request_user_attention(Some(UserAttention::Informational));
            }
            Message::TogglerToggledAutosave(value) => {
                self.toggler_value_autosave = value;
                return Command::none();
            }
            Message::TogglerToggledClipboard(value) => {
                self.toggler_value_clipboard = value;
                return Command::none();
            }
            Message::RadioSelectedMonitor(value) => {
                self.radio_value_monitor = value;
                return Command::none();
            }
            Message::RadioSelectedFormat(value) => {
                self.radio_value_format = value;
                return Command::none();
            }
            Message::TimerChange(value) => {
                self.timer_value = value;
                return Command::none();
            }
            Message::TextInputShortcut(value) => {
                self.shortcut_value = value;
                return Command::none();
            }
            Message::TextInputPath(value) => {
                self.path_value = value;
                return Command::none();
            }
            Message::EventOccurred(event) => {
                println!("{event:?}");
                if self.screen_result.is_some() && event == Event::Window(window::Event::Focused){
                    return window::resize(Size::new(1000, 500));
                }
                return Command::none();
            }
        }
    }

    fn view(&self) -> Element<Message> {
        return container(
            match self.page_state {
                PagesState::Home => home(self.screen_result.clone(), self.toggler_value_autosave.clone()),
                PagesState::Settings => settings(self.toggler_value_autosave.clone(), self.toggler_value_clipboard.clone(), self.radio_value_monitor, self.radio_value_format, self.timer_value.clone(), self.shortcut_value.clone(), self.path_value.clone(), self.total_monitor_number.clone()),
                PagesState::Modify => modify(),
            })
            .width(Length::Fill)
            .padding(25)
            .center_x()
            .center_y()
            .into();
    }

    fn subscription(&self) -> Subscription<Message> {
        match self.subscription_state {
            SubscriptionState::Screenshotting => {
                return iced::subscription::unfold(
                    "channel",
                    self.receiver.take(),
                    move |mut receiver| async move {
                        let mut image = None;
                        while image == None{
                            image = match receiver.as_mut().unwrap().recv().await {
                                Some(img) => img,
                                None => None
                            };
                        }
                        return (Message::ScreenDone(image), receiver);
                    },
                );
            }
            SubscriptionState::None => {
                return iced::subscription::events().map(Message::EventOccurred);
            }
        }
    }
}
