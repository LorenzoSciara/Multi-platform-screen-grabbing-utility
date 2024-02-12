mod ui {
    pub mod home;
    pub mod modify;
    pub mod settings;
}

use crate::ui::home::home;
use crate::ui::modify::modify;
use crate::ui::settings::settings;
use iced::{executor, mouse, Point};
use iced::widget::{container};
use iced::window;
use iced::event;
use iced::{Application, Command, Subscription, Element, Length, Settings, Theme, Size, Event, Rectangle};
use std::{io, thread, time};
use std::alloc::System;
use tokio::sync::mpsc;
use std::cell::RefCell;
use std::time::{Duration};
use chrono::{Datelike, prelude::*};
use iced::advanced::svg::Data::Path;
use iced::window::{screenshot, UserAttention};
use multi_platform_screen_grabbing_utility::hotkeys::{HotkeyListener, HotkeyConfig};
use multi_platform_screen_grabbing_utility::screenshot::Screenshot;
use multi_platform_screen_grabbing_utility::image_handler::ImageHandler;
use screenshots::{Screen};
use rfd::FileDialog;
use env_logger;
use once_cell::sync::Lazy;
use image::Rgba;
use image::{GenericImageView, RgbaImage, SubImage};
use crate::CropMode::Crop;
use rusttype::{Font, Scale};
use crate::Draw::{FreeHand, Nothing};
use imageproc::rect::Rect;

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
    shortcut_value: String,
    path_value: String,
    screen_result: Option<RgbaImage>,
    subscription_state: SubscriptionState,
    total_monitor_number: usize,
    event: Event,
    crop: CropMode,
    crop_start: (i32,i32),
    crop_end: (i32, i32),
    draw: Draw,
    draw_mouse_pressed: bool,
    draw_figure_press: (i32, i32),
    draw_figure_released: (i32, i32),
    draw_text_input: String,
    screen_result_backup: Option<RgbaImage>,
    draw_color_slider_value: u8,
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

#[derive(
Debug, Clone, Copy, PartialEq, Eq
)]
pub enum Draw {
    FreeHand,
    Circle,
    Text,
    Arrow,
    Rectangle,
    Nothing,
    Crop,
}

#[derive(
Debug, Clone, Copy, PartialEq, Eq
)]
pub enum CropMode {
    Crop,
    CropConfirm,
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
    Shortcut(String),
    Path(String),
    EventOccurred(Event),
    ModifyImage(Option<Rectangle>, Option<Event>),
    CropButton,
    DrawFreeButton,
    DrawCircleButton,
    DrawTextButton,
    DrawTextInput(String),
    DrawClearButton,
    DrawArrowButton,
    DrawColorSlider(u8),
}

static SCREENSHOT_CONTAINER: Lazy<container::Id> = Lazy::new(|| container::Id::new("screenshot"));

impl Application for ScreenshotGrabber {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        let (tx, rx) = mpsc::unbounded_channel::<Option<RgbaImage>>();
        env_logger::init();
        return (ScreenshotGrabber {
            page_state: PagesState::Home,
            sender: RefCell::new(Some(tx)),
            receiver: RefCell::new(Some(rx)),
            toggler_value_clipboard: false,
            toggler_value_autosave: false,
            radio_value_monitor: Choice::A,
            radio_value_format: Choice::A,
            timer_value: 0,
            shortcut_value: String::new(),
            path_value: "".to_string(),
            screen_result: None,
            subscription_state: SubscriptionState::None,
            total_monitor_number: Screenshot::monitors_num(),
            event: Event::Window(window::Event::Focused),
            crop: Crop,
            crop_start: (0, 0),
            crop_end: (0, 0),
            draw: Nothing,
            draw_mouse_pressed: false,
            draw_figure_press: (0, 0),
            draw_figure_released: (0, 0),
            draw_text_input: "".to_string(),
            screen_result_backup: None,
            draw_color_slider_value: 0,
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
                let timer_vaule = self.timer_value.clone();
                self.subscription_state = SubscriptionState::Screenshotting;
                thread::spawn(move || {
                    thread::sleep(time::Duration::from_millis((timer_vaule * 1000 + 500) as u64)); //Aspetto che si chiuda l'applicazione e faccio lo screen
                    let screen_result;
                    match Screenshot::capture_first() {
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
                return window::resize(Size::new(1000, 500));
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
                    None => ()
                }

                return Command::none();
            }

            Message::ScreenDone(image) => {
                self.screen_result = image;
                self.screen_result_backup = self.screen_result.clone();
                self.subscription_state = SubscriptionState::None;
                let (tx, rx) = mpsc::unbounded_channel::<Option<RgbaImage>>();
                self.sender = RefCell::new(Some(tx));
                self.receiver = RefCell::new(Some(rx));
                if self.toggler_value_autosave {
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

                            let imghndl: ImageHandler = img.clone().into();
                            ImageHandler::save_image(&imghndl, format!("{}{}{}", self.path_value, current_time_string, self.radio_value_format.to_format()).into());

                            println!("{}{}{}", self.path_value, current_time_string, self.radio_value_format.to_format());
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
            Message::Shortcut(value) => {
                self.shortcut_value = value;
                return Command::none();
            }
            Message::Path(value) => {
                self.path_value = value;
                return Command::none();
            }
            Message::EventOccurred(event) => {
                self.event = event.clone();
                //println!("{0:?}", self.event);
                if self.screen_result.is_some() && event == Event::Window(window::Event::Focused) {
                    return window::resize(Size::new(1000, 500));
                }
                if self.page_state == PagesState::Modify{
                    return container::visible_bounds(SCREENSHOT_CONTAINER.clone()).map(move |bounds|{Message::ModifyImage(bounds, Some(event.clone()))});
                }
                return Command::none();
            }
            Message::ModifyImage(screenshot_bounds, event) => {
                match self.draw {
                    FreeHand => {
                        let color = Rgba([50u8, 255u8, 0u8, 200u8]);
                        let screen = self.screen_result.clone().unwrap();
                        match event {
                            Some(Event::Mouse(mouse::Event::CursorMoved { position })) => {
                                if screenshot_bounds.unwrap().contains(position) && self.draw_mouse_pressed.clone() {
                                    let position = (((position.x.clone() - screenshot_bounds.unwrap().x.clone()) * 3.2) as i32, ((position.y.clone() - screenshot_bounds.unwrap().y.clone()) * 3.2) as i32);
                                    self.screen_result = Some(imageproc::drawing::draw_filled_circle(&screen, position, 5, color));
                                }
                            }
                            Some(Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left))) => {
                                self.draw_mouse_pressed = true;
                            }
                            Some(Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left))) => {
                                self.draw_mouse_pressed = false;
                            }
                            _ => {}
                        };
                    }
                    Draw::Circle => {
                        let color = Rgba([255, 0, 0, 0]);
                        let screen = self.screen_result.clone().unwrap();
                        match event {
                            Some(Event::Mouse(mouse::Event::CursorMoved { position })) => {
                                if screenshot_bounds.unwrap().contains(position) && self.draw_mouse_pressed.clone() && self.draw_figure_press == (0, 0) {
                                    self.draw_figure_press = (((position.x.clone() - screenshot_bounds.clone().unwrap().x) * 3.2) as i32, ((position.y.clone() - screenshot_bounds.clone().unwrap().y.clone()) * 3.2) as i32);
                                }
                                if screenshot_bounds.unwrap().contains(position) && self.draw_mouse_pressed.clone() && self.draw_figure_press != (0, 0) {
                                    self.draw_figure_released = (((position.x.clone() - screenshot_bounds.clone().unwrap().x) * 3.2) as i32, ((position.y.clone() - screenshot_bounds.clone().unwrap().y) * 3.2) as i32);
                                }
                            }
                            Some(Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left))) => {
                                self.draw_mouse_pressed = true;
                            }
                            Some(Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left))) => {
                                self.draw_mouse_pressed = false;
                                self.screen_result = Some(imageproc::drawing::draw_hollow_circle(&screen, self.draw_figure_press.clone(), (((self.draw_figure_released.0.clone() - self.draw_figure_press.0.clone()).pow(2) + (self.draw_figure_released.1.clone() - self.draw_figure_press.1.clone()).pow(2)) as f64).sqrt() as i32, color));
                                self.draw_figure_press = (0, 0);
                                self.draw_figure_released = (0, 0);
                            }
                            _ => {}
                        };
                    }
                    Draw::Text =>{
                        let color = Rgba([255, 0, 0, 0]);
                        let screen = self.screen_result.clone().unwrap();
                        match event {
                            Some(Event::Mouse(mouse::Event::CursorMoved { position })) => {
                                if screenshot_bounds.unwrap().contains(position) {
                                    self.draw_figure_press = (position.clone().x as i32, position.clone().y as i32);
                                }
                            }
                            Some(Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left))) => {
                                self.screen_result = Some(imageproc::drawing::draw_text(&screen, color, (self.draw_figure_press.0.clone() as f32 *1.75) as i32, (self.draw_figure_press.1.clone() as f32 *1.35) as i32, Scale{x: 24.8, y: 24.8},  &Font::try_from_vec(Vec::from(include_bytes!("DejaVuSans.ttf") as &[u8])).unwrap(), self.draw_text_input.clone().as_str()));
                                self.draw_figure_press = (0, 0);
                            }
                            _ => {}
                        };
                    }
                    Draw::Arrow => {
                        let color = Rgba([255, 0, 0, 0]);
                        let screen = self.screen_result.clone().unwrap();
                        match event {
                            Some(Event::Mouse(mouse::Event::CursorMoved { position })) => {
                                if screenshot_bounds.unwrap().contains(position) && self.draw_mouse_pressed.clone() && self.draw_figure_press == (0, 0) {
                                    self.draw_figure_press = (((position.x.clone() - screenshot_bounds.clone().unwrap().x) * 3.2) as i32, ((position.y.clone() - screenshot_bounds.clone().unwrap().y.clone()) * 3.2) as i32);
                                }
                                if screenshot_bounds.unwrap().contains(position) && self.draw_mouse_pressed.clone() && self.draw_figure_press != (0, 0) {
                                    self.draw_figure_released = (((position.x.clone() - screenshot_bounds.clone().unwrap().x) * 3.2) as i32, ((position.y.clone() - screenshot_bounds.clone().unwrap().y) * 3.2) as i32);
                                }
                            }
                            Some(Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left))) => {
                                self.draw_mouse_pressed = true;
                            }
                            Some(Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left))) => {
                                self.draw_mouse_pressed = false;
                                let slope = (self.draw_figure_released.clone().1 - self.draw_figure_press.clone().1)as f32 / (self.draw_figure_released.clone().0 - self.draw_figure_press.clone().0) as f32;
                                //if self.draw_figure_press.clone().0 <= self.draw_figure_released.clone().0 {
                                    let image_tmp1 = imageproc::drawing::draw_line_segment(&screen, ((self.draw_figure_released.clone().0 as f32 - (30.0 * slope.clone())), (self.draw_figure_released.clone().1 as f32 - (30.0 * slope.clone()))), (self.draw_figure_released.clone().0 as f32, self.draw_figure_released.clone().1 as f32), color);
                                    let image_tmp2 = imageproc::drawing::draw_line_segment(&image_tmp1, ((self.draw_figure_released.clone().0 as f32 - (30.0 * slope.clone())), (self.draw_figure_released.clone().1 as f32 + (30.0 * slope.clone()))), (self.draw_figure_released.clone().0 as f32, self.draw_figure_released.clone().1 as f32), color);
                                    self.screen_result = Some(imageproc::drawing::draw_line_segment(&image_tmp2, (self.draw_figure_press.clone().0 as f32, self.draw_figure_press.clone().1 as f32), (self.draw_figure_released.clone().0 as f32, self.draw_figure_released.clone().1 as f32), color));
                                //}
                                /*else if self.draw_figure_press.clone().0 > self.draw_figure_released.clone().0 {
                                    let image_tmp1 = imageproc::drawing::draw_line_segment(&screen, ((self.draw_figure_released.clone().0 + 30) as f32, (self.draw_figure_released.clone().1 + 30)  as f32), (self.draw_figure_released.clone().0 as f32, self.draw_figure_released.clone().1 as f32), color);
                                    let image_tmp2 = imageproc::drawing::draw_line_segment(&image_tmp1, ((self.draw_figure_released.clone().0 + 30) as f32, (self.draw_figure_released.clone().1 - 30) as f32), (self.draw_figure_released.clone().0 as f32, self.draw_figure_released.clone().1 as f32), color);
                                    self.screen_result = Some(imageproc::drawing::draw_line_segment(&image_tmp2, (self.draw_figure_press.clone().0 as f32, self.draw_figure_press.clone().1 as f32), (self.draw_figure_released.clone().0 as f32, self.draw_figure_released.clone().1 as f32), color));
                                }*/
                                self.draw_figure_press = (0, 0);
                                self.draw_figure_released = (0, 0);
                            }
                            _ => {}
                        };
                    }
                    Draw::Crop => {
                        let screen = self.screen_result.clone().unwrap();
                        let color = Rgba([0u8, 0u8, 0u8, 255u8]);
                        let mut rect = Rect::at(1, 1).of_size(1, 1);
                        match event{
                            Some(Event::Mouse(mouse::Event::CursorMoved { position })) => {
                                //println!("{} {}",position.x,position.y);
                                if screenshot_bounds.unwrap().contains(position) && self.draw_mouse_pressed.clone() && self.crop_start == (0, 0) {
                                    self.crop_start = (((position.x.clone()-screenshot_bounds.unwrap().x.clone())*3.2) as i32, ((position.y.clone()-screenshot_bounds.unwrap().y.clone())*3.2) as i32);
                                }
                                if screenshot_bounds.unwrap().contains(position) && self.draw_mouse_pressed.clone() && self.crop_start != (0, 0) {
                                    self.crop_end = (((position.x.clone()-screenshot_bounds.unwrap().x.clone())*3.2) as i32, ((position.y.clone()-screenshot_bounds.unwrap().y.clone())*3.2) as i32);
                                }
                            }
                            Some(Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left))) => {
                                self.draw_mouse_pressed = true;
                            }
                            Some(Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left))) => {
                                self.draw_mouse_pressed = false;
                                self.crop = CropMode::CropConfirm;
                                //println!("x1:{} y1:{} x2:{} y3:{}",self.crop_start.0,self.crop_start.1,self.crop_end.0,self.crop_end.1);
                                rect = Rect::at(self.crop_start.0.clone(), self.crop_start.1.clone()).of_size((self.crop_end.0.clone()-self.crop_start.0.clone()) as u32, (self.crop_end.1.clone()-self.crop_start.1.clone()) as u32);
                                self.screen_result = Some(imageproc::drawing::draw_hollow_rect(&screen, rect, color));
                            }
                            _ => {}
                        };
                    }
                    _ => {}
                }
                return Command::none();
            }
            Message::DrawFreeButton => {
                if self.draw == Draw::FreeHand {
                    self.draw = Draw::Nothing;
                } else {
                    self.draw = Draw::FreeHand;
                }
                return Command::none();
            }
            Message::DrawCircleButton => {
                if self.draw == Draw::Circle {
                    self.draw = Draw::Nothing;
                } else {
                    self.draw = Draw::Circle;
                }
                return Command::none();
            }
            Message::DrawTextButton => {
                if self.draw == Draw::Text {
                    self.draw = Draw::Nothing;
                    self.draw_text_input = "".to_string();
                } else {
                    self.draw = Draw::Text;
                }
                return Command::none();
            }
            Message::DrawTextInput(value) => {
                self.draw_text_input = value;
                return Command::none();
            }
            Message::DrawClearButton => {
                self.screen_result = self.screen_result_backup.clone();
                return Command::none();
            }
            Message::DrawArrowButton => {
                if self.draw == Draw::Arrow {
                    self.draw = Draw::Nothing;
                } else {
                    self.draw = Draw::Arrow;
                }
                return Command::none();
            }
            Message::CropButton => {
                if self.draw == Draw::Crop && self.crop == CropMode::Crop {
                    self.draw = Draw::Nothing;
                }
                else if self.draw == Draw::Crop && self.crop == CropMode::CropConfirm {
                    let cropped: SubImage<&RgbaImage> = self.screen_result.as_ref().unwrap().view(self.crop_start.0.clone() as u32, self.crop_start.1.clone() as u32, (self.crop_end.0.clone()-self.crop_start.0.clone()) as u32, (self.crop_end.1.clone()-self.crop_start.1.clone()) as u32);
                    self.screen_result = Some(cropped.to_image());
                    self.crop = CropMode::Crop;
                    self.draw = Draw::Nothing;
                    self.crop_start = (0,0);
                    self.crop_end = (0,0);
                }
                else {
                    self.draw = Draw::Crop;
                }
                return Command::none();
            }
            Message::DrawColorSlider(value) => {
                self.draw_color_slider_value = value;
                return Command::none();
            }
        }
    }

    fn view(&self) -> Element<Message> {
        return container(
            match self.page_state {
                PagesState::Home => home(self.screen_result.clone(), self.toggler_value_autosave.clone()),
                PagesState::Settings => settings(self.toggler_value_autosave, self.toggler_value_clipboard, self.radio_value_monitor, self.radio_value_format, self.timer_value, self.shortcut_value.clone(), self.path_value.clone()),
                PagesState::Modify => modify(self.screen_result.clone(), self.draw.clone(), self.draw_text_input.clone(), self.screen_result_backup.clone(), self.draw_color_slider_value.clone(), self.crop.clone()),
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
                        while image == None {
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