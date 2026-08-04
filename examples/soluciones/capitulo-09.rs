//! Solución del ejercicio del capítulo 09.

use std::time::Duration;

use rust_video::{
    Annotation, BoundingBox, Detection, DetectionThreshold, FrameMetadata, FrameOutput,
    VideoResolution,
};

fn main() {
    let resolution = VideoResolution::new(640, 480).expect("dimensiones válidas");
    let frame = FrameMetadata::new(9, Duration::from_millis(297), resolution);
    let area = BoundingBox::new(10, 20, 30, 40).expect("caja válida");
    let accepted = Detection::new("objeto", area, 0.9).expect("detección válida");
    let rejected = Detection::new("ruido", area, 0.3).expect("detección válida");
    let threshold = DetectionThreshold::new(0.8).expect("umbral válido");
    let mut output = FrameOutput::new(frame);

    if threshold.accepts(&accepted) {
        output.add_annotation(Annotation::from_detection(&accepted));
    }
    if threshold.accepts(&rejected) {
        output.add_annotation(Annotation::from_detection(&rejected));
    }

    assert_eq!(output.frame().sequence(), 9);
    assert_eq!(output.annotations().len(), 1);
    assert_eq!(output.annotations()[0].label(), "objeto");

    println!("salida lateral lista para un consumidor independiente");
}
