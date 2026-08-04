//! Solución del ejercicio del capítulo 07.

use std::time::Duration;

use rust_video::LatencyBudget;

fn main() {
    let mut budget = LatencyBudget::new(Duration::from_millis(45)).expect("límite válido");
    budget.add_stage("ingesta", Duration::from_millis(5));
    budget.add_stage("decodificación", Duration::from_millis(12));
    budget.add_stage("análisis", Duration::from_millis(20));
    budget.add_stage("salida", Duration::from_millis(8));

    assert_eq!(budget.total(), Duration::from_millis(45));
    assert!(budget.is_within_limit());

    budget.add_stage("espera adicional", Duration::from_millis(1));
    assert!(!budget.is_within_limit());

    println!("un presupuesto excedido exige una decisión de producto explícita");
}
