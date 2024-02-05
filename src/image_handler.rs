use arboard::{Clipboard, ImageData};
use image::{ImageFormat, RgbaImage};
use std::path::PathBuf;
use log::error;
use env_logger;

#[derive(Clone)]
pub struct ImageHandler {
    pub buffer : Vec<u8>,
    width : u32,
    height : u32,
}

impl From<RgbaImage> for ImageHandler {
    fn from(value: RgbaImage) -> Self
    {
        Self{
            buffer: value.clone().into_raw(),
            width: value.width(),
            height: value.height(),
        }
    }
}

impl ImageHandler {

    pub fn to_clipboard(&self) -> Result<(), arboard::Error>
    {
        let mut cb = Clipboard::new()?;
        match notifica::notify("Screenshot saved in the clipboard.", "") {
            Ok(_) => {}
            Err(_) => {}
        }
        cb.set_image(ImageData {
            width: self.width as usize,
            height: self.height as usize,
            bytes: (&self.buffer).into(),
        })

    }

    pub fn save_image(image: RgbaImage, path: PathBuf) {

        let format : ImageFormat;
        env_logger::init();

        match path.clone().extension(){
            Some(ext) => {
                match ext.to_str().unwrap() {
                    "png" => format = ImageFormat::Png,
                    "jpg" => format = ImageFormat::Jpeg,
                    "jpeg" => format = ImageFormat::Jpeg,
                    "gif" => format = ImageFormat::Gif,
                    _ => {
                        error!("Format not supported.");
                        return;
                    }
                }
            }
            None => {
                error!("Format not supported.");
                return;
            }
        }

        image.save_with_format(path,format);
    }

}