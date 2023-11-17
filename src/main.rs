use iced::executor;
use iced::{Application, Command, Element, Settings, Theme};

pub fn main() -> iced::Result {
    Screenshot::run(Settings::default())
}

struct Screenshot;

#[derive(Debug, Clone, Copy)]
pub enum Message {
    NewScreenshot,

}

impl Application for Screenshot {
    type Executor = executor::Default;
    type Flags = ();
    type Message = ();
    type Theme = Theme;

    fn new(_flags: ()) -> (Screenshot, Command<Self::Message>) {
        (Screenshot, Command::none())
    }

    fn title(&self) -> String {
        String::from("Multi-platform Screen-grabbing Utility")
    }

    fn update(&mut self, _message: Self::Message) -> Command<Self::Message> {
        Command::none()
    }

    fn view(&self) -> Element<Self::Message> {
        "Hello, world!".into()
    }
}