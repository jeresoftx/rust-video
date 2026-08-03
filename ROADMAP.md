# Roadmap

Ruta viva de `rust-video`, el curso aplicado de procesamiento de video en
streaming de Jeresoft Academy.

No hay fechas límite: la calidad y la documentación del porqué tienen prioridad
sobre la velocidad (RFC-0001 §1 y §2). El roadmap ordena dependencias
pedagógicas, no promete publicación.

## Orden De Construcción

1. Fundamentos, ingesta y contrato del pipeline.
2. Decodificación, frames, detección y tracking.
3. Latencia, paralelismo, zero-copy, salida y anotación.
4. Auditoría de draft y revisión humana obligatoria.

El [Project operativo](https://github.com/users/jeresoftx/projects/29) conserva
issues, milestones y ruta crítica. El plan detallado vive en
[plan/curso-rust-video.md](plan/curso-rust-video.md).

## Límites De Autonomía

La planificación puede avanzar con modelos, diagramas y ejemplos sintéticos.
Requieren una decisión humana previa las dependencias no triviales de FFmpeg,
aceleración por hardware, runtimes de inferencia, acceso a cámara o red, datos
de video no sintéticos y cualquier afirmación de rendimiento o privacidad de
producción.
