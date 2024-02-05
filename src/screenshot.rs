use screenshots::{Screen};
use std::error::Error;
use image::RgbaImage;

pub struct Screenshot {
    pub screen: Screen,
}

impl Screenshot {

    pub fn capture_all() -> Result<Vec<Self>, Box<dyn Error>> { //ritorna o un vettore di screen o un errore (in Box)

        let screens = Screen::all().unwrap();

        let screenshots: Vec<Self> = screens
            .into_iter()
            .map(|screen| Self {
                screen: screen,
            })
            .collect();

        Ok(screenshots)
    }


    pub fn capture_first() -> Result<Self, Box<dyn Error>> { //Ritorna solo lo screen dello schermo principale

        let screens = Screen::all().unwrap();
        let primary = screens.get(0).ok_or("Screen not found")?;

        Ok(Self {
            screen: *primary,
        })
    }


    pub fn capture_screen(sc: u32) -> Result<Self, Box<dyn Error>> { //Ritorna lo screen dello schermo selezionato come indice (sc)

        let screens = Screen::all().unwrap();
        let screen = screens.get(sc as usize).ok_or("Screen index out of bounds")?;

        Ok(Self {
            screen: *screen,
        })
    }


    pub fn monitors_num() -> usize { //Ritorna il numero di monitor presenti nel sistema

        let screens = Screen::all().unwrap();
        let monum = screens.len();
        return monum;
    }

    pub fn convert(&self) -> Result<RgbaImage,Box<dyn Error>> { //Trasforma lo screeen da tipo screen a RgbImag

        let img = self.screen.capture().unwrap();

        Ok(img)
    }

}