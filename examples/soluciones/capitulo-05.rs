//! Solución del ejercicio del capítulo 05.

use rust_video::{BoundingBox, Detection, DetectionThreshold};

fn main() {
    let area = BoundingBox::new(0, 0, 16, 16).expect("caja válida");
    let detection = Detection::new("objeto", area, 0.65).expect("detección válida");
    let permissive = DetectionThreshold::new(0.60).expect("umbral válido");
    let strict = DetectionThreshold::new(0.70).expect("umbral válido");

    assert!(permissive.accepts(&detection));
    assert!(!strict.accepts(&detection));
}
