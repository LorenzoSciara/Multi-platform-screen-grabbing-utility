use image::{ImageBuffer, Rgba};
use std::io::{self, ErrorKind};

pub struct ImageProcessor;

impl ImageProcessor {
    pub fn new() -> Self {
        ImageProcessor
    }

    pub fn process_and_save_image(
        &self,
        raw_data: &[u8],
        width: u32,
        height: u32,
        file_path: &str,
    ) -> io::Result<()> {
        let image = Self::create_image(raw_data, width, height)?;
        image.save(file_path).map_err(|e| io::Error::new(ErrorKind::Other, e))
    }

    fn create_image(raw_data: &[u8], width: u32, height: u32) -> io::Result<ImageBuffer<Rgba<u8>, Vec<u8>>> {
        let mut img = ImageBuffer::new(width, height);
        for (x, y, pixel) in img.enumerate_pixels_mut() {
            let index = 4 * (y * width + x) as usize;
            *pixel = Rgba([raw_data[index + 2], raw_data[index + 1], raw_data[index], 255]);
        }
        Ok(img)
    }
}