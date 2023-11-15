use scrap::{Capturer, Display};
use std::io::{self, ErrorKind};
use std::thread;
use std::time::Duration;

pub struct ScreenCapturer {
    capturer: Capturer,
    width: u32,
    height: u32,
}

impl ScreenCapturer {
    pub fn new() -> io::Result<Self> {
        let display = Display::primary()?;
        let mut capturer = Capturer::new(display)?;
        let (width, height) = (capturer.width() as u32, capturer.height() as u32);

        Ok(ScreenCapturer { capturer, width, height })
    }

    pub fn capture_screen(&mut self) -> io::Result<(Vec<u8>, u32, u32)> {
        loop {
            match self.capturer.frame() {
                Ok(frame) => return Ok((frame.to_vec(), self.width, self.height)),
                Err(error) => {
                    if error.kind() == ErrorKind::WouldBlock {
                        thread::sleep(Duration::from_millis(10));
                        continue;
                    } else {
                        return Err(error);
                    }
                }
            }
        }
    }
}