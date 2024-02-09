use rdev::{listen, EventType, Key};
use std::collections::HashSet;
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread;

pub struct HotkeyListener {
    sender: Sender<HashSet<Key>>,
    receiver: Receiver<HashSet<Key>>,
    pressed_keys: Arc<Mutex<HashSet<Key>>>,
}

impl HotkeyListener {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::channel();
        let pressed_keys = Arc::new(Mutex::new(HashSet::new()));
        HotkeyListener {
            sender: tx,
            receiver: rx,
            pressed_keys,
        }
    }

    pub fn start(&self) {
        let sender = self.sender.clone();
        let pressed_keys = self.pressed_keys.clone();
        thread::spawn(move || {
            if let Err(error) = listen(move |event| {
                match event.event_type {
                    EventType::KeyPress(key) => {
                        let mut keys = pressed_keys.lock().unwrap();
                        keys.insert(key);
                        sender.send(keys.clone()).unwrap();
                    }
                    EventType::KeyRelease(key) => {
                        let mut keys = pressed_keys.lock().unwrap();
                        keys.remove(&key);
                        sender.send(keys.clone()).unwrap();
                    }
                    _ => {}
                }
            }) {
                eprintln!("Errore durante l'ascolto: {:?}", error);
            }
        });
    }

    pub fn listen(&self) -> Option<HashSet<Key>> {
        self.receiver.try_recv().ok()
    }
}

pub struct HotkeyConfig {
    pub modifier: Key,
    pub key: Key,
}

impl HotkeyConfig {
    pub fn parse_hotkey(modifier_arg: &str, key_arg: &str) -> Result<HotkeyConfig, &'static str> {
        let modifier = match modifier_arg.to_lowercase().as_str() {
            "control" => Key::ControlLeft,
            "shift" => Key::ShiftLeft,
            "alt" => Key::Alt,
            _ => return Err("Il primo argomento deve essere 'Control' o 'Shift'"),
        };
        let key = match key_arg.to_uppercase().as_str() {
            "A" => Key::KeyA, "B" => Key::KeyB, "C" => Key::KeyC, "D" => Key::KeyD, "E" => Key::KeyE,
            "F" => Key::KeyF, "G" => Key::KeyG, "H" => Key::KeyH, "I" => Key::KeyI, "J" => Key::KeyJ,
            "K" => Key::KeyK, "L" => Key::KeyL, "M" => Key::KeyM, "N" => Key::KeyN, "O" => Key::KeyO,
            "P" => Key::KeyP, "Q" => Key::KeyQ, "R" => Key::KeyR, "S" => Key::KeyS, "T" => Key::KeyT,
            "U" => Key::KeyU, "V" => Key::KeyV, "W" => Key::KeyW, "X" => Key::KeyX, "Y" => Key::KeyY,
            "Z" => Key::KeyZ,
            _ => return Err("Il secondo argomento deve essere una lettera"),
        };
        Ok(HotkeyConfig { modifier, key })
    }
}



