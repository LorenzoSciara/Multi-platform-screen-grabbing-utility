use iced::{Element, Length, theme, Alignment};
use iced::widget::{button, row, text, container, column, image};
use crate::{Message};
use iced::widget::{horizontal_space, scrollable, toggler, vertical_space, Radio, Container};
use multi_platform_screen_grabbing_utility::screenshot::Screenshot;
use image::RgbaImage;

pub fn home(screen_result: Option<RgbaImage>) -> Element<'static, Message> {
    let controlRow;
    let imageRow;

    match screen_result {
        Some(screen) => {
            controlRow = row![
                        button(text("New Screenshot").width(Length::Fill).size(20)).style(theme::Button::Primary).on_press(Message::NewScreenshotButton),
                        button(text("Settings").width(Length::Fill).size(20)).style(theme::Button::Secondary).on_press(Message::SettingsButton),
                        button(text("Modify").width(Length::Fill).size(20)).style(theme::Button::Secondary).on_press(Message::ModifyButton)
                ]
                .spacing(20)
                .align_items(Alignment::Center)
                .into();
            imageRow = row![
                        //qua invece di image di screen che non va ho usare funzioni per convertire questo screen di tipo RgbaImage in un formato raw che equivalga ad un file .png
                        //oppure passare a home.rs solo un true come screen result per fare un match o un if e prendere il file scrivendo "file.png", in tal caso secondo me ci vuole
                        //un file.png che esiste fin dall'inizio con scritto "screen not found" nell'immagine e quando viene fatto lo screen quel file viene eliminato e poi salvato uno
                        //nuovo con lo stesso nome, comunque è una soluzione di merda, con il file salvato in un nostro archivio temporaneo segreto....
                        image(screen);//image("tour/images/ferris.png")
        ].spacing(20)
                .align_items(Alignment::Center)
                .into();
        }
        None => {
            controlRow = row![
                        button(text("New Screenshot").width(Length::Fill).size(20)).style(theme::Button::Primary).on_press(Message::NewScreenshotButton),
                        button(text("Settings").width(Length::Fill).size(20)).style(theme::Button::Secondary).on_press(Message::SettingsButton),
                ]
                .spacing(20)
                .align_items(Alignment::Center)
                .into();

            //qua la riga rimane vuota, non la migliore delle soluzioni, oppure l'immagine di default con la scritta "screen not found"
            imageRow = row![].spacing(20)
                .align_items(Alignment::Center)
                .into();
        }
    }

        let spacev = vertical_space(Length::Fixed(20.0));


        let content: Element < _ > = column![ controlRow, spacev, imageRow ]
        .spacing(20)
        .padding(20)
        .into();


        return container(content).height(Length::Fill).center_y().center_x().into();

    }
