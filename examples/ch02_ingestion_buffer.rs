//! Hace visible la pérdida en una ingesta simulada.

use std::time::Duration;

use rust_video::{FrameBuffer, FrameMetadata, VideoResolution};

fn frame(sequence: u64) -> FrameMetadata {
    let resolution = VideoResolution::new(640, 480).expect("dimensiones válidas");
    FrameMetadata::new(sequence, Duration::from_millis(sequence * 33), resolution)
}

fn main() {
    let mut buffer = FrameBuffer::new(2).expect("capacidad válida");

    for sequence in 1..=3 {
        if let Some(discarded) = buffer.push(frame(sequence)) {
            println!("descartado={}", discarded.sequence());
        }
    }

    while let Some(available) = buffer.pop_front() {
        println!("procesar={}", available.sequence());
    }
}
