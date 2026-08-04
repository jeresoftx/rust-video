//! Comparte un payload entre etapas de solo lectura.

use rust_video::SharedFramePayload;

fn main() {
    let input = SharedFramePayload::from_bytes(vec![10, 20, 30, 40]);
    let analysis_stage = input.clone();
    let output_stage = input.clone();

    println!("bytes: {}", input.len());
    println!(
        "análisis comparte almacenamiento: {}",
        input.shares_storage_with(&analysis_stage)
    );
    println!(
        "salida comparte almacenamiento: {}",
        input.shares_storage_with(&output_stage)
    );
}
