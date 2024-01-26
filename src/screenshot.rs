use screenshots::{Screen};
use std::error::Error;
use image::RgbaImage;

pub struct Screenshot {
    pub screen: Screen,
    pub width: u32,
    pub height: u32,
}

impl Screenshot {

    pub fn capture_all() -> Result<Vec<Self>, Box<dyn Error>> {
        let screens = Screen::all().unwrap();

        let screenshots: Vec<Self> = screens
            .into_iter()
            .map(|screen| Self {
                screen: screen,
                width: screen.display_info.width,
                height: screen.display_info.height,
            })
            .collect();

        Ok(screenshots)
    }

    pub fn capture_first() -> Result<Self, Box<dyn Error>> {
        let screens = Screen::all().unwrap();
        let primary = screens.get(0).ok_or("Screen not found")?;

        Ok(Self {
            screen: *primary,
            width: primary.display_info.width,
            height: primary.display_info.height,
        })
    }

    pub fn capture_screen(sc: u32) -> Result<Self, Box<dyn Error>> {
        let screens = Screen::all().unwrap();
        let screen = screens.get(sc as usize).ok_or("Screen index out of bounds")?;

        Ok(Self {
            screen: *screen,
            width: screen.display_info.width,
            height: screen.display_info.height,
        })
    }

    pub fn monitors_num() -> usize {
        let screens = Screen::all().unwrap();
        let monum = screens.len();
        return monum;
    }

    pub fn convert(&self) -> Result<RgbaImage,Box<dyn Error>> {
        let img = self.screen.capture().unwrap();

        Ok(img)
    }

}