use rodio::{Decoder, OutputStream, Sink};
use std::io::BufReader;
use std::fs::File;
use std::thread;

pub fn play_sound(file_path: &str) {
    // Clone file_path into a String to move it into the new thread
    let file_path = file_path.to_string();

    thread::spawn(move || {
        // Create an output stream and a sink to control the playback
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&stream_handle).unwrap();

        // Load the audio file
        let file = File::open(file_path).unwrap();
        let source = Decoder::new(BufReader::new(file)).unwrap();

        // Play the sound
        sink.append(source);
        sink.sleep_until_end(); // Wait until the sound finishes playing
    });
}
