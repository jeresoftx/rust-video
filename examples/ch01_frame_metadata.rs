//! Construye metadatos de un frame sin abrir una fuente de video.

use std::time::Duration;

use rust_video::{FrameMetadata, VideoResolution};

fn main() {
    let resolution = VideoResolution::new(1_920, 1_080).expect("dimensiones válidas");
    let frame = FrameMetadata::new(12, Duration::from_millis(400), resolution);

    println!(
        "frame={} tiempo={:?} resolución={}x{}",
        frame.sequence(),
        frame.timestamp(),
        frame.resolution().width(),
        frame.resolution().height()
    );
}
