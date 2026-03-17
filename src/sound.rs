use rodio::{Decoder, DeviceSinkBuilder};
use std::io::Cursor;

pub fn play_notification() {
    let sound_data = include_bytes!("../assets/beep.mp3");
    let sink = DeviceSinkBuilder::open_default_sink().expect("open default audio sink");
    let mixer = sink.mixer();
    let cursor = Cursor::new(sound_data);
    let source = Decoder::new(cursor).unwrap();
    mixer.add(source);
    std::thread::sleep(std::time::Duration::from_secs(2));
}
