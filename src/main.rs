use iced::{Element, Sandbox, Settings, Alignment, Length, alignment, theme};
use iced::Background::Color;
use iced::widget::{button, row, text, column, image, Image};

pub fn main() -> iced::Result {
    Screenshot::run(Settings::default())
}

struct Screenshot{
    name: String,
    image: image::Handle,
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    NewScreenshot,
    Settings,
}

impl Sandbox for Screenshot {
    type Message = Message;
    fn new() -> Screenshot { Screenshot{name: "Empty".to_string(), image: image::Handle::from_memory("resources/empty-image.png")}  }
    fn title(&self) -> String {
        String::from("Multi-platform Screen-grabbing Utility")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::NewScreenshot =>{

            }
            Message::Settings => {

            }
        }
    }

    fn view(&self) -> Element<Message> {
        // We use a column: a simple vertical layout
        return column![
            image::viewer(self.image.clone()),
            row![
                button("New Screenshot").style(theme::Button::Primary).on_press(Message::NewScreenshot),
                button("Settings").style(theme::Button::Secondary).on_press(Message::Settings),
            ]
                .padding(20)
                .spacing(20)
                .align_items(Alignment::Center),
        ]
            .spacing(20)
            .align_items(Alignment::Center)
            .into();
    }
}
