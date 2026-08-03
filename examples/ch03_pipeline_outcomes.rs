//! Registra una falla local sin detener un pipeline secuencial.

use std::time::Duration;

use rust_video::{
    FrameMetadata, FrameStage, Pipeline, StageFailure, StageOutcome, VideoResolution,
};

struct ValidarSecuencia;

impl FrameStage for ValidarSecuencia {
    fn process(&mut self, frame: FrameMetadata) -> StageOutcome {
        if frame.sequence() == 2 {
            StageOutcome::Failed(StageFailure::new("frame rechazado por la etapa"))
        } else {
            StageOutcome::Forward(frame)
        }
    }
}

fn main() {
    let resolution = VideoResolution::new(640, 480).expect("dimensiones válidas");
    let frames = (1..=3).map(|sequence| {
        FrameMetadata::new(sequence, Duration::from_millis(sequence * 33), resolution)
    });

    let report = Pipeline::new(ValidarSecuencia).run(frames);

    println!("avanzaron={:?}", report.forwarded_sequences());
    println!("fallas={}", report.failures().len());
}
