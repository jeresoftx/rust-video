# Rust Video

Curso aplicado de Jeresoft Academy para estudiar procesamiento de video en
streaming con Rust. Reúne performance, low-level, concurrencia y AI engineering
para explicar un pipeline de detección en tiempo real con decisiones, límites y
tradeoffs explícitos (RFC-0001 §10).

No busca reimplementar códecs, FFmpeg, protocolos ni redes neuronales. El foco
es diseñar, orquestar, medir y operar el pipeline que los integra. La experiencia
del dominio se narra con honestidad: procesamiento de video y detección en
tiempo real es un área de construcción directa del autor (RFC-0001 §1 y §10).

## Estado

El curso está en planificación. Su [Project operativo](https://github.com/users/jeresoftx/projects/29)
convierte el temario vigente en milestones e issues antes de escribir código de
curso. La ampliación propuesta a temas audiovisuales avanzados vive en
`RFC-0004`, aún en borrador; no forma parte del alcance de implementación.

## Temario Base

| # | Capítulo | Estado |
|---|---|---|
| 01 | Fundamentos de video: frames, códecs y contenedores | planned |
| 02 | Captura e ingesta de streams | planned |
| 03 | Pipeline de procesamiento en tiempo real | planned |
| 04 | Decodificación y manejo de FFmpeg desde Rust | planned |
| 05 | Detección de objetos sobre frames | planned |
| 06 | Seguimiento entre frames | planned |
| 07 | Rendimiento y presupuesto de latencia | planned |
| 08 | Procesamiento paralelo y zero-copy | planned |
| 09 | Salida y anotación del stream | planned |

## Prerrequisitos y Lugar En El Camino

Es un dominio aplicado complementario. Reutiliza los contenidos canónicos de:

- `rust-performance` para perfiles, presupuesto de latencia y medición.
- `rust-low-level` para memoria, caché y zero-copy.
- `rust-concurrency` para paralelismo, coordinación y backpressure.
- `rust-ai-engineering` para ciclos de vida de modelos e inferencia responsable.
- `rust-networking` para transporte e ingesta de streams.

## Verificación Base

```bash
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-targets
cargo test --doc
```

## Gobernanza

- [ROADMAP.md](ROADMAP.md) ordena el avance pedagógico sin fechas artificiales.
- [plan/curso-rust-video.md](plan/curso-rust-video.md) es el plan operativo
  vigente y la fuente del Project.
- [AGENTS.md](AGENTS.md) define el flujo para personas y agentes.
- [LICENSE.md](LICENSE.md) documenta la doble licencia de código y contenido.

Cada capítulo explica concepto, problema, alternativas y justificación antes de
la implementación. Ningún contenido se marca como `reviewed` o `published` sin
revisión humana (RFC-0001 §2, §14 y §20).
