mod screen_capture;
mod image_processing;

use screen_capture::ScreenCapturer;
use image_processing::ImageProcessor;
use std::io;
use std::path::Path;
use image::ImageFormat;


fn main() -> io::Result<()> {

    //Codice da integrare nella gui
    let mut capturer = ScreenCapturer::new()?;
    let (frame, width, height) = capturer.capture_screen()?;

    let base_file_path = "screenshot"; // Modifica per includere o escludere estensioni diverse

    let (final_path, format) = match Path::new(base_file_path)
        .extension()
        .and_then(std::ffi::OsStr::to_str)
        .map(|ext| ext.to_lowercase()) {
        Some(ext) if ext == "jpg" || ext == "jpeg" => (format!("{}.jpg", base_file_path), ImageFormat::Jpeg),
        Some(ext) if ext == "gif" => (format!("{}.gif", base_file_path), ImageFormat::Gif),
        _ => {
            eprintln!("Estensione file non riconosciuta o non supportata. Salvataggio in formato PNG.");
            (format!("{}.png", base_file_path), ImageFormat::Png)
        }
    };

    let processor = ImageProcessor::new();
    processor.process_and_save_image(&frame, width, height, &final_path, format)?;

    Ok(())
}