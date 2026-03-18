use crate::error::{PerleError, Result};
use rodio::{Decoder, DeviceSinkBuilder};
use std::io::Cursor;

pub fn play_notification() -> Result<()> {
    let sound_data = include_bytes!("../assets/beep.mp3");

    let sink =
        DeviceSinkBuilder::open_default_sink().map_err(|e| PerleError::Audio(e.to_string()))?;

    let mixer = sink.mixer();
    let cursor = Cursor::new(sound_data);

    let source = Decoder::new(cursor).map_err(|e| PerleError::Audio(e.to_string()))?;

    mixer.add(source);
    std::thread::sleep(std::time::Duration::from_secs(2));
    Ok(())
}
