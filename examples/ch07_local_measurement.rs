//! Mide solo el modelo local; no representa una fuente o un decodificador real.

use std::time::{Duration, Instant};

use rust_video::LatencyBudget;

fn main() {
    let iterations = 10_000;
    let started_at = Instant::now();
    let mut budgets_outside_limit = 0;

    for _ in 0..iterations {
        let mut budget = LatencyBudget::new(Duration::from_millis(45)).expect("límite válido");
        budget.add_stage("ingesta", Duration::from_millis(5));
        budget.add_stage("análisis", Duration::from_millis(20));
        budget.add_stage("salida", Duration::from_millis(8));

        if !budget.is_within_limit() {
            budgets_outside_limit += 1;
        }
    }

    println!("iteraciones: {iterations}");
    println!("duración local: {:?}", started_at.elapsed());
    println!("presupuestos fuera del límite: {budgets_outside_limit}");
    println!("esta cifra no representa cámara, FFmpeg, GPU, red ni producción");
}
