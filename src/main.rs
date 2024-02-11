mod ui { pub mod home; pub mod modify; pub mod settings; }
use crate::ui::home::home;
use crate::ui::modify::modify;
use crate::ui::settings::settings;
use iced::{executor};
use iced::widget::{container};
use iced::window;
use iced::{Application, Command, Subscription, Element, Length, Settings, Theme, Size, Event};
use std::{thread, time};
use tokio::sync::mpsc;
use std::cell::RefCell;
use arboard::Clipboard;
use iced::window::{UserAttention};
use multi_platform_screen_grabbing_utility::screenshot::Screenshot;
use image::RgbaImage;
use multi_platform_screen_grabbing_utility::image_handler::ImageHandler;
use multi_platform_screen_grabbing_utility::hotkeys::{check_shortcut_event, generate_current_time_string};
use multi_platform_screen_grabbing_utility::choice::Choice;

pub fn main() -> iced::Result {
    let settings = Settings {
        window: window::Settings {
            size: (350, 120),
            ..Default::default()
        },
        ..Settings::default()
    };
    return ScreenshotGrabber::run(settings);
}

#[derive()]
struct ScreenshotGrabber {
    page_state: PagesState,
    sender: RefCell<Option<mpsc::UnboundedSender<Vec<Option<RgbaImage>>>>>,
    receiver: RefCell<Option<mpsc::UnboundedReceiver<Vec<Option<RgbaImage>>>>>,
    toggler_value_clipboard: bool,
    toggler_value_autosave: bool,
    radio_value_monitor: Choice,
    radio_value_format: Choice,
    timer_value: i32,
    shortcut_value: String,
    path_value: String,
    shortcut_listen: bool,
    screen_result: Vec<Option<RgbaImage>>,
    screen_selected: usize,
    subscription_state: SubscriptionState,
    total_monitor_number: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PagesState {
    Home,
    Settings,
    Modify,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SubscriptionState {
    Screenshotting,
    None,
}

#[derive(Debug, Clone)]
pub enum Message {
    SettingsButton,
    NewScreenshotButton,
    ModifyButton,
    HomeButton,
    SaveButton,
    ScreenDone(Vec<Option<RgbaImage>>),
    TogglerToggledAutosave(bool),
    TogglerToggledClipboard(bool),
    RadioSelectedMonitor(Choice),
    RadioSelectedFormat(Choice),
    TimerChange(i32),
    ShortcutListen(bool),
    InputPath,
    EventOccurred(Event),
    ChangeSelectedScreen(usize)
}

impl Application for ScreenshotGrabber {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        let (tx, rx) = mpsc::unbounded_channel::<Vec<Option<RgbaImage>>>();
        return (ScreenshotGrabber {
            page_state: PagesState::Home,
            sender: RefCell::new(Some(tx)),
            receiver: RefCell::new(Some(rx)),
            toggler_value_clipboard: true,
            toggler_value_autosave: false,
            radio_value_monitor: Choice::A,
            radio_value_format: Choice::A,
            timer_value: 0,
            shortcut_value: "CTRL + s".to_string(),
            shortcut_listen: false,
            path_value: "".to_string(),
            screen_result: Vec::new(),
            screen_selected: 0,
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
                let timer_value = self.timer_value.clone();
                let radio_value_monitor = self.radio_value_monitor.to_numeric().clone();
                self.subscription_state = SubscriptionState::Screenshotting;
                thread::spawn(move || {
                    thread::sleep(time::Duration::from_millis((timer_value * 1000 + 500) as u64));
                    let mut screen_results = Vec::new();
                    if radio_value_monitor != 6 {
                        match Screenshot::capture_screen(radio_value_monitor - 1) {
                            Ok(res) => {
                                let img = res.convert().unwrap();
                                screen_results.push(Some(img));
                            }
                            Err(err) => {
                                screen_results.push(None);
                                eprintln!("Error: {}", err);
                            }
                        }
                    } else {
                        match Screenshot::capture_all() {
                            Ok(res) => {
                                for screen in res {
                                    let img = screen.convert().unwrap();
                                    screen_results.push(Some(img));
                                }
                            }
                            Err(err) => {
                                screen_results.push(None);
                                eprintln!("Error: {}", err);
                            }
                        }
                    }
                    sender.take().as_mut().unwrap().send(screen_results).unwrap();
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
                if self.screen_result.is_empty() {
                    return window::resize(Size::new(350, 120));
                } else {
                    return Command::none();
                }
            }
            Message::SaveButton => {
                if !self.screen_result.is_empty() {
                    if let Some(img) = self.screen_result[self.screen_selected].clone() {
                        let current_time_string = generate_current_time_string();
                        let path = std::env::current_dir().unwrap();
                        let imghndl: ImageHandler = img.clone().into();
                        let res = rfd::FileDialog::new()
                            .set_file_name(current_time_string)
                            .set_directory(&path)
                            .add_filter("png", &["png"])
                            .add_filter("jpg", &["jpg"])
                            .add_filter("gif", &["gif"])
                            .save_file();
                        match res {
                            Some(save_path) => {
                                ImageHandler::save_image(&imghndl, save_path);
                            }
                            None => ()
                        }
                    }
                }
                return Command::none();
            }
            Message::ChangeSelectedScreen(value) => {
                self.screen_selected = value;
                return Command::none();
            }
            Message::ScreenDone(images) => {
                self.screen_result = images.clone();
                self.screen_selected = 0;
                self.subscription_state = SubscriptionState::None;
                let (tx, rx) = mpsc::unbounded_channel::<Vec<Option<RgbaImage>>>();
                self.sender = RefCell::new(Some(tx));
                self.receiver = RefCell::new(Some(rx));
                if self.toggler_value_clipboard {
                    match Clipboard::new() {
                        Ok(mut cb) => {
                            for screen in &self.screen_result {
                                match screen {
                                    Some(img) => {
                                        let img_clipboard: ImageHandler = img.clone().into();
                                        if let Err(err) = img_clipboard.to_clipboard(&mut cb) {
                                            eprintln!("Error copying image to clipboard: {:?}", err);
                                        }
                                    }
                                    None => (),
                                }
                            }
                        }
                        Err(err) => {
                            eprintln!("Error creating clipboard: {:?}", err);
                        }
                    }
                }
                if self.toggler_value_autosave {
                    for (index, screen) in self.screen_result.iter().enumerate() {
                        match screen {
                            Some(img) => {
                                let current_time_string = generate_current_time_string();
                                let filename = format!("{}{}_{}{}", self.path_value, current_time_string, index, self.radio_value_format.to_format());
                                let imghndl: ImageHandler = img.clone().into();
                                ImageHandler::save_image(&imghndl, filename.into());
                            }
                            None => ()
                        }
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
            Message::ShortcutListen(value) => {
                self.shortcut_listen = value;
                return Command::none();
            }
            Message::InputPath => {
                let res = rfd::FileDialog::new().pick_folder();
                match res {
                    Some(path) => {
                        self.path_value = path.display().to_string();
                    }
                    None => ()
                }
                return Command::none();
            }
            Message::EventOccurred(event) => {
                println!("{event:?}");
                if check_shortcut_event(&event) == self.shortcut_value {
                    return Command::perform(async { Message::NewScreenshotButton }, |msg| msg);
                }
                if self.shortcut_listen {
                    if check_shortcut_event(&event) != "".to_string() {
                        self.shortcut_value = check_shortcut_event(&event);
                        self.shortcut_listen = false;
                    }
                }
                if !self.screen_result.is_empty() {
                    if self.screen_result[self.screen_selected].is_some() && event == Event::Window(window::Event::Focused) {
                        return window::resize(Size::new(1000, 500));
                    }
                }
                return Command::none();
            }
        }
    }

    fn view(&self) -> Element<Message> {
        return container(
            match self.page_state {
                PagesState::Home => home(self.screen_result.clone(), self.screen_selected.clone(), self.toggler_value_autosave.clone()),
                PagesState::Settings => settings(self.toggler_value_autosave.clone(), self.toggler_value_clipboard.clone(), self.radio_value_monitor, self.radio_value_format, self.timer_value.clone(), self.shortcut_value.clone(), self.path_value.clone(), self.total_monitor_number.clone(), self.shortcut_listen.clone()),
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
                        let mut images = Vec::new();
                        while images.is_empty() {
                            match receiver.as_mut().unwrap().recv().await {
                                Some(imgs) => {
                                    for screen in imgs {
                                        images.push(screen);
                                    }
                                }
                                None => ()
                            };
                        }
                        return (Message::ScreenDone(images), receiver);
                    },
                );
            }
            SubscriptionState::None => {
                return iced::subscription::events().map(Message::EventOccurred);
            }
        }
    }
}