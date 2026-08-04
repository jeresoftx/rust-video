//! Declara un presupuesto de latencia por etapa antes de medir nada externo.

use std::time::Duration;

use rust_video::LatencyBudget;

fn main() {
    let mut budget = LatencyBudget::new(Duration::from_millis(45)).expect("límite válido");
    budget.add_stage("ingesta", Duration::from_millis(5));
    budget.add_stage("decodificación", Duration::from_millis(12));
    budget.add_stage("análisis", Duration::from_millis(20));
    budget.add_stage("salida", Duration::from_millis(8));

    println!("latencia declarada: {:?}", budget.total());
    println!("dentro del límite: {}", budget.is_within_limit());
}
