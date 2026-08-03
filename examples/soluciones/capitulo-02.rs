//! Solución del ejercicio del capítulo 02.

use std::time::Duration;

use rust_video::{FrameBuffer, FrameMetadata, VideoResolution};

fn frame(sequence: u64) -> FrameMetadata {
    let resolution = VideoResolution::new(1_280, 720).expect("dimensiones válidas");
    FrameMetadata::new(sequence, Duration::from_millis(sequence * 33), resolution)
}

fn main() {
    let mut buffer = FrameBuffer::new(2).expect("capacidad válida");

    for sequence in 10..=12 {
        if let Some(discarded) = buffer.push(frame(sequence)) {
            println!("descartado={}", discarded.sequence());
        }
    }

    assert_eq!(buffer.pop_front().map(FrameMetadata::sequence), Some(11));
    assert_eq!(buffer.pop_front().map(FrameMetadata::sequence), Some(12));
}
