use iced::{Element, Sandbox, Settings, Alignment};
use iced::widget::{button, row};

pub fn main() -> iced::Result {
    Screenshot::run(Settings::default())
}

struct Screenshot;

#[derive(Debug, Clone, Copy)]
pub enum Message {
    NewScreenshot,
    Settings,
}

impl Sandbox for Screenshot {
    type Message = Message;
    fn new() -> Screenshot {
        Screenshot
    }
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
        return row![
            // The increment button. We tell it to produce an
            // `IncrementPressed` message when pressed
            button("New Screenshot").on_press(Message::NewScreenshot),

            // The decrement button. We tell it to produce a
            // `DecrementPressed` message when pressed
            button("Settings").on_press(Message::Settings),
        ].padding(100).align_items(Alignment::Center).into();
    }
}