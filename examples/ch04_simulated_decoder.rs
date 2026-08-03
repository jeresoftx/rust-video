//! Consume un decodificador simulado sin conocer FFmpeg.

use std::time::Duration;

use rust_video::{FrameDecoder, FrameMetadata, SimulatedDecoder, VideoResolution};

fn main() {
    let resolution = VideoResolution::new(1_280, 720).expect("dimensiones válidas");
    let frames = [
        FrameMetadata::new(1, Duration::ZERO, resolution),
        FrameMetadata::new(2, Duration::from_millis(33), resolution),
    ];
    let mut decoder = SimulatedDecoder::new(frames);

    while let Some(frame) = decoder.decode_next().expect("entrada simulada válida") {
        println!("frame={}", frame.sequence());
    }
}
