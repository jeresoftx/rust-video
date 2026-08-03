//! Solución del ejercicio del capítulo 03.

use std::time::Duration;

use rust_video::{
    FrameMetadata, FrameStage, Pipeline, StageFailure, StageOutcome, VideoResolution,
};

struct DetenerEn {
    sequence: u64,
    cancel: bool,
}

impl FrameStage for DetenerEn {
    fn process(&mut self, frame: FrameMetadata) -> StageOutcome {
        if frame.sequence() != self.sequence {
            return StageOutcome::Forward(frame);
        }

        if self.cancel {
            StageOutcome::Cancelled
        } else {
            StageOutcome::Failed(StageFailure::new("falla recuperable"))
        }
    }
}

fn frames() -> impl Iterator<Item = FrameMetadata> {
    let resolution = VideoResolution::new(640, 480).expect("dimensiones válidas");
    (1..=3).map(move |sequence| {
        FrameMetadata::new(sequence, Duration::from_millis(sequence * 33), resolution)
    })
}

fn main() {
    let recovered = Pipeline::new(DetenerEn {
        sequence: 2,
        cancel: false,
    })
    .run(frames());
    assert_eq!(recovered.forwarded_sequences(), &[1, 3]);

    let cancelled = Pipeline::new(DetenerEn {
        sequence: 2,
        cancel: true,
    })
    .run(frames());
    assert_eq!(cancelled.forwarded_sequences(), &[1]);
    assert!(cancelled.was_cancelled());
}
