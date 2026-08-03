# AGENTS.md

Este repositorio pertenece a Jeresoft Academy y se rige por el Manual
Fundacional RFC-0001.

## Objetivo

Enseñar cómo diseñar un pipeline de procesamiento de video en streaming con
Rust, sin confundir integración de herramientas maduras con reimplementarlas.

## Antes De Escribir Código

Para cada cambio no trivial, documenta este orden (RFC-0001 §2):

1. Concepto.
2. Problema.
3. Alternativas.
4. Justificación.
5. Implementación.

## Límites Canónicos

- Performance, perfiles y presupuestos de latencia: `rust-performance`.
- Memoria, caché y zero-copy: `rust-low-level`.
- Paralelismo, coordinación y backpressure: `rust-concurrency`.
- Ingesta y transporte de streams: `rust-networking`.
- Modelos, inferencia y evaluación responsable: `rust-ai-engineering`.

`rust-video` integra esos conocimientos en un dominio aplicado; no los vuelve a
explicar desde cero.

## Seguridad, Privacidad Y Dependencias

- Rust idiomático, rustfmt y Clippy sin advertencias; `unsafe` está prohibido.
- No agregar dependencias no triviales sin justificación escrita y autorización
  humana. FFmpeg y runtimes de inferencia requieren decisión por capítulo.
- No acceder a cámara, streams privados ni datos biométricos sin autorización
  humana, fuente legítima y política de privacidad explícita.
- No afirmar precisión de modelos, latencia de producción ni capacidades de
  vigilancia sin evidencia verificable y revisión humana.

## Flujo De GitHub

Antes de código de curso, el plan completo debe vivir en milestones, issues y
un GitHub Project. Cada issue se asigna a `jeresoftx`, tiene milestone y labels.
Cada PR resuelve un issue, conserva la misma metadata, se agrega al mismo
Project y se verifica antes de revisión o fusión.

Flujo obligatorio: `1 issue -> 1 rama -> 1 commit principal -> 1 PR`.
No se fusiona un PR sin revisión humana, salvo autorización explícita de modo
autónomo con revisión diferida conforme a RFC-0001 §20.

## Verificación Base

```bash
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-targets
cargo test --doc
git diff --check
```
