mod screen_capture;
mod image_processing;
mod hotkeys;

use screen_capture::ScreenCapturer;
use image_processing::ImageProcessor;
use std::{io, path::Path, thread, time};
use image::ImageFormat;
use hotkeys::{HotkeyListener, HotkeyConfig};


fn main() -> io::Result<()> {

    //Codice da integrare nella gui

    //implementazione hotkeys

    let hotkey_config = HotkeyConfig::parse_hotkey("Shift", "Q")
        .expect("Configurazione hotkey non valida");

    let hotkey_listener = HotkeyListener::new();
    hotkey_listener.start();

    loop {
        if let Some(pressed_keys) = hotkey_listener.listen() {
            if pressed_keys.contains(&hotkey_config.modifier) && pressed_keys.contains(&hotkey_config.key) {
                println!("Combinazione {:?} + {:?} premuta!", hotkey_config.modifier, hotkey_config.key);
                // Implementa qui la tua logica per la combinazione di hotkey
                let mut capturer = ScreenCapturer::new()?;
                let (frame, width, height) = capturer.capture_screen()?;

                let file_path = "screenshot"; // Cambia l'estensione per .png, .gif, ecc.
                //verifica che il formato sia giusto
                let format = Path::new(file_path)
                    .extension()
                    .and_then(std::ffi::OsStr::to_str)
                    .map(|ext| match ext.to_lowercase().as_str() {
                        "jpg" | "jpeg" => ImageFormat::Jpeg,
                        "gif" => ImageFormat::Gif,
                        "png" => ImageFormat::Png,
                        _ => {
                            eprintln!("Formato file non supportato. Salvataggio in formato PNG.");
                            ImageFormat::Png
                        }
                    })
                    .unwrap_or_else(|| {
                        eprintln!("Nessuna estensione trovata. Salvataggio in formato PNG.");
                        ImageFormat::Png
                    });

                let final_path = match format {
                    ImageFormat::Png => if !file_path.ends_with(".png") { format!("{}.png", file_path) } else { String::from(file_path) },
                    _ => String::from(file_path),
                };

                let processor = ImageProcessor::new();
                processor.process_and_save_image(&frame, width, height, &final_path, format)?;

                return Ok(())
            }
        }

        thread::sleep(time::Duration::from_millis(100));
    }
}