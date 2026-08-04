//! Recorre una oclusión y expiración deterministas.

use std::time::Duration;

use rust_video::{FrameMetadata, Track, TrackId, VideoResolution};

fn main() {
    let resolution = VideoResolution::new(640, 480).expect("dimensiones válidas");
    let first = FrameMetadata::new(1, Duration::ZERO, resolution);
    let mut track = Track::new(TrackId::new(1), first, 1);

    println!("estado={:?}", track.mark_missing());
    println!("estado={:?}", track.mark_missing());
}
