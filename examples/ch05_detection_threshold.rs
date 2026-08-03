//! Aplica un umbral explícito a detecciones sintéticas.

use rust_video::{BoundingBox, Detection, DetectionThreshold};

fn main() {
    let area = BoundingBox::new(12, 18, 40, 30).expect("caja válida");
    let detection = Detection::new("objeto", area, 0.72).expect("detección válida");
    let threshold = DetectionThreshold::new(0.8).expect("umbral válido");

    println!("aceptada={}", threshold.accepts(&detection));
}
