//! Usa una cola acotada para hacer visible el backpressure.

use std::{sync::mpsc::sync_channel, thread};

use rust_video::{ProcessingPlan, SharedFramePayload};

fn main() {
    let plan = ProcessingPlan::new(1, 2).expect("plan válido");
    let (sender, receiver) = sync_channel::<SharedFramePayload>(plan.maximum_in_flight());
    let worker = thread::spawn(move || {
        let mut processed_bytes = 0;
        while let Ok(payload) = receiver.recv() {
            processed_bytes += payload.len();
        }
        processed_bytes
    });

    for bytes in [[1, 2], [3, 4], [5, 6]] {
        sender
            .send(SharedFramePayload::from_bytes(bytes.to_vec()))
            .expect("worker disponible");
    }
    drop(sender);

    println!("workers declarados: {}", plan.worker_count());
    println!(
        "bytes procesados: {}",
        worker.join().expect("worker sin pánico")
    );
}
