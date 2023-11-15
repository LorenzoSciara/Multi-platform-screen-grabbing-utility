mod screen_capture;
mod image_processing;

use screen_capture::ScreenCapturer;
use image_processing::ImageProcessor;
use std::io;

fn main() -> io::Result<()> {
    let mut capturer = ScreenCapturer::new()?;
    let (frame, width, height) = capturer.capture_screen()?;

    let processor = ImageProcessor::new();
    processor.process_and_save_image(&frame, width, height, "screenshot.png")?;

    Ok(())
}