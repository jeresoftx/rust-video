//! Solución del ejercicio del capítulo 08.

use rust_video::{ProcessingPlan, SharedFramePayload};

fn main() {
    let plan = ProcessingPlan::new(2, 3).expect("plan válido");
    let payload = SharedFramePayload::from_bytes(vec![7, 8, 9]);
    let first_stage = payload.clone();
    let second_stage = payload.clone();

    assert!(plan.can_accept(2));
    assert!(!plan.can_accept(3));
    assert!(payload.shares_storage_with(&first_stage));
    assert!(payload.shares_storage_with(&second_stage));

    println!("la cola decide presión; Arc comparte lectura sin unsafe");
}
