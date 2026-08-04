//! Recorre una salida determinista sin video, modelo ni renderer reales.

use std::time::Duration;

use rust_video::{
    Annotation, BoundingBox, Detection, DetectionThreshold, FrameDecoder, FrameMetadata,
    FrameOutput, SimulatedDecoder, VideoResolution,
};

fn main() {
    let resolution = VideoResolution::new(640, 480).expect("dimensiones válidas");
    let frames = [
        FrameMetadata::new(1, Duration::ZERO, resolution),
        FrameMetadata::new(2, Duration::from_millis(33), resolution),
    ];
    let mut decoder = SimulatedDecoder::new(frames);
    let threshold = DetectionThreshold::new(0.8).expect("umbral válido");
    let area = BoundingBox::new(10, 20, 30, 40).expect("caja válida");

    while let Some(frame) = decoder.decode_next().expect("entrada simulada válida") {
        let detection = Detection::new("objeto", area, 0.82).expect("detección válida");
        let mut output = FrameOutput::new(frame);

        if threshold.accepts(&detection) {
            output.add_annotation(Annotation::from_detection(&detection));
        }

        println!(
            "frame={} anotaciones={}",
            output.frame().sequence(),
            output.annotations().len()
        );
    }
}
