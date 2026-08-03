//! Solución del ejercicio del capítulo 01.

use std::time::Duration;

use rust_video::{FrameMetadata, VideoResolution};

fn main() {
    let resolution = VideoResolution::new(1_280, 720).expect("dimensiones válidas");
    let frame = FrameMetadata::new(24, Duration::from_millis(800), resolution);

    println!(
        "frame={} tiempo={}ms resolución={}x{}",
        frame.sequence(),
        frame.timestamp().as_millis(),
        frame.resolution().width(),
        frame.resolution().height()
    );
}
