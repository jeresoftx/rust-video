//! Solución del ejercicio del capítulo 06.

use std::time::Duration;

use rust_video::{FrameMetadata, Track, TrackId, TrackStatus, VideoResolution};

fn main() {
    let resolution = VideoResolution::new(640, 480).expect("dimensiones válidas");
    let first = FrameMetadata::new(1, Duration::ZERO, resolution);
    let next = FrameMetadata::new(2, Duration::from_millis(33), resolution);
    let mut track = Track::new(TrackId::new(1), first, 1);

    assert_eq!(track.mark_missing(), TrackStatus::Occluded);
    assert_eq!(track.mark_missing(), TrackStatus::Expired);
    assert!(!track.observe(next));

    let reassigned = Track::new(TrackId::new(2), next, 1);
    assert_eq!(reassigned.id(), TrackId::new(2));
}
