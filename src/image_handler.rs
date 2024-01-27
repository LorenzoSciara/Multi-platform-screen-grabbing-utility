use arboard::{Clipboard, ImageData};
use image::RgbaImage;

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
        match notifica::notify("Screenshot salvato nella clipboard.", "") {
            Ok(_) => {}
            Err(_) => {}
        }
        cb.set_image(ImageData {
            width: self.width as usize,
            height: self.height as usize,
            bytes: (&self.buffer).into(),
        })

    }

}