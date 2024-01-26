use std::{io, thread, time};
use multi_platform_screen_grabbing_utility::hotkeys::{HotkeyListener,HotkeyConfig};
use multi_platform_screen_grabbing_utility::screenshot::Screenshot;


fn main() -> io::Result<()> {

    let hotkey_config = HotkeyConfig::parse_hotkey("Shift", "Q")
        .expect("Configurazione hotkey non valida");

    let hotkey_listener = HotkeyListener::new();
    hotkey_listener.start();

    loop {
        if let Some(pressed_keys) = hotkey_listener.listen() {
            if pressed_keys.contains(&hotkey_config.modifier) && pressed_keys.contains(&hotkey_config.key) {
                println!("Combinazione {:?} + {:?} premuta!", hotkey_config.modifier, hotkey_config.key);

               match Screenshot::capture_all() {
                   Ok(res) => {
                       for (i,s) in res.iter().enumerate() {
                           s.image.save(format!("monitorasd{}.png", i));
                       }
                   }
                   Err(err) => {
                       eprintln!("Error: {}", err);
                   }
               }

            }
        }

        thread::sleep(time::Duration::from_millis(100));
    }
}